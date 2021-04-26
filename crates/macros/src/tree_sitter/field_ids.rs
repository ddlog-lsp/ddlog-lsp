use syn::parse::{Parse, ParseStream};

mod keyword {
    syn::custom_keyword!(language);
    syn::custom_keyword!(fields);
}

pub(crate) struct Field {
    pub(crate) ident: syn::Ident,
    pub(crate) name: String,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let ident = content.parse()?;
        content.parse::<syn::Token![,]>()?;
        let name = content.parse::<syn::LitStr>()?.value();
        Ok(Field { ident, name })
    }
}

pub(crate) struct MacroInput {
    pub(crate) language: crate::language::Language,
    pub(crate) fields: Vec<Field>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        input.parse::<keyword::language>()?;
        input.parse::<syn::Token![:]>()?;
        let language = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        input.parse::<keyword::fields>()?;
        input.parse::<syn::Token![:]>()?;
        let fields = {
            let content;
            syn::bracketed!(content in input);
            content
                .parse_terminated::<Field, syn::Token![,]>(|b| b.parse())?
                .into_iter()
                .collect()
        };
        input.parse::<syn::Token![,]>().ok();

        Ok(MacroInput { language, fields })
    }
}
