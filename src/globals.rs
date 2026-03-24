use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

// IN_ITEMIZE definisce il contesto temporaneo in cui é iniziata la lettura del \begin{itemize}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListType {
    Itemize,
    Enumerate,
    Description
}

pub struct InListing {
    value: AtomicBool,
    priority: Mutex<Vec<ListType>>,
}

pub static IN_LISTING: InListing = InListing {
    value: AtomicBool::new(false),
    priority: Mutex::new(Vec::new()),
};

pub fn set_in_listing_value(value: bool) {
    IN_LISTING.value.store(value, Ordering::SeqCst);
}

pub fn get_in_listing_value() -> bool {
    IN_LISTING.value.load(Ordering::SeqCst)
}

pub fn get_in_listing_priority() -> Vec<ListType> {
    IN_LISTING.priority.lock().unwrap().clone()
}

pub fn add_in_listing_priority(list_type: ListType) {
    IN_LISTING
        .priority
        .lock()
        .expect("Failed to acquire lock on IN_LISTING priority")
        .push(list_type);

    // se non é vuoto imposta il valore a true, stiamo in listing
    if !IN_LISTING.priority.lock().expect("Failed to acquire lock on IN_LISTING priority").is_empty() {
        set_in_listing_value(true);
    }
}

pub fn read_in_listing_priority() -> Option<ListType> {
    IN_LISTING
        .priority
        .lock()
        .expect("Failed to acquire lock on IN_LISTING priority")
        .last()
        .copied()
}
pub fn pop_in_listing_priority() {
    IN_LISTING
        .priority
        .lock()
        .expect("Failed to acquire lock on IN_LISTING priority")
        .pop();

    // se vuoto imposta il value a false, siamo appena usciti dal listing
    if IN_LISTING.priority.lock().expect("Failed to acquire lock on IN_LISTING priority").is_empty() {
        set_in_listing_value(false);
    }
}
