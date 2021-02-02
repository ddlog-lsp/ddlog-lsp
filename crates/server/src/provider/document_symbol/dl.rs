use crate::{
    core::{self, language::dl},
    provider::document_symbol::{symbol_range, Data, SymbolRange, Work},
};
use std::sync::Arc;

pub async fn document_symbol(
    session: Arc<core::Session>,
    params: lsp::DocumentSymbolParams,
    content: &ropey::Rope,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let tree = session.get_tree(&params.text_document.uri).await?;
    let tree = tree.lock().await;
    let node = tree.root_node();

    let mut syms: Vec<lsp::DocumentSymbol> = vec![];

    let mut data: Vec<Data> = vec![];
    let mut work: Vec<Work> = vec![Work::Node(node)];

    while let Some(next) = work.pop() {
        match next {
            Work::Data => {
                if let Some(Data {
                    node,
                    children_count,
                    kind,
                    name_hint,
                }) = data.pop()
                {
                    let SymbolRange {
                        name,
                        range,
                        selection_range,
                    } = { symbol_range(content, node, name_hint, *dl::field::IDENTIFIER) };

                    #[allow(deprecated)]
                    let this = lsp::DocumentSymbol {
                        children: if syms.is_empty() {
                            None
                        } else {
                            let children = syms.drain(syms.len() - children_count ..);
                            let children = children.rev();
                            Some(children.collect())
                        },
                        deprecated: Default::default(),
                        detail: Default::default(),
                        kind,
                        name: name.to_string(),
                        range,
                        selection_range,
                        tags: Default::default(),
                    };
                    syms.push(this);
                }
            },

            Work::Node(node) if *dl::kind::ROOT == node.kind_id() => {
                let mut cursor = node.walk();
                let annotated_items = node
                    .children(&mut cursor)
                    .filter(|it| [*dl::kind::ANNOTATED_ITEM].contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(annotated_items);
            },

            Work::Node(node) if *dl::kind::ANNOTATED_ITEM == node.kind_id() => {
                let mut cursor = node.walk();
                let items = node
                    .children(&mut cursor)
                    .filter(|it| [*dl::kind::ITEM].contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(items);
            },

            Work::Node(node) if *dl::kind::FUNCTION == node.kind_id() => {
                let mut cursor = node.walk();
                let items = node
                    .children(&mut cursor)
                    .filter(|it| [*dl::kind::FUNCTION_EXTERN, *dl::kind::FUNCTION_NORMAL].contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(items);
            },

            Work::Node(node) if *dl::kind::FUNCTION_EXTERN == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Function,
                    name_hint: "function",
                });
            },

            Work::Node(node) if *dl::kind::FUNCTION_NORMAL == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Function,
                    name_hint: "function",
                });
            },

            Work::Node(node) if *dl::kind::INDEX == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Unknown,
                    name_hint: "index",
                });
            },

            Work::Node(node) if *dl::kind::ITEM == node.kind_id() => {
                let mut cursor = node.walk();
                let items = node
                    .children(&mut cursor)
                    .filter(|it| {
                        [
                            *dl::kind::APPLY,
                            *dl::kind::FUNCTION,
                            *dl::kind::INDEX,
                            *dl::kind::REL,
                            *dl::kind::TRANSFORMER,
                            *dl::kind::TYPEDEF,
                        ]
                        .contains(&it.kind_id())
                    })
                    .map(Work::Node);
                work.extend(items);
            },

            Work::Node(node) if *dl::kind::REL == node.kind_id() => {
                let mut cursor = node.walk();
                let items = node
                    .children(&mut cursor)
                    .filter(|it| [*dl::kind::REL_ARGS, *dl::kind::REL_ELEM].contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(items);
            },

            Work::Node(node) if *dl::kind::REL_ARGS == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Unknown,
                    name_hint: "rel",
                });
            },

            Work::Node(node) if *dl::kind::REL_ELEM == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Unknown,
                    name_hint: "rel",
                });
            },

            Work::Node(node) if *dl::kind::TRANSFORMER == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Unknown,
                    name_hint: "transformer",
                });
            },

            Work::Node(node) if *dl::kind::TYPEDEF == node.kind_id() => {
                let mut cursor = node.walk();
                let items = node
                    .children(&mut cursor)
                    .filter(|it| [*dl::kind::TYPEDEF_EXTERN, *dl::kind::TYPEDEF_NORMAL].contains(&it.kind_id()))
                    .map(Work::Node);
                work.extend(items);
            },

            Work::Node(node) if *dl::kind::TYPEDEF_EXTERN == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Unknown,
                    name_hint: "typedef",
                });
            },

            Work::Node(node) if *dl::kind::TYPEDEF_NORMAL == node.kind_id() => {
                work.push(Work::Data);
                data.push(Data {
                    node,
                    children_count: 0,
                    kind: lsp::SymbolKind::Unknown,
                    name_hint: "typedef",
                });
            },

            _ => {},
        }
    }
    let results = syms.into_iter().rev().collect();

    Ok(Some(lsp::DocumentSymbolResponse::Nested(results)))
}
