//! Macros for the DDlog language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use glob::glob;
use proc_macro::TokenStream;
use quote::quote;

mod language;
mod testing;
mod tree_sitter;
mod visitor;

/// Generate tests from a corpus of wasm modules on the filesystem.
///
/// # Arguments
///
/// * `corpus` - name of the generated submodule containing the individual tests
/// * `include` - glob pattern of files to include for testing
/// * `exclude` - array of file names to exclude from testing
///
/// # Examples
///
/// ```
/// corpus_tests! {
///     corpus: annotations,
///     include: "vendor/corpus/vendor/WebAssembly/annotations/test/core/*.wast",
///     exclude: ["annotations.wast"],
/// }
/// ```
#[proc_macro]
pub fn corpus_tests(input: TokenStream) -> TokenStream {
    let testing::corpus_tests::MacroInput {
        corpus,
        include,
        exclude,
        handler,
    } = syn::parse_macro_input!(input as crate::testing::corpus_tests::MacroInput);
    // Compute a string representation for the corpus name.
    let corpus_name = corpus.to_string();
    let corpus_name = corpus_name.as_str();

    // Compute the paths from the glob pattern.
    let paths = glob(&include).unwrap();

    // Prepare the vector of syntax items; these items are the individual test
    // functions that will be enclosed in the generated test submodule.
    let mut content = vec![];

    for path in paths {
        // Ensure the path is canonicalized and absolute
        let path = path.unwrap().canonicalize().unwrap();
        let path_name = path.to_str().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // Skip the file if contained in the exclude list; otherwise continue.
        if !exclude.contains(&String::from(file_name)) {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let test_name = heck::SnakeCase::to_snake_case(file_stem);
            let test_name = format!("r#{}", test_name);

            // Compute the test identifier.
            let test = syn::parse_str::<syn::Ident>(&test_name).unwrap();

            // Generate the individual test function for the given file.
            let item = quote! {
                #[test]
                fn #test() {
                    #handler(#corpus_name, #path_name);
                }
            };
            content.push(item);
        }
    }

    // Generate the enclosing test submodule for the given corpus.
    let module = quote! {
        mod #corpus {
            // Include the test functions generated from the corpus files.
            #(#content)*
        }
    };

    module.into()
}

#[allow(missing_docs)]
#[proc_macro]
pub fn field_ids(input: TokenStream) -> TokenStream {
    use ddlog_lsp_languages::language;

    let macro_input = syn::parse_macro_input!(input as crate::tree_sitter::field_ids::MacroInput);

    #[allow(unsafe_code)]
    let language = match macro_input.language.0 {
        language::Language::DDlogDat => language::dat(),
        language::Language::DDlogDl => language::dl(),
    };

    let mut content = vec![];

    for field in macro_input.fields {
        let ident = field.ident;
        let name = field.name.as_str();
        let value = language.field_id_for_name(name).expect("field does not exist");
        let item = quote! {
            pub const #ident: u16 = #value;
        };
        content.push(item);
    }

    let result = quote! {
        #(#content)*
    };

    result.into()
}

#[allow(missing_docs)]
#[proc_macro]
pub fn node_kind_ids(input: TokenStream) -> TokenStream {
    use ddlog_lsp_languages::language;

    let macro_input = syn::parse_macro_input!(input as crate::tree_sitter::node_kind_ids::MacroInput);

    #[allow(unsafe_code)]
    let language = match macro_input.language.0 {
        language::Language::DDlogDat => language::dat(),
        language::Language::DDlogDl => language::dl(),
    };

    let mut content = vec![];

    for node_kind in macro_input.node_kinds {
        let ident = node_kind.ident;
        let kind = node_kind.kind.as_str();
        let value = language.id_for_node_kind(kind, node_kind.named);
        let item = quote! {
            pub const #ident: u16 = #value;
        };
        content.push(item);
    }

    let result = quote! {
        #(#content)*
    };

    result.into()
}

#[allow(missing_docs)]
#[proc_macro]
pub fn impl_choice(input: TokenStream) -> TokenStream {
    let crate::visitor::utils::impls::MacroInput { depth } =
        syn::parse_macro_input!(input as crate::visitor::utils::impls::MacroInput);

    let type_inputs_idents = crate::visitor::utils::idents(depth, Some("I"));
    let type_inputs_tuple = {
        let idents = type_inputs_idents.clone();
        quote! { (#((u16, #idents)),*,) }
    };

    let type_generics = type_inputs_idents.clone();
    let type_inputs_where = crate::visitor::utils::parsers_where(type_inputs_idents);

    let choice_body = {
        let cases = (0 .. depth).map(|n| {
            let i = syn::Index::from(n);
            quote! {
                // let language: tree_sitter::Language = visitor.walker().language().into();
                // let have_name = language.node_kind_for_id(self.#i.0).unwrap();
                // let want_name = language.node_kind_for_id(kind).unwrap();
                // log::info!("i: {}; have: {}; want: {}", #i, have_name, want_name);
                if self.#i.0 == kind {
                    let result = (self.#i.1)(visitor, m);
                    // if result.is_err() {
                    //     log::info!("result error: {:#?}", result);
                    // }
                    return result;
                }
            }
        });

        quote! {
            let kind = {
                let prev = visitor.walker().node();
                if !visitor.walker().goto_next(crate::node::GotoNext::StepInto, true) {
                    let language = visitor.walker().language();
                    let range = visitor.walker().node().range();
                    let data = ();
                    let error = SyntaxError::walker_move_error(language, range, data);
                    return Err(error);
                }
                let kind = visitor.walker().node().kind_id();
                visitor.walker().reset(prev);
                kind
            };

            let mut choices = vec![];

            #(#cases)*

            let language = visitor.walker().language();
            let range = visitor.walker().node().range();
            let data = ();
            let error = SyntaxError::choice_failure(language, range, choices, data);
            Err(error)
        }
    };

    let result = quote! {
        impl<'tree, Vis, #(#type_generics),*> Choice<'tree, Vis> for #type_inputs_tuple
        where
            Vis: HasWalker<'tree> + ?Sized,
            #(#type_inputs_where),*
        {
            fn choice(&self, visitor: &mut Vis, m: NodeMove) -> Result<(), SyntaxError<()>> {
                #choice_body
            }
        }
    };

    result.into()
}

#[allow(missing_docs)]
#[proc_macro]
pub fn impl_seq(input: TokenStream) -> TokenStream {
    let crate::visitor::utils::impls::MacroInput { depth } =
        syn::parse_macro_input!(input as crate::visitor::utils::impls::MacroInput);

    let type_inputs_idents = crate::visitor::utils::idents(depth, Some("I"));
    let type_inputs_tuple = {
        let idents = type_inputs_idents.clone();
        quote! { (#(#idents),*,) }
    };

    let type_generics = type_inputs_idents.clone();
    let type_inputs_where = crate::visitor::utils::parsers_where(type_inputs_idents);

    let seq_inner = {
        let accessors = (0 .. depth).map(syn::Index::from);
        quote! {
            #(self.#accessors(visitor, m)?);*;
            Ok(())
        }
    };

    let result = quote! {
        impl<'tree, Vis, #(#type_generics),*> Seq<'tree, Vis> for #type_inputs_tuple
        where
            Vis: HasWalker<'tree> + ?Sized,
            #(#type_inputs_where),*
        {
            fn seq(&self, visitor: &mut Vis, m: NodeMove) -> Result<(), SyntaxError<()>> {
                #seq_inner
            }
        }
    };

    result.into()
}
