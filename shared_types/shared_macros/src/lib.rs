use synstructure::decl_derive;

decl_derive!([ParseEnum] => derive_parse_enum);

fn derive_parse_enum(s: ::synstructure::Structure) -> ::proc_macro2::TokenStream {
    let variants = s.variants().iter().map(|v| {
        let name = v.ast().ident;
        if v.ast().fields.is_empty() {
            return quote::quote! {Self::#name};
        }
        let parse_bindings = v.bindings().iter().map(|binding| {
            quote::quote! {
                let #binding = input.parse()?;
            }
        });
        let bindings = v.bindings();

        quote::quote! {
            #(#parse_bindings)*
            Self::#name(#(#bindings)*)
        }
    });

    let body = quote::quote! {
        let ident = input.parse::<::syn::Ident>()?;
        match ident.to_string().as_str() {
            #(stringify!(#variants) => {
                ::syn::Result::Ok(#variants)
            })*
            _ => return Err(::syn::Error::new_spanned(ident, "Unknown variant")),
        }
    };

    s.gen_impl(quote::quote! {
        gen impl ::syn::parse::Parse for @Self {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                #body
            }
        }
    })
}
