use proc_macro::TokenStream;
use syn::Meta;

#[proc_macro_derive(Command, attributes(command))]
pub fn command_derive(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(input);

    let (idents, docs, chars) = match data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let mut idents = Vec::new();
            let mut docs = Vec::new();
            let mut chars = Vec::new();

            for var in &variants {
                match parse_variant_doc_comment(&var.attrs) {
                    Some(v) => docs.push(v),
                    None => panic!("no comments"),
                }

                match parse_variant_command(&var.attrs) {
                    Some(c) => chars.push(c),
                    None => panic!("no command attr"),
                }

                idents.push(var.ident.clone());
            }

            (idents, docs, chars)
        }
        _ => panic!(),
    };

    let output = quote::quote! {
        impl #ident {
            pub fn legend(&self) -> &'static str {
                match self {
                    #( #ident::#idents => #docs ),*
                }
            }
        }

        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    #( #ident::#idents => write!(f, #chars) ),*
                }
            }
        }

    };

    output.into()
}

fn parse_variant_command(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("command") {
            if let syn::Expr::Lit(l) = attr.parse_args().unwrap() {
                if let syn::Lit::Str(s) = &l.lit {
                    return Some(s.value());
                }
            }
        }
    }

    None
}

fn parse_variant_doc_comment(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(v) = &attr.meta {
                if let syn::Expr::Lit(lit) = &v.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        let trim = s.value().trim().to_string();
                        return Some(trim);
                    }
                }
            }
        }
    }

    None
}
