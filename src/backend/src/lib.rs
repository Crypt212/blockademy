use candid::{CandidType, Principal};
use ic_cdk::{caller};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize, CandidType)]
struct IDCounters {
    exam_id: u64,
    certificate_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
struct User {
    principal: Principal,
    username: String,
    role: Role,
    certificates: Vec<u64>,
    created_at: u64,
    last_login: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
struct Exam {
    id: u64,
    title: String,
    organization_name: String,
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
    correct_answer_index: u8,
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
fn register_user(username: String) -> Result<User, String> {
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
            username,
            role: Role::Student,
            certificates: Vec::new(),
            created_at: now,
            last_login: now,
        };
        
        store_mut.insert(caller_principal, new_user.clone());
        Ok(new_user)
    })
}

// Returns user profile data
#[ic_cdk::query]
fn get_user_data() -> Result<User, String> {
    is_user_registered()?;    
    let caller_principal = caller();

    USER_STORE.with(|store| {
        store
            .borrow()
            .get(&caller_principal)
            .cloned()
            .ok_or_else(|| "User not registered. Call register_or_login first.".to_string())
    })
}

/// Optional: Get all users (Admin only)
#[ic_cdk::query]
fn list_users() -> Result<Vec<User>, String> {
    is_user_admin()?;
    
    USER_STORE.with(|store| {
        Ok(store.borrow().values().cloned().collect())
    })
}

// Promote a user to admin (only callable by existing admin)
#[ic_cdk::update]
fn promote_to_admin(target_principal: Principal) -> Result<(), String> {
    is_user_admin()?;
    
    USER_STORE.with(|store| {
        let mut store_mut = store.borrow_mut();
        let user = store_mut.get_mut(&target_principal).ok_or("User not found.")?;
        user.role = Role::Admin;
        Ok(())
    })
}

// List the existing exams taken by the user
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
    is_user_registered()?;    
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
            let correct_answer = question.choices.get(question.correct_answer_index as usize);
            if let Some(correct) = correct_answer {
                if user_answer == correct {
                    score += question.score;
                }
            }
        }
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
        score,
        user_principal: caller_principal,
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

    Ok(score)
}

// Create a new exam (Admin only)
#[ic_cdk::update]
fn create_exam(title: String, organization_name: String, questions: Vec<Question>, level: Level) -> Result<u64, String> {
    is_user_admin()?;

    let new_id = ID_COUNTERS.with(|counters| {
        let mut counters_mut = counters.borrow_mut();
        let id = counters_mut.exam_id;
        counters_mut.exam_id += 1;
        id
    });

    let new_exam = Exam {
        id: new_id,
        title,
        organization_name,
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
    is_user_admin()?;

    EXAM_STORE.with(|store| {
        if store.borrow_mut().remove(&exam_id).is_none() {
            return Err("Exam not found.".to_string());
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
            correct_answer_index: 0,
            score: 10,
        },
        Question {
            text: "2 + 2 = ?".to_string(),
            choices: vec!["3".to_string(), "4".to_string(), "5".to_string()],
            correct_answer_index: 1,
            score: 10,
        }
    ];

    let _ = create_exam("General Knowledge Test".to_string(), "Testers".to_string(), questions, Level::Beginner); 
    
    // Make caller an admin for testing
    USER_STORE.with(|store| {
        if let Some(user) = store.borrow_mut().get_mut(&caller_principal) {
            user.role = Role::Admin;
        }
    });
}


// Check if a principal is registered
#[ic_cdk::query]
fn is_user_registered() -> Result<(), String> {
    let principal = caller();
    if !USER_STORE.with(|store| store.borrow().contains_key(&principal)) {
    }
    Ok(())
}


// Helper function to check if caller is admin
fn is_user_admin() -> Result<(), String> {
    is_user_registered()?;
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

ic_cdk::export_candid!();
