use candid::{CandidType, Principal};
use ic_cdk::{caller, trap};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize, CandidType)]
struct IDCounters {
    exam_id: u64,
    certificate_id: u64,
}

// Your improved User struct
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
struct User {
    principal: Principal,
    role: Role,
    exams_taken: Vec<u64>,
    certificates: Vec<u64>,
    created_at: u64,
    last_login: u64,
}

// --- AUTHENTICATION FUNCTIONS ---

/// Registers a new user or returns existing user info
/// This is usually called automatically after frontend login
#[ic_cdk::update]
fn register_or_login() -> Result<User, String> {
    let caller_principal = caller();
    let now = ic_cdk::api::time() as u64;
    
    USER_STORE.with(|store| {
        let mut store_mut = store.borrow_mut();
        
        if let Some(user) = store_mut.get_mut(&caller_principal) {
            // Existing user - update last login
            user.last_login = now;
            return Ok(user.clone());
        }
        
        // New user - create and register
        let new_user = User {
            principal: caller_principal,
            role: Role::Student,
            exams_taken: Vec::new(),
            certificates: Vec::new(),
            created_at: now,
            last_login: now,
        };
        
        store_mut.insert(caller_principal, new_user.clone());
        Ok(new_user)
    })
}

/// Gets current user info (requires registration first)
#[ic_cdk::query]
fn get_my_profile() -> Result<User, String> {
    let caller_principal = caller();
    
    USER_STORE.with(|store| {
        store
            .borrow()
            .get(&caller_principal)
            .cloned()
            .ok_or_else(|| "User not registered. Call register_or_login first.".to_string())
    })
}

/// Optional: Check if a principal is registered
#[ic_cdk::query]
fn is_user_registered(principal: Principal) -> bool {
    USER_STORE.with(|store| store.borrow().contains_key(&principal))
}

