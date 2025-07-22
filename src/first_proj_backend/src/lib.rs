use std::cell::RefCell;

use ic_cdk::query;
use ic_cdk::update;

thread_local! {
    static NOTES: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[update]
fn add_note(note: String) {
    NOTES.with(|notes| notes.borrow_mut().push(note));
}

#[query]
fn get_notes() -> Vec<String> {
    NOTES.with(|notes| notes.borrow().clone())
}

#[update]
fn update_note(index: usize, new_note: String) {
    NOTES.with(|notes| {
        if let Some(note) = notes.borrow_mut().get_mut(index) {
            *note = new_note;
        }
    });
}

#[update]
fn delete_note(index: usize) {
    NOTES.with(|notes| {
        let mut notes = notes.borrow_mut();
        if index < notes.len() {
            notes.remove(index);
        }
    });
}

ic_cdk::export_candid!();


//candid-extractor target/wasm32-unknown-unknown/release/first_proj_backend.wasm > first_proj_backend.did