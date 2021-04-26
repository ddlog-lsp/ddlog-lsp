use syn::parse::{Parse, ParseStream};

mod keyword {
    syn::custom_keyword!(language);
    syn::custom_keyword!(node_kinds);
}

pub(crate) struct NodeKind {
    pub(crate) ident: syn::Ident,
    pub(crate) kind: String,
    pub(crate) named: bool,
}

impl Parse for NodeKind {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let ident = content.parse()?;
        content.parse::<syn::Token![,]>()?;
        let kind = content.parse::<syn::LitStr>()?.value();
        content.parse::<syn::Token![,]>()?;
        let named = content.parse::<syn::LitBool>()?.value();
        Ok(NodeKind { ident, kind, named })
    }
}

pub(crate) struct MacroInput {
    pub(crate) language: crate::language::Language,
    pub(crate) node_kinds: Vec<NodeKind>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        input.parse::<keyword::language>()?;
        input.parse::<syn::Token![:]>()?;
        let language = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        input.parse::<keyword::node_kinds>()?;
        input.parse::<syn::Token![:]>()?;
        let node_kinds = {
            let content;
            syn::bracketed!(content in input);
            content
                .parse_terminated::<NodeKind, syn::Token![,]>(|b| b.parse())?
                .into_iter()
                .collect()
        };
        input.parse::<syn::Token![,]>().ok();

        Ok(MacroInput { language, node_kinds })
    }
}
