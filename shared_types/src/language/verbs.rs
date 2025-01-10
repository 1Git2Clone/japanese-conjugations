use crate::prelude::*;

#[derive(Clone)]
pub enum VerbType {
    Irregular(String),
    Ichidan,
    Godan,
}

impl VerbType {
    pub fn from_stem(stem: &str) -> Self {
        use VerbType as VT;

        match stem {
            "食べ" => VT::Ichidan,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub struct Verb {
    pub _type: VerbType,
    pub conjugation_form: ConjugationForm,
    pub stem: String,
}

impl Verb {
    pub fn new(stem: String, conjugation_form: ConjugationForm) -> Self {
        Verb {
            _type: VerbType::from_stem(&stem),
            conjugation_form,
            stem,
        }
    }
}

impl Conjugatable for Verb {}

impl ::syn::parse::Parse for Verb {
    fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
        let stem = input.parse::<::syn::LitStr>()?;
        _ = input.parse::<::syn::Token![,]>()?;
        let conjugation_form =
            input.parse::<crate::language::utils::conjugate::ConjugationForm>()?;

        Ok(Verb {
            _type: VerbType::from_stem(&stem.value()),
            conjugation_form,
            stem: stem.value(),
        })
    }
}

impl ::quote::ToTokens for Verb {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let conjugation = self
            .conjugate(
                &self.stem,
                self.conjugation_form,
                &WordType::Verb(self.clone()),
            )
            .value();

        conjugation.to_tokens(tokens);
    }
}