/// Optional: Get all users (Admin only)
#[ic_cdk::query]
fn get_all_users() -> Result<Vec<User>, String> {
    is_caller_admin()?;
    
    USER_STORE.with(|store| {
        Ok(store.borrow().values().cloned().collect())
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
struct Exam {
    id: u64,
    title: String,
    questions: Vec<Question>,
    level: Level,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
struct Certificate {
    id: u64,
    exam_id: u64,
    user_principal: Principal,
    score: u8,
    awarded_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
struct Question {
    text: String,
    choices: Vec<String>,
    answer_index: u8,
    score: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
enum Level {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, CandidType)]
enum Role {
    Student,
    Teacher,
    Admin,
}

// Create your canister's storage ("The Database")
thread_local! {
    static USER_STORE: RefCell<BTreeMap<Principal, User>> = RefCell::default();
    static EXAM_STORE: RefCell<BTreeMap<u64, Exam>> = RefCell::default();
    static CERTIFICATE_STORE: RefCell<BTreeMap<u64, Certificate>> = RefCell::default();
    static ID_COUNTERS: RefCell<IDCounters> = RefCell::default();
}

// Register user to the canister
#[ic_cdk::update]
fn register_user() {
    let caller_principal: Principal = caller();

    // Check if the user already exists.
    let user_exists = USER_STORE.with(|store| store.borrow().contains_key(&caller_principal));

    if user_exists {
        trap("User already registered.");
    }

    let new_user = User {
        principal: caller_principal,
        role: Role::Student, // Default role is Student
        exams_taken: Vec::new(),
        certificates: Vec::new(),
    };

    USER_STORE.with(|store| {
        store.borrow_mut().insert(caller_principal, new_user);
    });
}

// Promote a user to admin (only callable by existing admin)
#[ic_cdk::update]
fn promote_to_admin(target_principal: Principal) -> Result<(), String> {
    is_caller_admin()?;
    
    USER_STORE.with(|store| {
        let mut store_mut = store.borrow_mut();
        let user = store_mut.get_mut(&target_principal).ok_or("User not found.")?;
        user.role = Role::Admin;
        Ok(())
    })
}

// Gets user from the canister
#[ic_cdk::query]
fn get_user(user_principal: Principal) -> Result<User, String> {
    USER_STORE.with(|store| {
        store
            .borrow()
            .get(&user_principal)
            .cloned()
            .ok_or_else(|| "User not found.".to_string())
    })
}

// List the existing exams
#[ic_cdk::query]
fn list_exams() -> Vec<Exam> {
    EXAM_STORE.with(|store| store.borrow().values().cloned().collect())
}

// Get a specific exam by its ID
#[ic_cdk::query]
fn get_exam(exam_id: u64) -> Result<Exam, String> {
    EXAM_STORE.with(|store| {
        store
            .borrow()
            .get(&exam_id)
            .cloned()
            .ok_or_else(|| "Exam not found.".to_string())
    })
}

// Submit answers and grade exam
#[ic_cdk::update]
fn submit_answers(exam_id: u64, answers: Vec<String>) -> Result<u8, String> {
    let caller_principal = caller();
    
    // Get the exam
    let exam = EXAM_STORE.with(|store| {
        store.borrow().get(&exam_id).cloned().ok_or("Exam not found.")
    })?;

    // Grade the exam
    let mut score = 0;
    for (i, user_answer) in answers.iter().enumerate() {
        if i < exam.questions.len() {
            let question = &exam.questions[i];
            let correct_answer = question.choices.get(question.answer_index as usize);
            if let Some(correct) = correct_answer {
                if user_answer == correct {
                    score += question.score;
                }
            }
        }
    }

    // Update user's exams taken
    USER_STORE.with(|store| {
        let mut store_mut = store.borrow_mut();
        if let Some(user) = store_mut.get_mut(&caller_principal) {
            user.exams_taken.push(exam_id);
        }
    });

    Ok(score)
}

// Get exam result for a user
#[ic_cdk::query]
fn get_result(exam_id: u64, user_principal: Principal) -> Result<String, String> {
    // For simplicity, we'll just return a string result
    // In a real implementation, you might want to store results properly
    Ok("Passed with 85%".to_string())
}

// Claim certificate for an exam
#[ic_cdk::update]
fn claim_certificate(exam_id: u64) -> Result<u64, String> {
    let caller_principal = caller();
    
    // Check if user passed the exam (simplified - always pass for demo)
    let passed = true;
    
    if !passed {
        return Err("You did not pass the exam.".to_string());
    }

    // Generate new certificate ID
    let cert_id = ID_COUNTERS.with(|counters| {
        let mut counters_mut = counters.borrow_mut();
        let id = counters_mut.certificate_id;
        counters_mut.certificate_id += 1;
        id
    });

    let certificate = Certificate {
        id: cert_id,
        exam_id,
        user_principal: caller_principal,
        score: 85, // Example score
        awarded_at: ic_cdk::api::time() as u64,
    };

    // Store certificate
    CERTIFICATE_STORE.with(|store| {
        store.borrow_mut().insert(cert_id, certificate);
    });

    // Update user's certificates
    USER_STORE.with(|store| {
        let mut store_mut = store.borrow_mut();
        if let Some(user) = store_mut.get_mut(&caller_principal) {
            user.certificates.push(cert_id);
        }
    });

    Ok(cert_id)
}

// Get all certificates for a user
#[ic_cdk::query]
fn get_certificates(user_principal: Principal) -> Vec<Certificate> {
    USER_STORE.with(|user_store| {
        if let Some(user) = user_store.borrow().get(&user_principal) {
            CERTIFICATE_STORE.with(|cert_store| {
                user.certificates.iter()
                    .filter_map(|&id| cert_store.borrow().get(&id).cloned())
                    .collect()
            })
        } else {
            Vec::new()
        }
    })
}

// Create a new exam (Admin only)
#[ic_cdk::update]
fn create_exam(title: String, questions: Vec<Question>, level: Level) -> Result<u64, String> {
    is_caller_admin()?;

    let new_id = ID_COUNTERS.with(|counters| {
        let mut counters_mut = counters.borrow_mut();
        let id = counters_mut.exam_id;
        counters_mut.exam_id += 1;
        id
    });

    let new_exam = Exam {
        id: new_id,
        title,
        questions,
        level
    };

    EXAM_STORE.with(|store| {
        store.borrow_mut().insert(new_id, new_exam);
    });

    Ok(new_id)
}

// Delete an exam (Admin only)
#[ic_cdk::update]
fn delete_exam(exam_id: u64) -> Result<(), String> {
    is_caller_admin()?;

    EXAM_STORE.with(|store| {
        if store.borrow_mut().remove(&exam_id).is_none() {
            return Err("Exam not found.".to_string());
        }
        Ok(())
    })
}

// Helper function to check if caller is admin
fn is_caller_admin() -> Result<(), String> {
    let caller_principal = caller();
    
    USER_STORE.with(|store| {
        let user_store = store.borrow();
        let user = user_store.get(&caller_principal).ok_or("User not found.")?;
        
        if user.role != Role::Admin {
            return Err("Caller is not an admin.".to_string());
        }
        
        Ok(())
    })
}

// Create test data helper function
#[ic_cdk::update]
fn create_test_data() {
    let caller_principal = caller();
    
    // Create test exam
    let questions = vec![
        Question {
            text: "What is the capital of France?".to_string(),
            choices: vec!["Paris".to_string(), "London".to_string(), "Berlin".to_string()],
            answer_index: 0,
            score: 10,
        },
        Question {
            text: "2 + 2 = ?".to_string(),
            choices: vec!["3".to_string(), "4".to_string(), "5".to_string()],
            answer_index: 1,
            score: 10,
        }
    ];

    let _ = create_exam("General Knowledge Test".to_string(), questions, Level::Beginner); 
    
    // Make caller an admin for testing
    USER_STORE.with(|store| {
        if let Some(user) = store.borrow_mut().get_mut(&caller_principal) {
            user.role = Role::Admin;
        }
    });
}

ic_cdk::export_candid!();
