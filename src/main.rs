mod prelude;
use prelude::*;

fn main() {
    println!("{}", japanese_macros::verb!("食べ", ConjugationForm::Stem));
}
