mod prelude;
use crate::prelude::*;

extern crate shared_types;

#[proc_macro]
pub fn verb(input: TokenStream) -> TokenStream {
    let verb = parse_macro_input!(input as shared_types::language::verbs::Verb);

    quote! {#verb}.into()
}
