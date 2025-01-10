use verbs::Verb;

pub mod utils;
pub mod verbs;

#[derive(Clone)]
pub enum WordType {
    Adjective(),
    Noun(),
    Verb(Verb),
}
