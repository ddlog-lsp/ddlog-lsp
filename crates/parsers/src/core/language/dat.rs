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
        pub static ref APPLY: u16 = super::language().id_for_node_kind("apply", true);
        pub static ref ARG: u16 = super::language().id_for_node_kind("arg", true);
        pub static ref ARG_OPT_TYPE: u16 = super::language().id_for_node_kind("arg_opt_type", true);
        pub static ref ARG_TRANS: u16 = super::language().id_for_node_kind("arg_trans", true);
        pub static ref ATOM_ELEM: u16 = super::language().id_for_node_kind("atom_elem", true);
        pub static ref ATOM_POS: u16 = super::language().id_for_node_kind("atom_pos", true);
        pub static ref ATOM_REC: u16 = super::language().id_for_node_kind("atom_rec", true);
        pub static ref ATTRIBUTE: u16 = super::language().id_for_node_kind("attribute", true);
        pub static ref ATTRIBUTES: u16 = super::language().id_for_node_kind("attributes", true);
        pub static ref CONS_POS: u16 = super::language().id_for_node_kind("cons_pos", true);
        pub static ref CONS_REC: u16 = super::language().id_for_node_kind("cons_rec", true);
        pub static ref EXP_ADD: u16 = super::language().id_for_node_kind("exp_add", true);
        pub static ref EXP_ASSIGN: u16 = super::language().id_for_node_kind("exp_assign", true);
        pub static ref EXP_BINDING: u16 = super::language().id_for_node_kind("exp_binding", true);
        pub static ref EXP_BIT_AND: u16 = super::language().id_for_node_kind("exp_bit_and", true);
        pub static ref EXP_BIT_NEG: u16 = super::language().id_for_node_kind("exp_bit_neg", true);
        pub static ref EXP_BIT_OR: u16 = super::language().id_for_node_kind("exp_bit_or", true);
        pub static ref EXP_BIT_SLICE: u16 = super::language().id_for_node_kind("exp_bit_slice", true);
        pub static ref EXP_BIT_XOR: u16 = super::language().id_for_node_kind("exp_bit_xor", true);
        pub static ref EXP_BLOCK: u16 = super::language().id_for_node_kind("exp_block", true);
        pub static ref EXP_BREAK: u16 = super::language().id_for_node_kind("exp_break", true);
        pub static ref EXP_CAST: u16 = super::language().id_for_node_kind("exp_cast", true);
        pub static ref EXP_CAT: u16 = super::language().id_for_node_kind("exp_cat", true);
        pub static ref EXP_COND: u16 = super::language().id_for_node_kind("exp_cond", true);
        pub static ref EXP_CONTINUE: u16 = super::language().id_for_node_kind("exp_continue", true);
        pub static ref EXP_CONS_POS: u16 = super::language().id_for_node_kind("exp_cons_pos", true);
        pub static ref EXP_CONS_REC: u16 = super::language().id_for_node_kind("exp_cons_rec", true);
        pub static ref EXP_DECL_VAR: u16 = super::language().id_for_node_kind("exp_decl_var", true);
        pub static ref EXP_DIV: u16 = super::language().id_for_node_kind("exp_div", true);
        pub static ref EXP_EQ: u16 = super::language().id_for_node_kind("exp_eq", true);
        pub static ref EXP_FIELD: u16 = super::language().id_for_node_kind("exp_field", true);
        pub static ref EXP_FOR: u16 = super::language().id_for_node_kind("exp_for", true);
        pub static ref EXP_FUN_CALL: u16 = super::language().id_for_node_kind("exp_fun_call", true);
        pub static ref EXP_FUN_CALL_DOT: u16 = super::language().id_for_node_kind("exp_fun_call_dot", true);
        pub static ref EXP_GT: u16 = super::language().id_for_node_kind("exp_gt", true);
        pub static ref EXP_GTEQ: u16 = super::language().id_for_node_kind("exp_gteq", true);
        pub static ref EXP_LAMBDA: u16 = super::language().id_for_node_kind("exp_lambda", true);
        pub static ref EXP_LOG_AND: u16 = super::language().id_for_node_kind("exp_log_and", true);
        pub static ref EXP_LOG_IMP: u16 = super::language().id_for_node_kind("exp_log_imp", true);
        pub static ref EXP_LOG_NEG: u16 = super::language().id_for_node_kind("exp_log_neg", true);
        pub static ref EXP_LOG_OR: u16 = super::language().id_for_node_kind("exp_log_or", true);
        pub static ref EXP_LT: u16 = super::language().id_for_node_kind("exp_lt", true);
        pub static ref EXP_LTEQ: u16 = super::language().id_for_node_kind("exp_lteq", true);
        pub static ref EXP_MATCH: u16 = super::language().id_for_node_kind("exp_match", true);
        pub static ref EXP_MUL: u16 = super::language().id_for_node_kind("exp_mul", true);
        pub static ref EXP_NEG: u16 = super::language().id_for_node_kind("exp_neg", true);
        pub static ref EXP_NEQ: u16 = super::language().id_for_node_kind("exp_neq", true);
        pub static ref EXP_PROJ: u16 = super::language().id_for_node_kind("exp_proj", true);
        pub static ref EXP_REF: u16 = super::language().id_for_node_kind("exp_ref", true);
        pub static ref EXP_REM: u16 = super::language().id_for_node_kind("exp_rem", true);
        pub static ref EXP_RETURN: u16 = super::language().id_for_node_kind("exp_return", true);
        pub static ref EXP_SEQ: u16 = super::language().id_for_node_kind("exp_seq", true);
        pub static ref EXP_SHL: u16 = super::language().id_for_node_kind("exp_shl", true);
        pub static ref EXP_SHR: u16 = super::language().id_for_node_kind("exp_shr", true);
        pub static ref EXP_SLICE: u16 = super::language().id_for_node_kind("exp_slice", true);
        pub static ref EXP_SUB: u16 = super::language().id_for_node_kind("exp_sub", true);
        pub static ref EXP_TRY: u16 = super::language().id_for_node_kind("exp_try", true);
        pub static ref EXP_TUPLE: u16 = super::language().id_for_node_kind("exp_tuple", true);
        pub static ref EXP_TYPE: u16 = super::language().id_for_node_kind("exp_type", true);
        pub static ref EXP_WILD: u16 = super::language().id_for_node_kind("exp_wild", true);
        pub static ref FIELD: u16 = super::language().id_for_node_kind("field", true);
        pub static ref FUNCTION: u16 = super::language().id_for_node_kind("function", true);
        pub static ref FUNCTION_EXTERN: u16 = super::language().id_for_node_kind("function_extern", true);
        pub static ref IMPORT: u16 = super::language().id_for_node_kind("import", true);
        pub static ref INDEX: u16 = super::language().id_for_node_kind("index", true);
        pub static ref INTERPOLATION: u16 = super::language().id_for_node_kind("interpolation", true);
        pub static ref KEY_PRIMARY: u16 = super::language().id_for_node_kind("key_primary", true);
        pub static ref LIT_BOOL: u16 = super::language().id_for_node_kind("lit_bool", true);
        pub static ref LIT_NUM: u16 = super::language().id_for_node_kind("lit_num", true);
        pub static ref LIT_NUM_BIN: u16 = super::language().id_for_node_kind("lit_num_bin", true);
        pub static ref LIT_NUM_DEC: u16 = super::language().id_for_node_kind("lit_num_dec", true);
        pub static ref LIT_NUM_FLOAT: u16 = super::language().id_for_node_kind("lit_num_float", true);
        pub static ref LIT_NUM_HEX: u16 = super::language().id_for_node_kind("lit_num_hex", true);
        pub static ref LIT_NUM_OCT: u16 = super::language().id_for_node_kind("lit_num_oct", true);
        pub static ref LIT_MAP: u16 = super::language().id_for_node_kind("lit_map", true);
        pub static ref LIT_STRING: u16 = super::language().id_for_node_kind("lit_string", true);
        pub static ref LIT_VEC: u16 = super::language().id_for_node_kind("lit_vec", true);
        pub static ref MODULE_ALIAS: u16 = super::language().id_for_node_kind("module_alias", true);
        pub static ref MODULE_PATH: u16 = super::language().id_for_node_kind("module_path", true);
        pub static ref NAME: u16 = super::language().id_for_node_kind("name", true);
        pub static ref NAME_ARG: u16 = super::language().id_for_node_kind("name_arg", true);
        pub static ref NAME_CONS: u16 = super::language().id_for_node_kind("name_cons", true);
        pub static ref NAME_FIELD: u16 = super::language().id_for_node_kind("name_field", true);
        pub static ref NAME_FUNC: u16 = super::language().id_for_node_kind("name_func", true);
        pub static ref NAME_INDEX: u16 = super::language().id_for_node_kind("name_index", true);
        pub static ref NAME_REL: u16 = super::language().id_for_node_kind("name_rel", true);
        pub static ref NAME_TRANS: u16 = super::language().id_for_node_kind("name_trans", true);
        pub static ref NAME_TYPE: u16 = super::language().id_for_node_kind("name_type", true);
        pub static ref NAME_VAR_TERM: u16 = super::language().id_for_node_kind("name_var_term", true);
        pub static ref NAME_VAR_TYPE: u16 = super::language().id_for_node_kind("name_var_type", true);
        pub static ref PAT_CONS_POS: u16 = super::language().id_for_node_kind("pat_cons_pos", true);
        pub static ref PAT_CONS_REC: u16 = super::language().id_for_node_kind("pat_cons_rec", true);
        pub static ref PAT_TERM_DECL_VAR: u16 = super::language().id_for_node_kind("pat_term_decl_var", true);
        pub static ref PAT_TUPLE: u16 = super::language().id_for_node_kind("pat_tuple", true);
        pub static ref PAT_TYPE: u16 = super::language().id_for_node_kind("pat_type", true);
        pub static ref PAT_WILD: u16 = super::language().id_for_node_kind("pat_wild", true);
        pub static ref REL_ARGS: u16 = super::language().id_for_node_kind("rel_args", true);
        pub static ref REL_ELEM: u16 = super::language().id_for_node_kind("rel_elem", true);
        pub static ref REL_ROLE: u16 = super::language().id_for_node_kind("rel_role", true);
        pub static ref REL_SEMANTICS: u16 = super::language().id_for_node_kind("rel_semantics", true);
        pub static ref RHS_ATOM_NEG: u16 = super::language().id_for_node_kind("rhs_atom_neg", true);
        pub static ref RHS_FLAT_MAP: u16 = super::language().id_for_node_kind("rhs_flat_map", true);
        pub static ref RHS_GROUPING: u16 = super::language().id_for_node_kind("rhs_grouping", true);
        pub static ref RHS_INSPECT: u16 = super::language().id_for_node_kind("rhs_inspect", true);
        pub static ref ROOT: u16 = super::language().id_for_node_kind("ROOT", true);
        pub static ref RULE: u16 = super::language().id_for_node_kind("rule", true);
        pub static ref STATEMENT_ASSIGN: u16 = super::language().id_for_node_kind("statement_assign", true);
        pub static ref STATEMENT_BLOCK: u16 = super::language().id_for_node_kind("statement_block", true);
        pub static ref STATEMENT_EMPTY: u16 = super::language().id_for_node_kind("statement_empty", true);
        pub static ref STATEMENT_IF: u16 = super::language().id_for_node_kind("statement_if", true);
        pub static ref STATEMENT_INSERT: u16 = super::language().id_for_node_kind("statement_insert", true);
        pub static ref STATEMENT_FOR: u16 = super::language().id_for_node_kind("statement_for", true);
        pub static ref STATEMENT_MATCH: u16 = super::language().id_for_node_kind("statement_match", true);
        pub static ref STRING_QUOTED: u16 = super::language().id_for_node_kind("string_quoted", true);
        pub static ref STRING_QUOTED_ESCAPE: u16 = super::language().id_for_node_kind("string_quoted_escape", true);
        pub static ref STRING_RAW: u16 = super::language().id_for_node_kind("string_raw", true);
        pub static ref STRING_RAW_INTERPOLATED: u16 =
            super::language().id_for_node_kind("string_raw_interpolated", true);
        pub static ref TRANSFORMER: u16 = super::language().id_for_node_kind("transformer", true);
        pub static ref TYPE_BIGINT: u16 = super::language().id_for_node_kind("type_bigint", true);
        pub static ref TYPE_BIT: u16 = super::language().id_for_node_kind("type_bit", true);
        pub static ref TYPE_BOOL: u16 = super::language().id_for_node_kind("type_bool", true);
        pub static ref TYPE_DOUBLE: u16 = super::language().id_for_node_kind("type_double", true);
        pub static ref TYPE_FLOAT: u16 = super::language().id_for_node_kind("type_float", true);
        pub static ref TYPE_FUN: u16 = super::language().id_for_node_kind("type_fun", true);
        pub static ref TYPE_SIGNED: u16 = super::language().id_for_node_kind("type_signed", true);
        pub static ref TYPE_STRING: u16 = super::language().id_for_node_kind("type_string", true);
        pub static ref TYPE_TRANS_FUN: u16 = super::language().id_for_node_kind("type_trans_fun", true);
        pub static ref TYPE_TRANS_REL: u16 = super::language().id_for_node_kind("type_trans_rel", true);
        pub static ref TYPE_TUPLE: u16 = super::language().id_for_node_kind("type_tuple", true);
        pub static ref TYPE_UNION: u16 = super::language().id_for_node_kind("type_union", true);
        pub static ref TYPE_USER: u16 = super::language().id_for_node_kind("type_user", true);
        pub static ref TYPE_VAR: u16 = super::language().id_for_node_kind("type_var", true);
        pub static ref TYPEDEF: u16 = super::language().id_for_node_kind("typedef", true);
        pub static ref TYPEDEF_EXTERN: u16 = super::language().id_for_node_kind("typedef_extern", true);
        pub static ref WORD: u16 = super::language().id_for_node_kind("word", true);
    }
}
