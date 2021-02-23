//! Functions for working with the `.dat` grammar.

/// Tree-sitter language for the `.dat` grammar.
#[cfg(not(target_arch = "wasm32"))]
#[allow(unsafe_code)]
pub fn language() -> tree_sitter::Language {
    let inner = unsafe { crate::tree_sitter_ddlog_dat() };
    inner.into()
}

/// Tree-sitter language for the `.dat` grammar.
#[cfg(target_arch = "wasm32")]
pub fn language() -> tree_sitter::Language {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../../../vendor/tree-sitter-ddlog/ddlog/dat/tree-sitter-ddlog_dat.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let result = futures::future::block_on(future).unwrap();
    let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
    inner.into()
}

pub mod field {
    #![allow(missing_docs)]

    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref IDENTIFIER: u16 = super::language().field_id_for_name("identifier").unwrap();
    }
}

pub mod kind {
    #![allow(missing_docs)]

    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref ATOM_ELEM: u16 = super::language().id_for_node_kind("atom_elem", true);
        pub static ref ATOM_POS: u16 = super::language().id_for_node_kind("atom_pos", true);
        pub static ref ATOM_REC: u16 = super::language().id_for_node_kind("atom_rec", true);
        pub static ref ATOM: u16 = super::language().id_for_node_kind("atom", true);
        pub static ref CLEAR: u16 = super::language().id_for_node_kind("clear", true);
        pub static ref COMMAND: u16 = super::language().id_for_node_kind("command", true);
        pub static ref COMMENT_LINE: u16 = super::language().id_for_node_kind("comment_line", true);
        pub static ref COMMIT: u16 = super::language().id_for_node_kind("commit", true);
        pub static ref CONS_ARG: u16 = super::language().id_for_node_kind("cons_arg", true);
        pub static ref CONS_ARGS: u16 = super::language().id_for_node_kind("cons_args", true);
        pub static ref DELETE_KEY: u16 = super::language().id_for_node_kind("delete_key", true);
        pub static ref DELETE: u16 = super::language().id_for_node_kind("delete", true);
        pub static ref DUMP_INDEX: u16 = super::language().id_for_node_kind("dump_index", true);
        pub static ref DUMP: u16 = super::language().id_for_node_kind("dump", true);
        pub static ref ECHO: u16 = super::language().id_for_node_kind("echo", true);
        pub static ref EXIT: u16 = super::language().id_for_node_kind("exit", true);
        pub static ref EXP: u16 = super::language().id_for_node_kind("exp", true);
        pub static ref INSERT_OR_UPDATE: u16 = super::language().id_for_node_kind("insert_or_update", true);
        pub static ref INSERT: u16 = super::language().id_for_node_kind("insert", true);
        pub static ref LIT_NUM_HEX: u16 = super::language().id_for_node_kind("lit_num_hex", true);
        pub static ref LIT_SERIALIZED: u16 = super::language().id_for_node_kind("lit_serialized", true);
        pub static ref LIT_STRING: u16 = super::language().id_for_node_kind("lit_string", true);
        pub static ref LOG_LEVEL: u16 = super::language().id_for_node_kind("log_level", true);
        pub static ref MODIFY: u16 = super::language().id_for_node_kind("modify", true);
        pub static ref PROFILE: u16 = super::language().id_for_node_kind("profile", true);
        pub static ref QUERY_INDEX: u16 = super::language().id_for_node_kind("query_index", true);
        pub static ref RECORD_NAMED: u16 = super::language().id_for_node_kind("record_named", true);
        pub static ref RECORD: u16 = super::language().id_for_node_kind("record", true);
        pub static ref ROLLBACK: u16 = super::language().id_for_node_kind("rollback", true);
        pub static ref ROOT: u16 = super::language().id_for_node_kind("root", true);
        pub static ref SERDE_ENCODING: u16 = super::language().id_for_node_kind("serde_encoding", true);
        pub static ref SLEEP: u16 = super::language().id_for_node_kind("sleep", true);
        pub static ref START: u16 = super::language().id_for_node_kind("start", true);
        pub static ref TIMESTAMP: u16 = super::language().id_for_node_kind("timestamp", true);
        pub static ref UPDATE: u16 = super::language().id_for_node_kind("update", true);
        pub static ref UPDATES: u16 = super::language().id_for_node_kind("updates", true);
        pub static ref VAL_ARRAY: u16 = super::language().id_for_node_kind("val_array", true);
        pub static ref VAL_STRUCT: u16 = super::language().id_for_node_kind("val_struct", true);
        pub static ref VAL_TUPLE: u16 = super::language().id_for_node_kind("val_tuple", true);
        pub static ref WORD: u16 = super::language().id_for_node_kind("word", true);
    }
}
