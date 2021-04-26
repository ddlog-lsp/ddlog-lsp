use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};

pub(crate) struct Language(pub(crate) ddlog_lsp_languages::language::Language);

impl Parse for Language {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let language = input.parse::<syn::LitStr>()?.value();
        let language = ddlog_lsp_languages::language::Language::try_from(language.as_str());
        let language = language.map_err(|_| input.error("invalid language identifier"))?;
        Ok(Language(language))
    }
}
