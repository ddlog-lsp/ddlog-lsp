use crate::{
    core::{self, language::dl},
    provider::document_symbol::{symbol_range, Data, SymbolRange, Work},
};
use std::sync::Arc;

// Document symbol provider definitions for ".dl" files.
pub async fn document_symbol(
    _session: Arc<core::Session>,
    _params: lsp::DocumentSymbolParams,
    _content: &ropey::Rope,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    Ok(None)

    // // Prepare the syntax tree.
    // let tree = session.get_tree(&params.text_document.uri).await?;
    // let tree = tree.lock().await;
    // let node = tree.root_node();

    // // Vector to collect document symbols into as they are constructed.
    // let mut syms: Vec<lsp::DocumentSymbol> = vec![];

    // // Prepare the stack machine:
    // //   data: contains data for constructing upcoming DocumentSymbols
    // //   work: contains remaining tree_sitter nodes to process
    // let mut data: Vec<Data> = vec![];
    // let mut work: Vec<Work> = vec![Work::Node(node)];

    // // The stack machine work loop.
    // while let Some(next) = work.pop() {
    //     match next {
    //         // Construct a DocumentSymbol and pop data stack
    //         Work::Data => {
    //             if let Some(Data {
    //                 node,
    //                 children_count,
    //                 kind,
    //                 name_hint,
    //             }) = data.pop()
    //             {
    //                 let SymbolRange {
    //                     name,
    //                     range,
    //                     selection_range,
    //                 } = { symbol_range(content, node, name_hint, dl::field::IDENTIFIER) };

    //                 #[allow(deprecated)]
    //                 let this = lsp::DocumentSymbol {
    //                     children: if syms.is_empty() {
    //                         None
    //                     } else {
    //                         // Drain the syms array by the number of children nodes we counted
    // for this DocumentSymbol.                         // This allows us to properly
    // reconstruct symbol nesting.                         let children = syms.drain(syms.len()
    // - children_count ..);                         let children = children.rev();
    //   Some(children.collect()) }, deprecated: Default::default(), detail: Default::default(),
    //   kind, name: name.to_string(), range, selection_range, tags: Default::default(), };
    //   syms.push(this); } },

    //         Work::Node(node) if dl::kind::ROOT == node.kind_id() => {
    //             let mut cursor = node.walk();
    //             let annotated_items = node
    //                 .children(&mut cursor)
    //                 .filter(|it| [dl::kind::ANNOTATED_ITEM].contains(&it.kind_id()))
    //                 .map(Work::Node);
    //             work.extend(annotated_items);
    //         },

    //         Work::Node(node) if dl::kind::ANNOTATED_ITEM == node.kind_id() => {
    //             let mut cursor = node.walk();
    //             let items = node
    //                 .children(&mut cursor)
    //                 .filter(|it| [dl::kind::ITEM].contains(&it.kind_id()))
    //                 .map(Work::Node);
    //             work.extend(items);
    //         },

    //         Work::Node(node) if dl::kind::FUNCTION == node.kind_id() => {
    //             let mut cursor = node.walk();
    //             let items = node
    //                 .children(&mut cursor)
    //                 .filter(|it| [dl::kind::FUNCTION_EXTERN,
    // dl::kind::FUNCTION_NORMAL].contains(&it.kind_id()))                 .map(Work::Node);
    //             work.extend(items);
    //         },

    //         Work::Node(node) if dl::kind::FUNCTION_EXTERN == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Function,
    //                 name_hint: "function",
    //             });
    //         },

    //         Work::Node(node) if dl::kind::FUNCTION_NORMAL == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Function,
    //                 name_hint: "function",
    //             });
    //         },

    //         Work::Node(node) if dl::kind::INDEX == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Unknown,
    //                 name_hint: "index",
    //             });
    //         },

    //         Work::Node(node) if dl::kind::ITEM == node.kind_id() => {
    //             let mut cursor = node.walk();
    //             let items = node
    //                 .children(&mut cursor)
    //                 .filter(|it| {
    //                     [
    //                         dl::kind::APPLY,
    //                         dl::kind::FUNCTION,
    //                         dl::kind::INDEX,
    //                         dl::kind::REL,
    //                         dl::kind::TRANSFORMER,
    //                         dl::kind::TYPEDEF,
    //                     ]
    //                     .contains(&it.kind_id())
    //                 })
    //                 .map(Work::Node);
    //             work.extend(items);
    //         },

    //         Work::Node(node) if dl::kind::REL == node.kind_id() => {
    //             let mut cursor = node.walk();
    //             let items = node
    //                 .children(&mut cursor)
    //                 .filter(|it| [dl::kind::REL_ARGS,
    // dl::kind::REL_ELEM].contains(&it.kind_id()))                 .map(Work::Node);
    //             work.extend(items);
    //         },

    //         Work::Node(node) if dl::kind::REL_ARGS == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Unknown,
    //                 name_hint: "rel",
    //             });
    //         },

    //         Work::Node(node) if dl::kind::REL_ELEM == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Unknown,
    //                 name_hint: "rel",
    //             });
    //         },

    //         Work::Node(node) if dl::kind::TRANSFORMER == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Unknown,
    //                 name_hint: "transformer",
    //             });
    //         },

    //         Work::Node(node) if dl::kind::TYPEDEF == node.kind_id() => {
    //             let mut cursor = node.walk();
    //             let items = node
    //                 .children(&mut cursor)
    //                 .filter(|it| [dl::kind::TYPEDEF_EXTERN,
    // dl::kind::TYPEDEF_NORMAL].contains(&it.kind_id()))                 .map(Work::Node);
    //             work.extend(items);
    //         },

    //         Work::Node(node) if dl::kind::TYPEDEF_EXTERN == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Unknown,
    //                 name_hint: "typedef",
    //             });
    //         },

    //         Work::Node(node) if dl::kind::TYPEDEF_NORMAL == node.kind_id() => {
    //             work.push(Work::Data);
    //             data.push(Data {
    //                 node,
    //                 children_count: 0,
    //                 kind: lsp::SymbolKind::Unknown,
    //                 name_hint: "typedef",
    //             });
    //         },

    //         _ => {},
    //     }
    // }
    // // Reverse the syms vec so that document symbols are returned in the correct order. Note that
    // // children nodes are reversed _as the symbols are nested_.
    // syms.reverse();

    // Ok(Some(lsp::DocumentSymbolResponse::Nested(syms)))
}
