// src/lib.rs
use ic_cdk::{query, update};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Serialize, Deserialize, CandidType, Eq, PartialEq, Hash)]
struct Question {
    id: u64,
    text: String,
    options: Vec<String>,
    correct_index: u8,
    genres: Vec<String>,
    difficulty: u8
}

thread_local! {
    static QUESTIONS: RefCell<HashMap<u64, Question>> = RefCell::new(HashMap::new());
    static GENRE_INDEX: RefCell<HashMap<String, HashSet<u64>>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(0);
}

#[update]
fn store_question(
    text: String,
    options: Vec<String>,
    correct_index: u8,
    genres: Vec<String>,
    difficulty: u8,
) -> u64 {
    let id = NEXT_ID.with(|n| {
        let id = *n.borrow();
        *n.borrow_mut() = id + 1;
        id
    });

    let question = Question {
        id,
        text,
        options,
        correct_index,
        genres: genres.clone(),
        difficulty,
    };

    // Store in main question map
    QUESTIONS.with(|q| q.borrow_mut().insert(id, question));

    // Update genre index
    GENRE_INDEX.with(|g| {
        let mut g = g.borrow_mut();
        for genre in genres {
            g.entry(genre).or_insert_with(HashSet::new).insert(id);
        }
    });

    id
}

// 4. Get Single Question
#[query]
fn get_question(id: u64) -> Option<Question> {
    QUESTIONS.with(|q| q.borrow().get(&id).cloned())
}

#[query]
fn get_genre_questions(genre: String, limit: usize) -> Vec<Question> {
    GENRE_INDEX.with(|g| {
        g.borrow()
         .get(&genre)
         .map(|ids| {
             QUESTIONS.with(|q| {
                 let q = q.borrow();
                 ids.iter()
                    .take(limit)
                    .filter_map(|id| q.get(id).cloned())
                    .collect()
             })
         })
         .unwrap_or_default()
    })
}

ic_cdk::export_candid!();
