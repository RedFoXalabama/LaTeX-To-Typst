use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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


// ---------------------------- SECTIONS AND CHAPTER PART COUNTER ----------------------------------
pub struct PartCounter{
    value: AtomicUsize,
}
pub static PART_COUNTER: PartCounter = PartCounter {
    value: AtomicUsize::new(0),
};
pub fn update_part_counter() {
    PART_COUNTER.value.fetch_add(1, Ordering::SeqCst);
}
pub fn get_part_counter() -> String {
    let value = PART_COUNTER.value.load(Ordering::SeqCst);
    to_roman(value)
}
fn to_roman(mut n: usize) -> String {
    if n == 0 { //valore standard in caso di errore
        return "N".to_string();
    }
    let mapping = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    for &(value, symbol) in &mapping {
        while n >= value {
            result.push_str(symbol);
            n -= value;
        }
    }
    result
}