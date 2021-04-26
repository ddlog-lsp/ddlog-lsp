use syn::parse::{Parse, ParseStream};

mod keyword {
    syn::custom_keyword!(corpus);
    syn::custom_keyword!(include);
    syn::custom_keyword!(exclude);
    syn::custom_keyword!(handler);
}

pub(crate) struct MacroInput {
    pub(crate) corpus: syn::Ident,
    pub(crate) include: String,
    pub(crate) exclude: Vec<String>,
    pub(crate) handler: syn::Expr,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        input.parse::<keyword::corpus>()?;
        input.parse::<syn::Token![:]>()?;
        let corpus = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        input.parse::<keyword::include>()?;
        input.parse::<syn::Token![:]>()?;
        let include = input.parse::<syn::LitStr>()?.value();
        input.parse::<syn::Token![,]>()?;

        let mut exclude = vec![];
        if input.peek(keyword::exclude) {
            input.parse::<keyword::exclude>()?;
            input.parse::<syn::Token![:]>()?;
            let paths = {
                let content;
                syn::bracketed!(content in input);
                content.parse_terminated::<syn::LitStr, syn::Token![,]>(|b| b.parse())?
            };
            exclude = paths.into_iter().map(|s| s.value()).collect();
            input.parse::<syn::Token![,]>()?;
        }

        input.parse::<keyword::handler>()?;
        input.parse::<syn::Token![:]>()?;
        let handler = input.parse()?;
        input.parse::<syn::Token![,]>().ok();

        Ok(MacroInput {
            corpus,
            include,
            exclude,
            handler,
        })
    }
}
