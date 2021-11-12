use crate::core::language::dl;
use lsp_text::RopeExt;

#[derive(Clone, Debug)]
pub struct ModulePath {
    components: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Import {
    module_path: ModulePath,
    module_alias: Option<String>,
}

impl Import {
    fn new<'tree>(content: &ropey::Rope, node: tree_sitter::Node<'tree>) -> Self {
        let module_path = {
            let node = node
                .child_by_field_id(dl::field::MODULE_PATH)
                .expect("\"module_path\" should always exist if tree is well-formed");
            let components = content
                .utf8_text_for_tree_sitter_node(&node)
                .split("::")
                .map(String::from)
                .collect::<Vec<_>>();
            ModulePath { components }
        };
        let module_alias = {
            let node = node.child_by_field_id(dl::field::MODULE_ALIAS);
            node.map(|inner| content.utf8_text_for_tree_sitter_node(&inner).into_owned())
        };
        Self {
            module_path,
            module_alias,
        }
    }
}

pub fn collect_imports(content: &ropey::Rope, tree: &tree_sitter::Tree) -> Vec<Import> {
    vec![tree.root_node()]
        .into_iter()
        .flat_map(|root| {
            root.children(&mut root.walk())
                .filter(|annotated_item| dl::kind::ANNOTATED_ITEM == annotated_item.kind_id())
                .collect::<Vec<_>>()
        })
        .flat_map(|annotated_item| {
            annotated_item
                .children(&mut annotated_item.walk())
                .filter(|item| dl::kind::ITEM == item.kind_id())
                .collect::<Vec<_>>()
        })
        .flat_map(|item| {
            item.children(&mut item.walk())
                .filter(|import| dl::kind::IMPORT == import.kind_id())
                .collect::<Vec<_>>()
        })
        .map(|node| Import::new(content, node))
        .collect()
}
