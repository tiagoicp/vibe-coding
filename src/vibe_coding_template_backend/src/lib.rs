use ic_cdk::export_candid;
use std::cell::RefCell;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Default)]
enum Language {
    PT,
    FR,
    ES,
    IT,
    #[default]
    EN, // Default/English
}

thread_local! {
    static COUNTER: RefCell<u64> = const { RefCell::new(0) };
}

#[ic_cdk::query]
fn greet(name: String, language: Option<Language>) -> String {
    let greeting = match language.unwrap_or_default() {
        Language::PT => "Olá",
        Language::FR => "Bonjour",
        Language::ES => "Hola",
        Language::IT => "Ciao",
        Language::EN => "Hello",
    };

    format!("{}, {}!", greeting, name)
}

#[ic_cdk::update]
fn increment() -> u64 {
    COUNTER.with(|counter| {
        let val = *counter.borrow() + 1;
        *counter.borrow_mut() = val;
        val
    })
}

#[ic_cdk::query]
fn get_count() -> u64 {
    COUNTER.with(|counter| *counter.borrow())
}

#[ic_cdk::update]
fn set_count(value: u64) -> u64 {
    COUNTER.with(|counter| {
        *counter.borrow_mut() = value;
        value
    })
}

export_candid!();
