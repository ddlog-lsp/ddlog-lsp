use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn alphabet(prefix: &'static str) -> impl Iterator<Item = String> + Clone {
    ('A' ..= 'Z').cycle().zip(0 ..).map(move |(c, i)| {
        let suffix = i / 26;
        format!("{}{}{}", prefix, c, suffix)
    })
}

pub fn idents(depth: usize, prefix: Option<&'static str>) -> impl Iterator<Item = Ident> + Clone {
    let prefix = prefix.unwrap_or("");
    alphabet(prefix)
        .take(depth)
        .map(|x| Ident::new(x.as_str(), Span::call_site()))
}

pub fn parsers_where(inputs: impl Iterator<Item = Ident>) -> impl Iterator<Item = TokenStream> {
    inputs.map(|i| {
        quote! {
            #i: Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
        }
    })
}

pub mod impls {
    use syn::parse::{Parse, ParseStream};

    pub struct MacroInput {
        pub depth: usize,
    }

    impl Parse for MacroInput {
        fn parse(input: ParseStream) -> syn::parse::Result<Self> {
            let lit = input.parse::<syn::LitInt>()?;
            let depth = lit.base10_parse()?;
            if depth == 0 {
                return Err(syn::Error::new(lit.span(), "depth must be non-zero"));
            }
            Ok(MacroInput { depth })
        }
    }
}
