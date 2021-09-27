//! Functions for working with the `.dl` grammar.

use crate::{
    error::SyntaxError,
    language::{utils, HasWalker, NodeMove},
    node::{GotoNext, NodeWalker},
};
use ddlog_lsp_languages::language::Language;

pub mod field {
    #![allow(missing_docs)]

    ddlog_lsp_macros::field_ids! {
        language: "ddlog.dl",
        fields: [
        ]
    }
}

#[allow(missing_docs)]
pub mod kind {
    ddlog_lsp_macros::node_kind_ids! {
        language: "ddlog.dl",
        node_kinds: [
            (ROOT, "ROOT", true),
            (ANNOTATED_ITEM, "annotated_item", true),
            (APPLY, "apply", true),
            (ARG, "arg", true),
            (ARG_OPT_TYPE, "arg_opt_type", true),
            (ARG_TRANS, "arg_trans", true),
            (ATOM, "atom", true),
            (ATOM_ELEM, "atom_elem", true),
            (ATOM_POS, "atom_pos", true),
            (ATOM_REC, "atom_rec", true),
            (ATTRIBUTE, "attribute", true),
            (ATTRIBUTES, "attributes", true),
            (COMMENT_BLOCK, "comment_block", true),
            (COMMENT_BLOCK_INNER, "comment_block_inner", true),
            (COMMENT_LINE, "comment_line", true),
            (CONS, "cons", true),
            (CONS_POS, "cons_pos", true),
            (CONS_REC, "cons_rec", true),
            (ESCAPE_SEQUENCE, "escape_sequence", true),
            (ESCAPE_SEQUENCE_INTERPOLATED, "escape_sequence_interpolated", true),
            (EXP, "exp", true),
            (EXP_ADD, "exp_add", true),
            (EXP_ASSIGN, "exp_assign", true),
            (EXP_BINDING, "exp_binding", true),
            (EXP_BIT_AND, "exp_bit_and", true),
            (EXP_BIT_NEG, "exp_bit_neg", true),
            (EXP_BIT_OR, "exp_bit_or", true),
            (EXP_BIT_SLICE, "exp_bit_slice", true),
            (EXP_BIT_XOR, "exp_bit_xor", true),
            (EXP_BLOCK, "exp_block", true),
            (EXP_BREAK, "exp_break", true),
            (EXP_CAST, "exp_cast", true),
            (EXP_CAT, "exp_cat", true),
            (EXP_COND, "exp_cond", true),
            (EXP_CONS_POS, "exp_cons_pos", true),
            (EXP_CONS_REC, "exp_cons_rec", true),
            (EXP_CONTINUE, "exp_continue", true),
            (EXP_DECL_VAR, "exp_decl_var", true),
            (EXP_DIV, "exp_div", true),
            (EXP_EQ, "exp_eq", true),
            (EXP_FIELD, "exp_field", true),
            (EXP_FOR, "exp_for", true),
            (EXP_FUN_CALL, "exp_fun_call", true),
            (EXP_FUN_CALL_DOT, "exp_fun_call_dot", true),
            (EXP_GT, "exp_gt", true),
            (EXP_GTEQ, "exp_gteq", true),
            (EXP_LAMBDA, "exp_lambda", true),
            (EXP_LAMBDA_BRANCH_0, "exp_lambda_branch_0", true),
            (EXP_LAMBDA_BRANCH_1, "exp_lambda_branch_1", true),
            (EXP_LIT, "exp_lit", true),
            (EXP_LOG_AND, "exp_log_and", true),
            (EXP_LOG_IMP, "exp_log_imp", true),
            (EXP_LOG_NEG, "exp_log_neg", true),
            (EXP_LOG_OR, "exp_log_or", true),
            (EXP_LT, "exp_lt", true),
            (EXP_LTEQ, "exp_lteq", true),
            (EXP_MATCH, "exp_match", true),
            (EXP_MUL, "exp_mul", true),
            (EXP_NEG, "exp_neg", true),
            (EXP_NEQ, "exp_neq", true),
            (EXP_PROJ, "exp_proj", true),
            (EXP_PROJ_DIGITS, "exp_proj_digits", true),
            (EXP_REF, "exp_ref", true),
            (EXP_REM, "exp_rem", true),
            (EXP_RETURN, "exp_return", true),
            (EXP_SEQ, "exp_seq", true),
            (EXP_SHL, "exp_shl", true),
            (EXP_SHR, "exp_shr", true),
            (EXP_SLICE, "exp_slice", true),
            (EXP_SUB, "exp_sub", true),
            (EXP_TRY, "exp_try", true),
            (EXP_TUPLE, "exp_tuple", true),
            (EXP_TYPE, "exp_type", true),
            (EXP_WILD, "exp_wild", true),
            (FIELD, "field", true),
            (FUNCTION, "function", true),
            (FUNCTION_EXTERN, "function_extern", true),
            (FUNCTION_NORMAL, "function_normal", true),
            (FUNCTION_NORMAL_BRANCH_0, "function_normal_branch_0", true),
            (FUNCTION_NORMAL_BRANCH_1, "function_normal_branch_1", true),
            (IDENT, "ident", true),
            (IDENT_LOWER, "ident_lower", true),
            (IDENT_LOWER_SCOPED, "ident_lower_scoped", true),
            (IDENT_SCOPED, "ident_scoped", true),
            (IDENT_UPPER, "ident_upper", true),
            (IDENT_UPPER_SCOPED, "ident_upper_scoped", true),
            (IMPORT, "import", true),
            (INDEX, "index", true),
            (INTERPOLATION, "interpolation", true),
            (ITEM, "item", true),
            (KEY_PRIMARY, "key_primary", true),
            (LIT_BOOL, "lit_bool", true),
            (LIT_MAP, "lit_map", true),
            (LIT_NUM, "lit_num", true),
            (LIT_NUM_BRANCH_0, "lit_num_branch_0", true),
            (LIT_NUM_BRANCH_1, "lit_num_branch_1", true),
            (LIT_NUM_BRANCH_10, "lit_num_branch_10", true),
            (LIT_NUM_BRANCH_11, "lit_num_branch_11", true),
            (LIT_NUM_BRANCH_12, "lit_num_branch_12", true),
            (LIT_NUM_BRANCH_13, "lit_num_branch_13", true),
            (LIT_NUM_BRANCH_14, "lit_num_branch_14", true),
            (LIT_NUM_BRANCH_15, "lit_num_branch_15", true),
            (LIT_NUM_BRANCH_16, "lit_num_branch_16", true),
            (LIT_NUM_BRANCH_17, "lit_num_branch_17", true),
            (LIT_NUM_BRANCH_18, "lit_num_branch_18", true),
            (LIT_NUM_BIN, "lit_num_bin", true),
            (LIT_NUM_DEC, "lit_num_dec", true),
            (LIT_NUM_FLOAT, "lit_num_float", true),
            (LIT_NUM_HEX, "lit_num_hex", true),
            (LIT_NUM_OCT, "lit_num_oct", true),
            (LIT_STRING, "lit_string", true),
            (LIT_VEC, "lit_vec", true),
            (MISC_PAT0, "misc_pat0", true),
            (MODULE_ALIAS, "module_alias", true),
            (MODULE_PATH, "module_path", true),
            (NAME, "name", true),
            (NAME_ARG, "name_arg", true),
            (NAME_CONS, "name_cons", true),
            (NAME_FIELD, "name_field", true),
            (NAME_FUNC, "name_func", true),
            (NAME_INDEX, "name_index", true),
            (NAME_REL, "name_rel", true),
            (NAME_TRANS, "name_trans", true),
            (NAME_TYPE, "name_type", true),
            (NAME_VAR_TERM, "name_var_term", true),
            (NAME_VAR_TYPE, "name_var_type", true),
            (PAT, "pat", true),
            (PAT_CONS, "pat_cons", true),
            (PAT_CONS_POS, "pat_cons_pos", true),
            (PAT_CONS_REC, "pat_cons_rec", true),
            (PAT_LIT, "pat_lit", true),
            (PAT_TERM_DECL_VAR, "pat_term_decl_var", true),
            (PAT_TUPLE, "pat_tuple", true),
            (PAT_TYPE, "pat_type", true),
            (PAT_WILD, "pat_wild", true),
            (REL, "rel", true),
            (REL_ARGS, "rel_args", true),
            (REL_ELEM, "rel_elem", true),
            (REL_ROLE, "rel_role", true),
            (REL_SEMANTICS, "rel_semantics", true),
            (RHS, "rhs", true),
            (RHS_ATOM_NEG, "rhs_atom_neg", true),
            (RHS_FLAT_MAP, "rhs_flat_map", true),
            (RHS_GROUPING, "rhs_grouping", true),
            (RHS_INSPECT, "rhs_inspect", true),
            (RULE, "rule", true),
            (RULE_END, "rule_end", true),
            (STATEMENT, "statement", true),
            (STATEMENT_ASSIGN, "statement_assign", true),
            (STATEMENT_BLOCK, "statement_block", true),
            (STATEMENT_EMPTY, "statement_empty", true),
            (STATEMENT_FOR, "statement_for", true),
            (STATEMENT_IF, "statement_if", true),
            (STATEMENT_INSERT, "statement_insert", true),
            (STATEMENT_MATCH, "statement_match", true),
            (STRING_QUOTED, "string_quoted", true),
            (STRING_QUOTED_BRANCH_0, "string_quoted_branch_0", true),
            (STRING_QUOTED_BRANCH_1, "string_quoted_branch_1", true),
            (STRING_QUOTED_ESCAPED, "string_quoted_escaped", true),
            (STRING_QUOTED_ESCAPED_BRANCH_0, "string_quoted_escaped_branch_0", true),
            (STRING_QUOTED_ESCAPED_BRANCH_1, "string_quoted_escaped_branch_1", true),
            (STRING_RAW, "string_raw", true),
            (STRING_RAW_INTERPOLATED, "string_raw_interpolated", true),
            (STRING_RAW_INTERPOLATED_BRANCH_0, "string_raw_interpolated_branch_0", true),
            (STRING_RAW_INTERPOLATED_BRANCH_1, "string_raw_interpolated_branch_1", true),
            (TRANSFORMER, "transformer", true),
            (TYPE, "type", true),
            (TYPE_ATOM, "type_atom", true),
            (TYPE_BIGINT, "type_bigint", true),
            (TYPE_BIT, "type_bit", true),
            (TYPE_BOOL, "type_bool", true),
            (TYPE_DOUBLE, "type_double", true),
            (TYPE_FLOAT, "type_float", true),
            (TYPE_FUN, "type_fun", true),
            (TYPE_FUN_BRANCH_0, "type_fun_branch_0", true),
            (TYPE_FUN_BRANCH_1, "type_fun_branch_1", true),
            (TYPE_SIGNED, "type_signed", true),
            (TYPE_STRING, "type_string", true),
            (TYPE_TRANS, "type_trans", true),
            (TYPE_TRANS_FUN, "type_trans_fun", true),
            (TYPE_TRANS_REL, "type_trans_rel", true),
            (TYPE_TUPLE, "type_tuple", true),
            (TYPE_UNION, "type_union", true),
            (TYPE_USER, "type_user", true),
            (TYPE_VAR, "type_var", true),
            (TYPEDEF, "typedef", true),
            (TYPEDEF_EXTERN, "typedef_extern", true),
            (TYPEDEF_NORMAL, "typedef_normal", true),
            (WORD, "word", true),
        ],
    }
}

#[allow(missing_docs)]
pub mod keyword {
    // keywords
    ddlog_lsp_macros::node_kind_ids! {
        language: "ddlog.dl",
        node_kinds: [
            (AND, "and", false),
            (APPLY, "apply", false),
            (AS, "as", false),
            // (BIGINT, "bigint", false),
            (BIT, "bit", false),
            // (BOOL, "bool", false),
            // (BREAK, "break", false),
            // (CONTINUE, "continue", false),
            // (DOUBLE, "double", false),
            (ELSE, "else", false),
            (EXTERN, "extern", false),
            (FALSE, "false", false),
            (FLAT_MAP, "FlatMap", false),
            // (FLOAT, "float", false),
            (FOR, "for", false),
            (FUNCTION, "function", false),
            (GROUP_BY, "group_by", false),
            (IDENTIFIER, "identifier", false),
            (IF, "if", false),
            (IMPORT, "import", false),
            (IN, "in", false),
            (INDEX, "index", false),
            (INPUT, "input", false),
            (INSPECT, "Inspect", false),
            (INTERNAL, "internal", false),
            (KEY, "key", false),
            (MATCH, "match", false),
            (MULTISET, "multiset", false),
            (MUT, "mut", false),
            (NOT, "not", false),
            (ON, "on", false),
            (OR, "or", false),
            (OUTPUT, "output", false),
            (PRIMARY, "primary", false),
            (RELATION, "relation", false),
            (RETURN, "return", false),
            (SIGNED, "signed", false),
            // (SKIP, "skip", false),
            (STREAM, "stream", false),
            // (STRING, "string", false),
            (TRANSFORMER, "transformer", false),
            (TRUE, "true", false),
            (TYPE, "type", false),
            (TYPEDEF, "typedef", false),
            (VAR, "var", false),
        ]
    }
}

#[allow(missing_docs)]
pub mod symbol {
    // tokens
    ddlog_lsp_macros::node_kind_ids! {
        language: "ddlog.dl",
        node_kinds: [
            (AMPERSAND, "&", false),
            (APOSTROPHE, "'", false),
            (ASTERISK, "*", false),
            (ASTERISK_SOLIDUS, "*/", false),
            (CIRCUMFLEX_ACCENT, "^", false),
            (COLON, ":", false),
            (COLON_COLON, "::", false),
            (COLON_HYPHEN_MINUS, ":-", false),
            (COMMA, ",", false),
            (COMMERCIAL_AT, "@", false),
            (DOLLAR_SIGN, "$", false),
            (DOLLAR_SIGN_LEFT_CURLY_BRACKET, "${", false),
            (EQUALS_SIGN, "=", false),
            (EQUALS_SIGN_EQUALS_SIGN, "==", false),
            (EQUALS_SIGN_GREATER_THAN_SIGN, "=>", false),
            (EXCLAMATION_MARK_EQUALS_SIGN, "!=", false),
            (FULL_STOP, ".", false),
            (GREATER_THAN_SIGN, ">", false),
            (GREATER_THAN_SIGN_EQUALS_SIGN, ">=", false),
            (GREATER_THAN_SIGN_GREATER_THAN_SIGN, ">>", false),
            (HYPHEN_MINUS, "-", false),
            (LEFT_CURLY_BRACKET, "{", false),
            (LEFT_PARENTHESIS, "(", false),
            (LEFT_SQUARE_BRACKET, "[", false),
            (LESS_THAN_SIGN, "<", false),
            (LESS_THAN_SIGN_EQUALS_SIGN, "<=", false),
            (LESS_THAN_SIGN_LESS_THAN_SIGN, "<<", false),
            (LIT_BIN, "'b", false),
            (LIT_DEC, "'d", false),
            (LIT_FLOAT, "'f", false),
            (LIT_HEX, "'h", false),
            (LIT_OCT, "'o", false),
            (LIT_S_BIN, "'sb", false),
            (LIT_S_DEC, "'sd", false),
            (LIT_S_HEX, "'sh", false),
            (LIT_S_OCT, "'so", false),
            // (LOW_LINE, "_", false),
            (NUMBER_SIGN, "#", false),
            (NUMBER_SIGN_LEFT_SQUARE_BRACKET, "#[", false),
            (PERCENT_SIGN, "%", false),
            (PLUS_SIGN, "+", false),
            (PLUS_SIGN_PLUS_SIGN, "++", false),
            (QUESTION_MARK, "?", false),
            (QUOTATION_MARK, "\"", false),
            (REVERSE_SOLIDUS_REVERSE_SOLIDUS, "\\", false),
            (RIGHT_CURLY_BRACKET, "}", false),
            (RIGHT_PARENTHESIS, ")", false),
            (RIGHT_SQUARE_BRACKET, "]", false),
            (RIGHTWARDS_ARROW, "->", false),
            (SEMICOLON, ";", false),
            (SOLIDUS, "/", false),
            (SOLIDUS_ASTERISK, "/*", false),
            (SOLIDUS_SOLIDUS, "//", false),
            (TILDE, "~", false),
            (VERTICAL_LINE, "|", false),
            (VERTICAL_LINE_RIGHT_SQUARE_BRACKET, "|]", false),
        ]
    }
}

#[allow(missing_docs)]
pub trait Visitor<'tree>: HasWalker<'tree> {
    fn visit(&mut self) -> Result<(), SyntaxError<()>> {
        let walker = self.walker();
        loop {
            match walker.node().kind_id() {
                kind::ROOT => {
                    self.visit_ROOT(NodeMove::Init)?;
                    break;
                },
                kind::ANNOTATED_ITEM => {
                    self.visit_annotated_item(NodeMove::Init)?;
                    break;
                },
                kind::APPLY => {
                    self.visit_apply(NodeMove::Init)?;
                    break;
                },
                kind::ARG => {
                    self.visit_arg(NodeMove::Init)?;
                    break;
                },
                kind::ARG_OPT_TYPE => {
                    self.visit_arg_opt_type(NodeMove::Init)?;
                    break;
                },
                kind::ARG_TRANS => {
                    self.visit_arg_trans(NodeMove::Init)?;
                    break;
                },
                kind::ATOM => {
                    self.visit_atom(NodeMove::Init)?;
                    break;
                },
                kind::ATOM_ELEM => {
                    self.visit_atom_elem(NodeMove::Init)?;
                    break;
                },
                kind::ATOM_POS => {
                    self.visit_atom_pos(NodeMove::Init)?;
                    break;
                },
                kind::ATOM_REC => {
                    self.visit_atom_rec(NodeMove::Init)?;
                    break;
                },
                kind::ATTRIBUTE => {
                    self.visit_attribute(NodeMove::Init)?;
                    break;
                },
                kind::ATTRIBUTES => {
                    self.visit_attributes(NodeMove::Init)?;
                    break;
                },
                kind::COMMENT_BLOCK => {
                    self.visit_comment_block(NodeMove::Init)?;
                    break;
                },
                kind::COMMENT_BLOCK_INNER => {
                    self.visit_comment_block_inner(NodeMove::Init)?;
                    break;
                },
                kind::COMMENT_LINE => {
                    self.visit_comment_line(NodeMove::Init)?;
                    break;
                },
                kind::CONS => {
                    self.visit_cons(NodeMove::Init)?;
                    break;
                },
                kind::CONS_POS => {
                    self.visit_cons_pos(NodeMove::Init)?;
                    break;
                },
                kind::CONS_REC => {
                    self.visit_cons_rec(NodeMove::Init)?;
                    break;
                },
                kind::ESCAPE_SEQUENCE => {
                    self.visit_escape_sequence(NodeMove::Init)?;
                    break;
                },
                kind::ESCAPE_SEQUENCE_INTERPOLATED => {
                    self.visit_escape_sequence_interpolated(NodeMove::Init)?;
                    break;
                },
                kind::EXP => {
                    self.visit_exp(NodeMove::Init)?;
                    break;
                },
                kind::EXP_ADD => {
                    self.visit_exp_add(NodeMove::Init)?;
                    break;
                },
                kind::EXP_ASSIGN => {
                    self.visit_exp_assign(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BINDING => {
                    self.visit_exp_binding(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BIT_AND => {
                    self.visit_exp_bit_and(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BIT_NEG => {
                    self.visit_exp_bit_neg(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BIT_OR => {
                    self.visit_exp_bit_or(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BIT_SLICE => {
                    self.visit_exp_bit_slice(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BIT_XOR => {
                    self.visit_exp_bit_xor(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BLOCK => {
                    self.visit_exp_block(NodeMove::Init)?;
                    break;
                },
                kind::EXP_BREAK => {
                    self.visit_exp_break(NodeMove::Init)?;
                    break;
                },
                kind::EXP_CAST => {
                    self.visit_exp_cast(NodeMove::Init)?;
                    break;
                },
                kind::EXP_CAT => {
                    self.visit_exp_cat(NodeMove::Init)?;
                    break;
                },
                kind::EXP_COND => {
                    self.visit_exp_cond(NodeMove::Init)?;
                    break;
                },
                kind::EXP_CONS_POS => {
                    self.visit_exp_cons_pos(NodeMove::Init)?;
                    break;
                },
                kind::EXP_CONS_REC => {
                    self.visit_exp_cons_rec(NodeMove::Init)?;
                    break;
                },
                kind::EXP_CONTINUE => {
                    self.visit_exp_continue(NodeMove::Init)?;
                    break;
                },
                kind::EXP_DECL_VAR => {
                    self.visit_exp_decl_var(NodeMove::Init)?;
                    break;
                },
                kind::EXP_DIV => {
                    self.visit_exp_div(NodeMove::Init)?;
                    break;
                },
                kind::EXP_EQ => {
                    self.visit_exp_eq(NodeMove::Init)?;
                    break;
                },
                kind::EXP_FIELD => {
                    self.visit_exp_field(NodeMove::Init)?;
                    break;
                },
                kind::EXP_FOR => {
                    self.visit_exp_for(NodeMove::Init)?;
                    break;
                },
                kind::EXP_FUN_CALL => {
                    self.visit_exp_fun_call(NodeMove::Init)?;
                    break;
                },
                kind::EXP_FUN_CALL_DOT => {
                    self.visit_exp_fun_call_dot(NodeMove::Init)?;
                    break;
                },
                kind::EXP_GT => {
                    self.visit_exp_gt(NodeMove::Init)?;
                    break;
                },
                kind::EXP_GTEQ => {
                    self.visit_exp_gteq(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LAMBDA => {
                    self.visit_exp_lambda(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LAMBDA_BRANCH_0 => {
                    self.visit_exp_lambda_branch_0(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LAMBDA_BRANCH_1 => {
                    self.visit_exp_lambda_branch_1(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LIT => {
                    self.visit_exp_lit(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LOG_AND => {
                    self.visit_exp_log_and(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LOG_IMP => {
                    self.visit_exp_log_imp(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LOG_NEG => {
                    self.visit_exp_log_neg(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LOG_OR => {
                    self.visit_exp_log_or(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LT => {
                    self.visit_exp_lt(NodeMove::Init)?;
                    break;
                },
                kind::EXP_LTEQ => {
                    self.visit_exp_lteq(NodeMove::Init)?;
                    break;
                },
                kind::EXP_MATCH => {
                    self.visit_exp_match(NodeMove::Init)?;
                    break;
                },
                kind::EXP_MUL => {
                    self.visit_exp_mul(NodeMove::Init)?;
                    break;
                },
                kind::EXP_NEG => {
                    self.visit_exp_neg(NodeMove::Init)?;
                    break;
                },
                kind::EXP_NEQ => {
                    self.visit_exp_neq(NodeMove::Init)?;
                    break;
                },
                kind::EXP_PROJ => {
                    self.visit_exp_proj(NodeMove::Init)?;
                    break;
                },
                kind::EXP_PROJ_DIGITS => {
                    self.visit_exp_proj_digits(NodeMove::Init)?;
                    break;
                },
                kind::EXP_REF => {
                    self.visit_exp_ref(NodeMove::Init)?;
                    break;
                },
                kind::EXP_REM => {
                    self.visit_exp_rem(NodeMove::Init)?;
                    break;
                },
                kind::EXP_RETURN => {
                    self.visit_exp_return(NodeMove::Init)?;
                    break;
                },
                kind::EXP_SEQ => {
                    self.visit_exp_seq(NodeMove::Init)?;
                    break;
                },
                kind::EXP_SHL => {
                    self.visit_exp_shl(NodeMove::Init)?;
                    break;
                },
                kind::EXP_SHR => {
                    self.visit_exp_shr(NodeMove::Init)?;
                    break;
                },
                kind::EXP_SLICE => {
                    self.visit_exp_slice(NodeMove::Init)?;
                    break;
                },
                kind::EXP_SUB => {
                    self.visit_exp_sub(NodeMove::Init)?;
                    break;
                },
                kind::EXP_TRY => {
                    self.visit_exp_try(NodeMove::Init)?;
                    break;
                },
                kind::EXP_TUPLE => {
                    self.visit_exp_tuple(NodeMove::Init)?;
                    break;
                },
                kind::EXP_TYPE => {
                    self.visit_exp_type(NodeMove::Init)?;
                    break;
                },
                kind::EXP_WILD => {
                    self.visit_exp_wild(NodeMove::Init)?;
                    break;
                },
                kind::FIELD => {
                    self.visit_field(NodeMove::Init)?;
                    break;
                },
                kind::FUNCTION => {
                    self.visit_function(NodeMove::Init)?;
                    break;
                },
                kind::FUNCTION_EXTERN => {
                    self.visit_function_extern(NodeMove::Init)?;
                    break;
                },
                kind::FUNCTION_NORMAL => {
                    self.visit_function_normal(NodeMove::Init)?;
                    break;
                },
                kind::FUNCTION_NORMAL_BRANCH_0 => {
                    self.visit_function_normal_branch_0(NodeMove::Init)?;
                    break;
                },
                kind::FUNCTION_NORMAL_BRANCH_1 => {
                    self.visit_function_normal_branch_1(NodeMove::Init)?;
                    break;
                },
                kind::IDENT => {
                    self.visit_ident(NodeMove::Init)?;
                    break;
                },
                kind::IDENT_LOWER => {
                    self.visit_ident_lower(NodeMove::Init)?;
                    break;
                },
                kind::IDENT_LOWER_SCOPED => {
                    self.visit_ident_lower_scoped(NodeMove::Init)?;
                    break;
                },
                kind::IDENT_SCOPED => {
                    self.visit_ident_scoped(NodeMove::Init)?;
                    break;
                },
                kind::IDENT_UPPER => {
                    self.visit_ident_upper(NodeMove::Init)?;
                    break;
                },
                kind::IDENT_UPPER_SCOPED => {
                    self.visit_ident_upper_scoped(NodeMove::Init)?;
                    break;
                },
                kind::IMPORT => {
                    self.visit_import(NodeMove::Init)?;
                    break;
                },
                kind::INDEX => {
                    self.visit_index(NodeMove::Init)?;
                    break;
                },
                kind::INTERPOLATION => {
                    self.visit_interpolation(NodeMove::Init)?;
                    break;
                },
                kind::ITEM => {
                    self.visit_item(NodeMove::Init)?;
                    break;
                },
                kind::KEY_PRIMARY => {
                    self.visit_key_primary(NodeMove::Init)?;
                    break;
                },
                kind::LIT_BOOL => {
                    self.visit_lit_bool(NodeMove::Init)?;
                    break;
                },
                kind::LIT_MAP => {
                    self.visit_lit_map(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM => {
                    self.visit_lit_num(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_0 => {
                    self.visit_lit_num_branch_0(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_1 => {
                    self.visit_lit_num_branch_1(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_10 => {
                    self.visit_lit_num_branch_10(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_11 => {
                    self.visit_lit_num_branch_11(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_12 => {
                    self.visit_lit_num_branch_12(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_13 => {
                    self.visit_lit_num_branch_13(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_14 => {
                    self.visit_lit_num_branch_14(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_15 => {
                    self.visit_lit_num_branch_15(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_16 => {
                    self.visit_lit_num_branch_16(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_17 => {
                    self.visit_lit_num_branch_17(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_18 => {
                    self.visit_lit_num_branch_18(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BIN => {
                    self.visit_lit_num_bin(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_DEC => {
                    self.visit_lit_num_dec(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_FLOAT => {
                    self.visit_lit_num_float(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_HEX => {
                    self.visit_lit_num_hex(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_OCT => {
                    self.visit_lit_num_oct(NodeMove::Init)?;
                    break;
                },
                kind::LIT_STRING => {
                    self.visit_lit_string(NodeMove::Init)?;
                    break;
                },
                kind::LIT_VEC => {
                    self.visit_lit_vec(NodeMove::Init)?;
                    break;
                },
                kind::MISC_PAT0 => {
                    self.visit_misc_pat0(NodeMove::Init)?;
                    break;
                },
                kind::MODULE_ALIAS => {
                    self.visit_module_alias(NodeMove::Init)?;
                    break;
                },
                kind::MODULE_PATH => {
                    self.visit_module_path(NodeMove::Init)?;
                    break;
                },
                kind::NAME => {
                    self.visit_name(NodeMove::Init)?;
                    break;
                },
                kind::NAME_ARG => {
                    self.visit_name_arg(NodeMove::Init)?;
                    break;
                },
                kind::NAME_CONS => {
                    self.visit_name_cons(NodeMove::Init)?;
                    break;
                },
                kind::NAME_FIELD => {
                    self.visit_name_field(NodeMove::Init)?;
                    break;
                },
                kind::NAME_FUNC => {
                    self.visit_name_func(NodeMove::Init)?;
                    break;
                },
                kind::NAME_INDEX => {
                    self.visit_name_index(NodeMove::Init)?;
                    break;
                },
                kind::NAME_REL => {
                    self.visit_name_rel(NodeMove::Init)?;
                    break;
                },
                kind::NAME_TRANS => {
                    self.visit_name_trans(NodeMove::Init)?;
                    break;
                },
                kind::NAME_TYPE => {
                    self.visit_name_type(NodeMove::Init)?;
                    break;
                },
                kind::NAME_VAR_TERM => {
                    self.visit_name_var_term(NodeMove::Init)?;
                    break;
                },
                kind::NAME_VAR_TYPE => {
                    self.visit_name_var_type(NodeMove::Init)?;
                    break;
                },
                kind::PAT => {
                    self.visit_pat(NodeMove::Init)?;
                    break;
                },
                kind::PAT_CONS => {
                    self.visit_pat_cons(NodeMove::Init)?;
                    break;
                },
                kind::PAT_CONS_POS => {
                    self.visit_pat_cons_pos(NodeMove::Init)?;
                    break;
                },
                kind::PAT_CONS_REC => {
                    self.visit_pat_cons_rec(NodeMove::Init)?;
                    break;
                },
                kind::PAT_LIT => {
                    self.visit_pat_lit(NodeMove::Init)?;
                    break;
                },
                kind::PAT_TERM_DECL_VAR => {
                    self.visit_pat_term_decl_var(NodeMove::Init)?;
                    break;
                },
                kind::PAT_TUPLE => {
                    self.visit_pat_tuple(NodeMove::Init)?;
                    break;
                },
                kind::PAT_TYPE => {
                    self.visit_pat_type(NodeMove::Init)?;
                    break;
                },
                kind::PAT_WILD => {
                    self.visit_pat_wild(NodeMove::Init)?;
                    break;
                },
                kind::REL => {
                    self.visit_rel(NodeMove::Init)?;
                    break;
                },
                kind::REL_ARGS => {
                    self.visit_rel_args(NodeMove::Init)?;
                    break;
                },
                kind::REL_ELEM => {
                    self.visit_rel_elem(NodeMove::Init)?;
                    break;
                },
                kind::REL_ROLE => {
                    self.visit_rel_role(NodeMove::Init)?;
                    break;
                },
                kind::REL_SEMANTICS => {
                    self.visit_rel_semantics(NodeMove::Init)?;
                    break;
                },
                kind::RHS => {
                    self.visit_rhs(NodeMove::Init)?;
                    break;
                },
                kind::RHS_ATOM_NEG => {
                    self.visit_rhs_atom_neg(NodeMove::Init)?;
                    break;
                },
                kind::RHS_FLAT_MAP => {
                    self.visit_rhs_flat_map(NodeMove::Init)?;
                    break;
                },
                kind::RHS_GROUPING => {
                    self.visit_rhs_grouping(NodeMove::Init)?;
                    break;
                },
                kind::RHS_INSPECT => {
                    self.visit_rhs_inspect(NodeMove::Init)?;
                    break;
                },
                kind::RULE => {
                    self.visit_rule(NodeMove::Init)?;
                    break;
                },
                kind::RULE_END => {
                    self.visit_rule_end(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT => {
                    self.visit_statement(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT_ASSIGN => {
                    self.visit_statement_assign(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT_BLOCK => {
                    self.visit_statement_block(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT_EMPTY => {
                    self.visit_statement_empty(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT_FOR => {
                    self.visit_statement_for(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT_IF => {
                    self.visit_statement_if(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT_INSERT => {
                    self.visit_statement_insert(NodeMove::Init)?;
                    break;
                },
                kind::STATEMENT_MATCH => {
                    self.visit_statement_match(NodeMove::Init)?;
                    break;
                },
                kind::STRING_QUOTED => {
                    self.visit_string_quoted(NodeMove::Init)?;
                    break;
                },
                kind::STRING_QUOTED_BRANCH_0 => {
                    self.visit_string_quoted_branch_0(NodeMove::Init)?;
                    break;
                },
                kind::STRING_QUOTED_BRANCH_1 => {
                    self.visit_string_quoted_branch_1(NodeMove::Init)?;
                    break;
                },
                kind::STRING_QUOTED_ESCAPED => {
                    self.visit_string_quoted_escaped(NodeMove::Init)?;
                    break;
                },
                kind::STRING_QUOTED_ESCAPED_BRANCH_0 => {
                    self.visit_string_quoted_escaped_branch_0(NodeMove::Init)?;
                    break;
                },
                kind::STRING_QUOTED_ESCAPED_BRANCH_1 => {
                    self.visit_string_quoted_escaped_branch_1(NodeMove::Init)?;
                    break;
                },
                kind::STRING_RAW => {
                    self.visit_string_raw(NodeMove::Init)?;
                    break;
                },
                kind::STRING_RAW_INTERPOLATED => {
                    self.visit_string_raw_interpolated(NodeMove::Init)?;
                    break;
                },
                kind::TRANSFORMER => {
                    self.visit_transformer(NodeMove::Init)?;
                    break;
                },
                kind::TYPE => {
                    self.visit_type(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_ATOM => {
                    self.visit_type_atom(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_BIGINT => {
                    self.visit_type_bigint(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_BIT => {
                    self.visit_type_bit(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_BOOL => {
                    self.visit_type_bool(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_DOUBLE => {
                    self.visit_type_double(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_FLOAT => {
                    self.visit_type_float(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_FUN => {
                    self.visit_type_fun(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_FUN_BRANCH_0 => {
                    self.visit_type_fun_branch_0(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_FUN_BRANCH_1 => {
                    self.visit_type_fun_branch_1(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_SIGNED => {
                    self.visit_type_signed(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_STRING => {
                    self.visit_type_string(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_TRANS => {
                    self.visit_type_trans(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_TRANS_FUN => {
                    self.visit_type_trans_fun(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_TRANS_REL => {
                    self.visit_type_trans_rel(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_TUPLE => {
                    self.visit_type_tuple(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_UNION => {
                    self.visit_type_union(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_USER => {
                    self.visit_type_user(NodeMove::Init)?;
                    break;
                },
                kind::TYPE_VAR => {
                    self.visit_type_var(NodeMove::Init)?;
                    break;
                },
                kind::TYPEDEF => {
                    self.visit_typedef(NodeMove::Init)?;
                    break;
                },
                kind::TYPEDEF_EXTERN => {
                    self.visit_typedef_extern(NodeMove::Init)?;
                    break;
                },
                kind::TYPEDEF_NORMAL => {
                    self.visit_typedef_normal(NodeMove::Init)?;
                    break;
                },
                kind::WORD => {
                    self.visit_word(NodeMove::Init)?;
                    break;
                },
                _ => {
                    if !walker.goto_next(GotoNext::StepInto, true) {
                        break;
                    }
                },
            }
        }
        Ok(())
    }

    #[allow(non_snake_case)]
    fn visit_ROOT(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ROOT");
        visit::ROOT(self, node_move)
    }

    fn visit_annotated_item(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::annotated_item");
        visit::annotated_item(self, node_move)
    }

    fn visit_apply(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::apply");
        visit::apply(self, node_move)
    }

    fn visit_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::arg");
        visit::arg(self, node_move)
    }

    fn visit_arg_opt_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::arg_opt_type");
        visit::arg_opt_type(self, node_move)
    }

    fn visit_arg_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::arg_trans");
        visit::arg_trans(self, node_move)
    }

    fn visit_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom");
        visit::atom(self, node_move)
    }

    fn visit_atom_elem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom_elem");
        visit::atom_elem(self, node_move)
    }

    fn visit_atom_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom_pos");
        visit::atom_pos(self, node_move)
    }

    fn visit_atom_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom_rec");
        visit::atom_rec(self, node_move)
    }

    fn visit_attribute(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::attribute");
        visit::attribute(self, node_move)
    }

    fn visit_attributes(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::attributes");
        visit::attributes(self, node_move)
    }

    fn visit_comment_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::comment_block");
        visit::comment_block(self, node_move)
    }

    fn visit_comment_block_inner(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::comment_block_inner");
        visit::comment_block_inner(self, node_move)
    }

    fn visit_comment_line(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::comment_line");
        visit::comment_line(self, node_move)
    }

    fn visit_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::cons");
        visit::cons(self, node_move)
    }

    fn visit_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::cons_pos");
        visit::cons_pos(self, node_move)
    }

    fn visit_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::cons_rec");
        visit::cons_rec(self, node_move)
    }

    fn visit_escape_sequence(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::escape_sequence");
        visit::escape_sequence(self, node_move)
    }

    fn visit_escape_sequence_interpolated(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::escape_sequence_interpolated");
        visit::escape_sequence_interpolated(self, node_move)
    }

    fn visit_exp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp");
        visit::exp(self, node_move)
    }

    fn visit_exp_add(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_add");
        visit::exp_add(self, node_move)
    }

    fn visit_exp_assign(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_assign");
        visit::exp_assign(self, node_move)
    }

    fn visit_exp_binding(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_binding");
        visit::exp_binding(self, node_move)
    }

    fn visit_exp_bit_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_and");
        visit::exp_bit_and(self, node_move)
    }

    fn visit_exp_bit_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_neg");
        visit::exp_bit_neg(self, node_move)
    }

    fn visit_exp_bit_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_or");
        visit::exp_bit_or(self, node_move)
    }

    fn visit_exp_bit_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_slice");
        visit::exp_bit_slice(self, node_move)
    }

    fn visit_exp_bit_xor(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_xor");
        visit::exp_bit_xor(self, node_move)
    }

    fn visit_exp_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_block");
        visit::exp_block(self, node_move)
    }

    fn visit_exp_break(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_break");
        visit::exp_break(self, node_move)
    }

    fn visit_exp_cast(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cast");
        visit::exp_cast(self, node_move)
    }

    fn visit_exp_cat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cat");
        visit::exp_cat(self, node_move)
    }

    fn visit_exp_cond(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cond");
        visit::exp_cond(self, node_move)
    }

    fn visit_exp_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cons_pos");
        visit::exp_cons_pos(self, node_move)
    }

    fn visit_exp_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cons_rec");
        visit::exp_cons_rec(self, node_move)
    }

    fn visit_exp_continue(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_continue");
        visit::exp_continue(self, node_move)
    }

    fn visit_exp_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_decl_var");
        visit::exp_decl_var(self, node_move)
    }

    fn visit_exp_div(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_div");
        visit::exp_div(self, node_move)
    }

    fn visit_exp_eq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_eq");
        visit::exp_eq(self, node_move)
    }

    fn visit_exp_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_field");
        visit::exp_field(self, node_move)
    }

    fn visit_exp_for(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_for");
        visit::exp_for(self, node_move)
    }

    fn visit_exp_fun_call(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_fun_call");
        visit::exp_fun_call(self, node_move)
    }

    fn visit_exp_fun_call_dot(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_fun_call_dot");
        visit::exp_fun_call_dot(self, node_move)
    }

    fn visit_exp_gt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_gt");
        visit::exp_gt(self, node_move)
    }

    fn visit_exp_gteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_gteq");
        visit::exp_gteq(self, node_move)
    }

    fn visit_exp_lambda(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lambda");
        visit::exp_lambda(self, node_move)
    }

    fn visit_exp_lambda_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lambda_branch_0");
        visit::exp_lambda_branch_0(self, node_move)
    }

    fn visit_exp_lambda_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lambda_branch_1");
        visit::exp_lambda_branch_1(self, node_move)
    }

    fn visit_exp_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lit");
        visit::exp_lit(self, node_move)
    }

    fn visit_exp_log_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_and");
        visit::exp_log_and(self, node_move)
    }

    fn visit_exp_log_imp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_imp");
        visit::exp_log_imp(self, node_move)
    }

    fn visit_exp_log_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_neg");
        visit::exp_log_neg(self, node_move)
    }

    fn visit_exp_log_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_or");
        visit::exp_log_or(self, node_move)
    }

    fn visit_exp_lt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lt");
        visit::exp_lt(self, node_move)
    }

    fn visit_exp_lteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lteq");
        visit::exp_lteq(self, node_move)
    }

    fn visit_exp_match(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_match");
        visit::exp_match(self, node_move)
    }

    fn visit_exp_mul(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_mul");
        visit::exp_mul(self, node_move)
    }

    fn visit_exp_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_neg");
        visit::exp_neg(self, node_move)
    }

    fn visit_exp_neq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_neq");
        visit::exp_neq(self, node_move)
    }

    fn visit_exp_proj(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_proj");
        visit::exp_proj(self, node_move)
    }

    fn visit_exp_proj_digits(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_proj_digits");
        visit::exp_proj_digits(self, node_move)
    }

    fn visit_exp_ref(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_ref");
        visit::exp_ref(self, node_move)
    }

    fn visit_exp_rem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_rem");
        visit::exp_rem(self, node_move)
    }

    fn visit_exp_return(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_return");
        visit::exp_return(self, node_move)
    }

    fn visit_exp_seq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_seq");
        visit::exp_seq(self, node_move)
    }

    fn visit_exp_shl(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_shl");
        visit::exp_shl(self, node_move)
    }

    fn visit_exp_shr(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_shr");
        visit::exp_shr(self, node_move)
    }

    fn visit_exp_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_slice");
        visit::exp_slice(self, node_move)
    }

    fn visit_exp_sub(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_sub");
        visit::exp_sub(self, node_move)
    }

    fn visit_exp_try(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_try");
        visit::exp_try(self, node_move)
    }

    fn visit_exp_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_tuple");
        visit::exp_tuple(self, node_move)
    }

    fn visit_exp_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_type");
        visit::exp_type(self, node_move)
    }

    fn visit_exp_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_wild");
        visit::exp_wild(self, node_move)
    }

    fn visit_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::field");
        visit::field(self, node_move)
    }

    fn visit_function(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function");
        visit::function(self, node_move)
    }

    fn visit_function_extern(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_extern");
        visit::function_extern(self, node_move)
    }

    fn visit_function_normal(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_normal");
        visit::function_normal(self, node_move)
    }

    fn visit_function_normal_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_normal_branch_0");
        visit::function_normal_branch_0(self, node_move)
    }

    fn visit_function_normal_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_normal_branch_1");
        visit::function_normal_branch_1(self, node_move)
    }

    fn visit_ident(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident");
        visit::ident(self, node_move)
    }

    fn visit_ident_lower(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_lower");
        visit::ident_lower(self, node_move)
    }

    fn visit_ident_lower_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_lower_scoped");
        visit::ident_lower_scoped(self, node_move)
    }

    fn visit_ident_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_scoped");
        visit::ident_scoped(self, node_move)
    }

    fn visit_ident_upper(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_upper");
        visit::ident_upper(self, node_move)
    }

    fn visit_ident_upper_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_upper_scoped");
        visit::ident_upper_scoped(self, node_move)
    }

    fn visit_import(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::import");
        visit::import(self, node_move)
    }

    fn visit_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::index");
        visit::index(self, node_move)
    }

    fn visit_interpolation(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::interpolation");
        visit::interpolation(self, node_move)
    }

    fn visit_item(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::item");
        visit::item(self, node_move)
    }

    fn visit_key_primary(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::key_primary");
        visit::key_primary(self, node_move)
    }

    fn visit_lit_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_bool");
        visit::lit_bool(self, node_move)
    }

    fn visit_lit_map(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_map");
        visit::lit_map(self, node_move)
    }

    fn visit_lit_num(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num");
        visit::lit_num(self, node_move)
    }

    fn visit_lit_num_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_0");
        visit::lit_num_branch_0(self, node_move)
    }

    fn visit_lit_num_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_1");
        visit::lit_num_branch_1(self, node_move)
    }

    fn visit_lit_num_branch_10(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_10");
        visit::lit_num_branch_10(self, node_move)
    }

    fn visit_lit_num_branch_11(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_11");
        visit::lit_num_branch_11(self, node_move)
    }

    fn visit_lit_num_branch_12(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_12");
        visit::lit_num_branch_12(self, node_move)
    }

    fn visit_lit_num_branch_13(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_13");
        visit::lit_num_branch_13(self, node_move)
    }

    fn visit_lit_num_branch_14(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_14");
        visit::lit_num_branch_14(self, node_move)
    }

    fn visit_lit_num_branch_15(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_15");
        visit::lit_num_branch_15(self, node_move)
    }

    fn visit_lit_num_branch_16(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_16");
        visit::lit_num_branch_16(self, node_move)
    }

    fn visit_lit_num_branch_17(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_17");
        visit::lit_num_branch_17(self, node_move)
    }

    fn visit_lit_num_branch_18(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_18");
        visit::lit_num_branch_18(self, node_move)
    }

    fn visit_lit_num_bin(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_bin");
        visit::lit_num_bin(self, node_move)
    }

    fn visit_lit_num_dec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_dec");
        visit::lit_num_dec(self, node_move)
    }

    fn visit_lit_num_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_float");
        visit::lit_num_float(self, node_move)
    }

    fn visit_lit_num_hex(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_hex");
        visit::lit_num_hex(self, node_move)
    }

    fn visit_lit_num_oct(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_oct");
        visit::lit_num_oct(self, node_move)
    }

    fn visit_lit_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_string");
        visit::lit_string(self, node_move)
    }

    fn visit_lit_vec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_vec");
        visit::lit_vec(self, node_move)
    }

    fn visit_misc_pat0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::misc_pat0");
        visit::misc_pat0(self, node_move)
    }

    fn visit_module_alias(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::module_alias");
        visit::module_alias(self, node_move)
    }

    fn visit_module_path(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::module_path");
        visit::module_path(self, node_move)
    }

    fn visit_name(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name");
        visit::name(self, node_move)
    }

    fn visit_name_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_arg");
        visit::name_arg(self, node_move)
    }

    fn visit_name_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_cons");
        visit::name_cons(self, node_move)
    }

    fn visit_name_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_field");
        visit::name_field(self, node_move)
    }

    fn visit_name_func(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_func");
        visit::name_func(self, node_move)
    }

    fn visit_name_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_index");
        visit::name_index(self, node_move)
    }

    fn visit_name_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_rel");
        visit::name_rel(self, node_move)
    }

    fn visit_name_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_trans");
        visit::name_trans(self, node_move)
    }

    fn visit_name_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_type");
        visit::name_type(self, node_move)
    }

    fn visit_name_var_term(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_var_term");
        visit::name_var_term(self, node_move)
    }

    fn visit_name_var_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_var_type");
        visit::name_var_type(self, node_move)
    }

    fn visit_pat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat");
        visit::pat(self, node_move)
    }

    fn visit_pat_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_cons");
        visit::pat_cons(self, node_move)
    }

    fn visit_pat_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_cons_pos");
        visit::pat_cons_pos(self, node_move)
    }

    fn visit_pat_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_cons_rec");
        visit::pat_cons_rec(self, node_move)
    }

    fn visit_pat_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_lit");
        visit::pat_lit(self, node_move)
    }

    fn visit_pat_term_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_term_decl_var");
        visit::pat_term_decl_var(self, node_move)
    }

    fn visit_pat_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_tuple");
        visit::pat_tuple(self, node_move)
    }

    fn visit_pat_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_type");
        visit::pat_type(self, node_move)
    }

    fn visit_pat_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_wild");
        visit::pat_wild(self, node_move)
    }

    fn visit_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel");
        visit::rel(self, node_move)
    }

    fn visit_rel_args(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_args");
        visit::rel_args(self, node_move)
    }

    fn visit_rel_elem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_elem");
        visit::rel_elem(self, node_move)
    }

    fn visit_rel_role(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_role");
        visit::rel_role(self, node_move)
    }

    fn visit_rel_semantics(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_semantics");
        visit::rel_semantics(self, node_move)
    }

    fn visit_rhs(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs");
        visit::rhs(self, node_move)
    }

    fn visit_rhs_atom_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_atom_neg");
        visit::rhs_atom_neg(self, node_move)
    }

    fn visit_rhs_flat_map(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_flat_map");
        visit::rhs_flat_map(self, node_move)
    }

    fn visit_rhs_grouping(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_grouping");
        visit::rhs_grouping(self, node_move)
    }

    fn visit_rhs_inspect(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_inspect");
        visit::rhs_inspect(self, node_move)
    }

    fn visit_rule(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rule");
        visit::rule(self, node_move)
    }

    fn visit_rule_end(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rule_end");
        visit::rule_end(self, node_move)
    }

    fn visit_statement(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement");
        visit::statement(self, node_move)
    }

    fn visit_statement_assign(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_assign");
        visit::statement_assign(self, node_move)
    }

    fn visit_statement_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_block");
        visit::statement_block(self, node_move)
    }

    fn visit_statement_empty(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_empty");
        visit::statement_empty(self, node_move)
    }

    fn visit_statement_for(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_for");
        visit::statement_for(self, node_move)
    }

    fn visit_statement_if(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_if");
        visit::statement_if(self, node_move)
    }

    fn visit_statement_insert(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_insert");
        visit::statement_insert(self, node_move)
    }

    fn visit_statement_match(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_match");
        visit::statement_match(self, node_move)
    }

    fn visit_string_quoted(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted");
        visit::string_quoted(self, node_move)
    }

    fn visit_string_quoted_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_branch_0");
        visit::string_quoted_branch_0(self, node_move)
    }

    fn visit_string_quoted_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_branch_0");
        visit::string_quoted_branch_1(self, node_move)
    }

    fn visit_string_quoted_escaped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_escaped");
        visit::string_quoted_escaped(self, node_move)
    }

    fn visit_string_quoted_escaped_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_escaped_branch_0");
        visit::string_quoted_escaped_branch_0(self, node_move)
    }

    fn visit_string_quoted_escaped_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_escaped_branch_1");
        visit::string_quoted_escaped_branch_1(self, node_move)
    }

    fn visit_string_raw(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw");
        visit::string_raw(self, node_move)
    }

    fn visit_string_raw_interpolated(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw_interpolated");
        visit::string_raw_interpolated(self, node_move)
    }

    fn visit_string_raw_interpolated_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw_interpolated_branch_0");
        visit::string_raw_interpolated_branch_0(self, node_move)
    }

    fn visit_string_raw_interpolated_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw_interpolated_branch_1");
        visit::string_raw_interpolated_branch_1(self, node_move)
    }

    fn visit_transformer(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::transformer");
        visit::transformer(self, node_move)
    }

    fn visit_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type");
        visit::r#type(self, node_move)
    }

    fn visit_type_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_atom");
        visit::type_atom(self, node_move)
    }

    fn visit_type_bigint(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_bigint");
        visit::type_bigint(self, node_move)
    }

    fn visit_type_bit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_bit");
        visit::type_bit(self, node_move)
    }

    fn visit_type_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_bool");
        visit::type_bool(self, node_move)
    }

    fn visit_type_double(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_double");
        visit::type_double(self, node_move)
    }

    fn visit_type_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_float");
        visit::type_float(self, node_move)
    }

    fn visit_type_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_fun");
        visit::type_fun(self, node_move)
    }

    fn visit_type_fun_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_fun_branch_0");
        visit::type_fun_branch_0(self, node_move)
    }

    fn visit_type_fun_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_fun_branch_1");
        visit::type_fun_branch_1(self, node_move)
    }

    fn visit_type_signed(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_signed");
        visit::type_signed(self, node_move)
    }

    fn visit_type_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_string");
        visit::type_string(self, node_move)
    }

    fn visit_type_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_trans");
        visit::type_trans(self, node_move)
    }

    fn visit_type_trans_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_trans_fun");
        visit::type_trans_fun(self, node_move)
    }

    fn visit_type_trans_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_trans_rel");
        visit::type_trans_rel(self, node_move)
    }

    fn visit_type_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_tuple");
        visit::type_tuple(self, node_move)
    }

    fn visit_type_union(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_union");
        visit::type_union(self, node_move)
    }

    fn visit_type_user(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_user");
        visit::type_user(self, node_move)
    }

    fn visit_type_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_var");
        visit::type_var(self, node_move)
    }

    fn visit_typedef(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::typedef");
        visit::typedef(self, node_move)
    }

    fn visit_typedef_extern(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::typedef_extern");
        visit::typedef_extern(self, node_move)
    }

    fn visit_typedef_normal(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::typedef_normal");
        visit::typedef_normal(self, node_move)
    }

    fn visit_word(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::word");
        visit::word(self, node_move)
    }
}

#[allow(unused)]
#[allow(missing_docs)]
pub mod visit {
    use crate::{
        error::SyntaxError,
        language::{
            dl::{keyword, kind, symbol, utils, Visitor},
            NodeMove,
        },
        node::GotoNext,
    };

    #[allow(non_snake_case)]
    pub fn ROOT<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ROOT, node_move, GotoNext::StepInto)?;
        utils::repeat::eof(Vis::visit_annotated_item)(visitor, NodeMove::Step)
    }

    pub fn annotated_item<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::ANNOTATED_ITEM, node_move, GotoNext::StepInto)?;
        utils::seq((utils::optional(Vis::visit_attributes), Vis::visit_item))(visitor, NodeMove::Step)
    }

    pub fn apply<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::APPLY, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::APPLY),
            Vis::visit_name_trans,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::choice((
                (kind::NAME_FUNC, Vis::visit_name_func),
                (kind::NAME_REL, Vis::visit_name_rel),
            ))),
            utils::repeat(utils::seq((
                utils::token(symbol::COMMA),
                utils::choice((
                    (kind::NAME_FUNC, Vis::visit_name_func),
                    (kind::NAME_REL, Vis::visit_name_rel),
                )),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::token(symbol::RIGHTWARDS_ARROW),
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(Vis::visit_name_rel),
            utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_name_rel))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn arg<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ARG, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_arg,
            utils::token(symbol::COLON),
            utils::optional(utils::token(keyword::MUT)),
            Vis::visit_type_atom,
        ))(visitor, NodeMove::Step)
    }

    pub fn arg_opt_type<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::ARG_OPT_TYPE, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_arg,
            utils::optional(utils::seq((
                utils::token(symbol::COLON),
                utils::optional(utils::token(keyword::MUT)),
                Vis::visit_type_atom,
            ))),
            Vis::visit_type_atom,
        ))(visitor, NodeMove::Step)
    }

    pub fn arg_trans<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ARG_TRANS, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_trans,
            utils::token(symbol::COLON),
            Vis::visit_type_trans,
        ))(visitor, NodeMove::Step)
    }

    pub fn atom<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ATOM, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::ATOM_REC, Vis::visit_atom_rec),
            (kind::ATOM_POS, Vis::visit_atom_pos),
            (kind::ATOM_ELEM, Vis::visit_atom_elem),
        ))(visitor, NodeMove::Step)
    }

    pub fn atom_elem<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ATOM_ELEM, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_rel,
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn atom_pos<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ATOM_POS, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(utils::seq((Vis::visit_name_var_term, utils::token(keyword::IN)))),
            utils::seq((utils::optional(utils::token(symbol::AMPERSAND)), Vis::visit_name_rel)),
            utils::optional(utils::seq((
                utils::token(symbol::LEFT_PARENTHESIS),
                utils::optional(utils::seq((
                    Vis::visit_exp,
                    utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_exp))),
                    utils::optional(utils::token(symbol::COMMA)),
                ))),
                utils::token(symbol::RIGHT_PARENTHESIS),
            ))),
        ))(visitor, NodeMove::Step)
    }

    pub fn atom_rec<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ATOM_REC, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(utils::seq((Vis::visit_name_var_term, utils::token(keyword::IN)))),
            utils::seq((utils::optional(utils::token(symbol::AMPERSAND)), Vis::visit_name_rel)),
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::token(symbol::FULL_STOP),
            Vis::visit_name_arg,
            utils::token(symbol::EQUALS_SIGN),
            Vis::visit_exp,
            utils::repeat(utils::seq((
                utils::token(symbol::COMMA),
                utils::token(symbol::FULL_STOP),
                Vis::visit_name_arg,
                utils::token(symbol::EQUALS_SIGN),
                Vis::visit_exp,
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn attribute<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ATTRIBUTE, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name,
            utils::optional(utils::seq((utils::token(symbol::EQUALS_SIGN), Vis::visit_exp))),
        ))(visitor, NodeMove::Step)
    }

    pub fn attributes<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ATTRIBUTES, node_move, GotoNext::StepInto)?;
        utils::repeat1(utils::seq((
            utils::token(symbol::NUMBER_SIGN_LEFT_SQUARE_BRACKET),
            Vis::visit_attribute,
            utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_attribute))),
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        )))(visitor, NodeMove::Step)
    }

    pub fn comment_block<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::COMMENT_BLOCK, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::SOLIDUS_ASTERISK),
            utils::repeat(utils::choice((
                (kind::COMMENT_BLOCK, Vis::visit_comment_block),
                (kind::COMMENT_BLOCK_INNER, Vis::visit_comment_block_inner),
            ))),
            utils::token(symbol::ASTERISK_SOLIDUS),
        ))(visitor, NodeMove::Step)
    }

    pub fn comment_block_inner<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::COMMENT_BLOCK_INNER, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn comment_line<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::COMMENT_LINE, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn cons<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::CONS, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::CONS_REC, Vis::visit_cons_rec),
            (kind::CONS_POS, Vis::visit_cons_pos),
        ))(visitor, NodeMove::Step)
    }

    pub fn cons_pos<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::CONS_POS, node_move, GotoNext::StepInto)?;
        utils::seq((utils::optional(Vis::visit_attributes), Vis::visit_name_cons))(visitor, NodeMove::Step)
    }

    pub fn cons_rec<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::CONS_REC, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(Vis::visit_attributes),
            Vis::visit_name_cons,
            utils::token(symbol::LEFT_CURLY_BRACKET),
            utils::optional(utils::seq((
                Vis::visit_field,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_field))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn escape_sequence<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::ESCAPE_SEQUENCE, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn escape_sequence_interpolated<'tree, Vis>(
        visitor: &mut Vis,
        node_move: NodeMove,
    ) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::ESCAPE_SEQUENCE_INTERPOLATED, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn exp<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::EXP_ADD, Vis::visit_exp_add),
            (kind::EXP_ASSIGN, Vis::visit_exp_assign),
            (kind::EXP_BINDING, Vis::visit_exp_binding),
            (kind::EXP_BIT_AND, Vis::visit_exp_bit_and),
            (kind::EXP_BIT_NEG, Vis::visit_exp_bit_neg),
            (kind::EXP_BIT_OR, Vis::visit_exp_bit_or),
            (kind::EXP_BIT_SLICE, Vis::visit_exp_bit_slice),
            (kind::EXP_BIT_XOR, Vis::visit_exp_bit_xor),
            (kind::EXP_BLOCK, Vis::visit_exp_block),
            (kind::EXP_BREAK, Vis::visit_exp_break),
            (kind::EXP_CAST, Vis::visit_exp_cast),
            (kind::EXP_CAT, Vis::visit_exp_cat),
            (kind::EXP_COND, Vis::visit_exp_cond),
            (kind::EXP_CONS_POS, Vis::visit_exp_cons_pos),
            (kind::EXP_CONS_REC, Vis::visit_exp_cons_rec),
            (kind::EXP_CONTINUE, Vis::visit_exp_continue),
            (kind::EXP_DECL_VAR, Vis::visit_exp_decl_var),
            (kind::EXP_DIV, Vis::visit_exp_div),
            (kind::EXP_EQ, Vis::visit_exp_eq),
            (kind::EXP_FIELD, Vis::visit_exp_field),
            (kind::EXP_FOR, Vis::visit_exp_for),
            (kind::EXP_FUN_CALL, Vis::visit_exp_fun_call),
            (kind::EXP_FUN_CALL_DOT, Vis::visit_exp_fun_call_dot),
            (kind::EXP_GT, Vis::visit_exp_gt),
            (kind::EXP_GTEQ, Vis::visit_exp_gteq),
            (kind::EXP_LAMBDA, Vis::visit_exp_lambda),
            (kind::EXP_LIT, Vis::visit_exp_lit),
            (kind::EXP_LOG_AND, Vis::visit_exp_log_and),
            (kind::EXP_LOG_IMP, Vis::visit_exp_log_imp),
            (kind::EXP_LOG_NEG, Vis::visit_exp_log_neg),
            (kind::EXP_LOG_OR, Vis::visit_exp_log_or),
            (kind::EXP_LT, Vis::visit_exp_lt),
            (kind::EXP_LTEQ, Vis::visit_exp_lteq),
            (kind::EXP_MATCH, Vis::visit_exp_match),
            (kind::EXP_MUL, Vis::visit_exp_mul),
            (kind::EXP_NEG, Vis::visit_exp_neg),
            (kind::EXP_NEQ, Vis::visit_exp_neq),
            (kind::EXP_PROJ, Vis::visit_exp_proj),
            (kind::EXP_REF, Vis::visit_exp_ref),
            (kind::EXP_REM, Vis::visit_exp_rem),
            (kind::EXP_RETURN, Vis::visit_exp_return),
            (kind::EXP_SEQ, Vis::visit_exp_seq),
            (kind::EXP_SHL, Vis::visit_exp_shl),
            (kind::EXP_SHR, Vis::visit_exp_shr),
            (kind::EXP_SLICE, Vis::visit_exp_slice),
            (kind::EXP_SUB, Vis::visit_exp_sub),
            (kind::EXP_TRY, Vis::visit_exp_try),
            (kind::EXP_TUPLE, Vis::visit_exp_tuple),
            (kind::EXP_TYPE, Vis::visit_exp_type),
            (kind::EXP_WILD, Vis::visit_exp_wild),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_add<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_ADD, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::PLUS_SIGN), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_assign<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_ASSIGN, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::EQUALS_SIGN), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_binding<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_BINDING, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_var_term,
            utils::token(symbol::COMMERCIAL_AT),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_bit_and<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_BIT_AND, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::AMPERSAND), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_bit_neg<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_BIT_NEG, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::TILDE), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_bit_or<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_BIT_OR, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::VERTICAL_LINE), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_bit_slice<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_BIT_OR, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_bit_xor<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_BIT_XOR, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::CIRCUMFLEX_ACCENT), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_block<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_BLOCK, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_CURLY_BRACKET),
            utils::optional(Vis::visit_exp),
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_break<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_BREAK, node_move, GotoNext::StepInto)?;
        // token::BREAK(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn exp_cast<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_CAST, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(keyword::AS), Vis::visit_type_atom))(visitor, NodeMove::Step)
    }

    pub fn exp_cat<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_CAT, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::PLUS_SIGN_PLUS_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_cond<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_COND, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::IF),
            Vis::visit_exp,
            Vis::visit_exp,
            utils::optional(utils::seq((utils::token(keyword::ELSE), Vis::visit_exp))),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_cons_pos<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_CONS_POS, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_cons,
            utils::optional(utils::seq((
                utils::token(symbol::LEFT_CURLY_BRACKET),
                utils::optional(utils::seq((
                    Vis::visit_exp,
                    utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_exp))),
                ))),
                utils::token(symbol::RIGHT_CURLY_BRACKET),
            ))),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_cons_rec<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_CONS_REC, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_cons,
            utils::token(symbol::LEFT_CURLY_BRACKET),
            utils::optional(utils::seq((
                utils::token(symbol::FULL_STOP),
                Vis::visit_name_field,
                utils::token(symbol::EQUALS_SIGN),
                Vis::visit_exp,
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::token(symbol::FULL_STOP),
                    Vis::visit_name_field,
                    utils::token(symbol::EQUALS_SIGN),
                    Vis::visit_exp,
                ))),
            ))),
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_continue<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_CONTINUE, node_move, GotoNext::StepInto)?;
        // token::CONTINUE(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn exp_decl_var<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_DECL_VAR, node_move, GotoNext::StepInto)?;
        utils::seq((utils::optional(utils::token(keyword::VAR)), Vis::visit_name_var_term))(visitor, NodeMove::Step)
    }

    pub fn exp_div<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_DIV, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::SOLIDUS), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_eq<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_DIV, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::EQUALS_SIGN_EQUALS_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_field<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_FIELD, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::FULL_STOP), Vis::visit_ident))(visitor, NodeMove::Step)
    }

    pub fn exp_for<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_FOR, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::FOR),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_name_var_term,
            utils::token(keyword::IN),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_PARENTHESIS),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_fun_call<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_FUN_CALL, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_exp,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_exp))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_fun_call_dot<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_FUN_CALL_DOT, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::FULL_STOP),
            Vis::visit_name_func,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_exp,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_exp))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_gt<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_GT, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::GREATER_THAN_SIGN), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_gteq<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_GTEQ, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::GREATER_THAN_SIGN_EQUALS_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_lambda<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_LAMBDA, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::EXP_LAMBDA_BRANCH_0, Vis::visit_exp_lambda_branch_0),
            (kind::EXP_LAMBDA_BRANCH_1, Vis::visit_exp_lambda_branch_1),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_lambda_branch_0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_LAMBDA_BRANCH_0, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::FUNCTION),
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg_opt_type,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg_opt_type))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::optional(utils::seq((utils::token(symbol::COLON), Vis::visit_type_atom))),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_lambda_branch_1<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_LAMBDA_BRANCH_1, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::VERTICAL_LINE),
            utils::optional(utils::seq((
                Vis::visit_arg_opt_type,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg_opt_type))),
            ))),
            utils::token(symbol::VERTICAL_LINE),
            utils::optional(utils::seq((utils::token(symbol::COLON), Vis::visit_type_atom))),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_lit<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_LIT, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::LIT_BOOL, Vis::visit_lit_bool),
            (kind::LIT_NUM, Vis::visit_lit_num),
            (kind::LIT_MAP, Vis::visit_lit_map),
            (kind::LIT_STRING, Vis::visit_lit_string),
            (kind::LIT_VEC, Vis::visit_lit_vec),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_log_and<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_LOG_AND, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(keyword::AND), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_log_imp<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_LOG_IMP, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::EQUALS_SIGN_GREATER_THAN_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_log_neg<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_LOG_NEG, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::NOT), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_log_or<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_LOG_OR, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(keyword::OR), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_lt<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_LT, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::LESS_THAN_SIGN), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_lteq<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_LTEQ, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::LESS_THAN_SIGN_EQUALS_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_match<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_MATCH, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::MATCH),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::token(symbol::LEFT_CURLY_BRACKET),
            utils::optional(utils::seq((
                utils::seq((Vis::visit_pat, utils::token(symbol::RIGHTWARDS_ARROW), Vis::visit_exp)),
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::seq((Vis::visit_pat, utils::token(symbol::RIGHTWARDS_ARROW), Vis::visit_exp)),
                ))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_mul<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_MUL, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::ASTERISK), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_neg<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_NEG, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::HYPHEN_MINUS), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_neq<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_NEQ, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::EXCLAMATION_MARK_EQUALS_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_proj<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_PROJ, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::FULL_STOP),
            Vis::visit_exp_proj_digits,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_proj_digits<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::EXP_PROJ_DIGITS, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn exp_ref<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_REF, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::AMPERSAND), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_rem<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_REM, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::PERCENT_SIGN), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_return<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_RETURN, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::RETURN), utils::optional(Vis::visit_exp)))(visitor, NodeMove::Step)
    }

    pub fn exp_seq<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_SEQ, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::SEMICOLON),
            utils::optional(Vis::visit_exp),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_shl<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_SHL, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::LESS_THAN_SIGN_LESS_THAN_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_shr<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_SHR, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::GREATER_THAN_SIGN_GREATER_THAN_SIGN),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_slice<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_SLICE, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_exp,
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            Vis::visit_lit_num_dec,
            utils::token(symbol::COLON),
            Vis::visit_lit_num_dec,
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_sub<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_SUB, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::HYPHEN_MINUS), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn exp_try<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_TRY, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::QUESTION_MARK)))(visitor, NodeMove::Step)
    }

    pub fn exp_tuple<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_TUPLE, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_exp,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_exp))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn exp_type<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_TYPE, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(symbol::COLON), Vis::visit_type_atom))(visitor, NodeMove::Step)
    }

    pub fn exp_wild<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP_WILD, node_move, GotoNext::StepInto)?;
        // token::LOW_LINE(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn field<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::FIELD, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(Vis::visit_attributes),
            Vis::visit_name_field,
            utils::token(symbol::COLON),
            Vis::visit_type_atom,
        ))(visitor, NodeMove::Step)
    }

    pub fn function<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::FUNCTION, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::FUNCTION_NORMAL, Vis::visit_function_normal),
            (kind::FUNCTION_EXTERN, Vis::visit_function_extern),
        ))(visitor, NodeMove::Step)
    }

    pub fn function_extern<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::FUNCTION_EXTERN, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::EXTERN),
            utils::token(keyword::FUNCTION),
            Vis::visit_name_func,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::optional(utils::seq((utils::token(symbol::COLON), Vis::visit_type_atom))),
        ))(visitor, NodeMove::Step)
    }

    pub fn function_normal<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::FUNCTION_NORMAL, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::FUNCTION),
            Vis::visit_name_func,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::optional(utils::seq((utils::token(symbol::COLON), Vis::visit_type_atom))),
            utils::choice((
                (kind::FUNCTION_NORMAL_BRANCH_0, Vis::visit_function_normal_branch_0),
                (kind::FUNCTION_NORMAL_BRANCH_1, Vis::visit_function_normal_branch_1),
            )),
        ))(visitor, NodeMove::Step)
    }

    pub fn function_normal_branch_0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::FUNCTION_NORMAL_BRANCH_0, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::EQUALS_SIGN), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn function_normal_branch_1<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::FUNCTION_NORMAL_BRANCH_1, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_CURLY_BRACKET),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn ident<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::IDENT, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::IDENT_LOWER, Vis::visit_ident_lower),
            (kind::IDENT_UPPER, Vis::visit_ident_upper),
        ))(visitor, NodeMove::Step)
    }

    pub fn ident_lower<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::IDENT_LOWER, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn ident_lower_scoped<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::IDENT_LOWER_SCOPED, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn ident_scoped<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::IDENT_SCOPED, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn ident_upper<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::IDENT_UPPER, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn ident_upper_scoped<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::IDENT_UPPER_SCOPED, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn import<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::IMPORT, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::IMPORT),
            Vis::visit_module_path,
            utils::optional(utils::seq((utils::token(keyword::AS), Vis::visit_module_alias))),
        ))(visitor, NodeMove::Step)
    }

    pub fn index<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::INDEX, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::INDEX),
            Vis::visit_name_index,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::token(keyword::ON),
            Vis::visit_atom,
        ))(visitor, NodeMove::Step)
    }

    pub fn interpolation<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::INTERPOLATION, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::DOLLAR_SIGN_LEFT_CURLY_BRACKET),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn item<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ITEM, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::STATEMENT_FOR, Vis::visit_statement_for),
            (kind::APPLY, Vis::visit_apply),
            (kind::IMPORT, Vis::visit_import),
            (kind::FUNCTION, Vis::visit_function),
            (kind::INDEX, Vis::visit_index),
            (kind::REL, Vis::visit_rel),
            (kind::RULE, Vis::visit_rule),
            (kind::TRANSFORMER, Vis::visit_transformer),
            (kind::TYPEDEF, Vis::visit_typedef),
        ))(visitor, NodeMove::Step)
    }

    pub fn key_primary<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::KEY_PRIMARY, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::PRIMARY),
            utils::token(keyword::KEY),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_name_var_term,
            utils::token(symbol::RIGHT_PARENTHESIS),
            Vis::visit_exp,
        ))(visitor, NodeMove::Step)
    }

    pub fn lit_bool<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::LIT_BOOL, node_move, GotoNext::StepInto)?;
        utils::choice((
            (keyword::FALSE, utils::token(keyword::FALSE)),
            (keyword::TRUE, utils::token(keyword::TRUE)),
        ))(visitor, NodeMove::Step)
    }

    pub fn lit_map<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::LIT_MAP, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            Vis::visit_exp,
            utils::token(symbol::RIGHTWARDS_ARROW),
            Vis::visit_exp,
            utils::repeat(utils::seq((
                utils::token(symbol::COMMA),
                Vis::visit_exp,
                utils::token(symbol::RIGHTWARDS_ARROW),
                Vis::visit_exp,
            ))),
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn lit_num<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::LIT_NUM, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::LIT_NUM_BRANCH_0, Vis::visit_lit_num_branch_0),
            (kind::LIT_NUM_BRANCH_1, Vis::visit_lit_num_branch_1),
        ))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_0, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::LIT_NUM_DEC, Vis::visit_lit_num_dec),
            (kind::LIT_NUM_FLOAT, Vis::visit_lit_num_float),
            (kind::LIT_NUM_HEX, Vis::visit_lit_num_hex),
        ))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_1<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_1, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(Vis::visit_lit_num_dec),
            utils::choice((
                (kind::LIT_NUM_BRANCH_10, Vis::visit_lit_num_branch_10),
                (kind::LIT_NUM_BRANCH_11, Vis::visit_lit_num_branch_11),
                (kind::LIT_NUM_BRANCH_12, Vis::visit_lit_num_branch_12),
                (kind::LIT_NUM_BRANCH_13, Vis::visit_lit_num_branch_13),
                (kind::LIT_NUM_BRANCH_14, Vis::visit_lit_num_branch_14),
                (kind::LIT_NUM_BRANCH_15, Vis::visit_lit_num_branch_15),
                (kind::LIT_NUM_BRANCH_16, Vis::visit_lit_num_branch_16),
                (kind::LIT_NUM_BRANCH_17, Vis::visit_lit_num_branch_17),
                (kind::LIT_NUM_BRANCH_18, Vis::visit_lit_num_branch_18),
            )),
        ))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_10<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_10, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_BIN), Vis::visit_lit_num_bin))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_11<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_11, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_DEC), Vis::visit_lit_num_dec))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_12<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_12, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_FLOAT), Vis::visit_lit_num_float))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_13<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_13, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_HEX), Vis::visit_lit_num_hex))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_14<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_14, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_OCT), Vis::visit_lit_num_oct))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_15<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_15, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_S_BIN), Vis::visit_lit_num_bin))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_16<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_16, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_S_DEC), Vis::visit_lit_num_dec))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_17<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_17, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_S_HEX), Vis::visit_lit_num_hex))(visitor, NodeMove::Step)
    }

    pub fn lit_num_branch_18<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BRANCH_18, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::LIT_S_OCT), Vis::visit_lit_num_oct))(visitor, NodeMove::Step)
    }

    pub fn lit_num_bin<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_BIN, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn lit_num_dec<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_DEC, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn lit_num_float<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_FLOAT, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn lit_num_hex<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_HEX, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn lit_num_oct<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_NUM_OCT, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn lit_string<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::LIT_STRING, node_move, GotoNext::StepInto)?;
        utils::repeat1(utils::choice((
            (kind::STRING_QUOTED, Vis::visit_string_quoted),
            (kind::STRING_QUOTED_ESCAPED, Vis::visit_string_quoted_escaped),
            (kind::STRING_RAW, Vis::visit_string_raw),
            (kind::STRING_RAW_INTERPOLATED, Vis::visit_string_raw_interpolated),
        )))(visitor, NodeMove::Step)
    }

    pub fn lit_vec<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::LIT_VEC, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            Vis::visit_exp,
            utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_exp))),
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn misc_pat0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::MISC_PAT0, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn module_alias<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::MODULE_ALIAS, node_move, GotoNext::StepInto)?;
        Vis::visit_ident(visitor, NodeMove::Step)
    }

    pub fn module_path<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::MODULE_PATH, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_ident,
            utils::repeat(utils::seq((utils::token(symbol::COLON_COLON), Vis::visit_ident))),
        ))(visitor, NodeMove::Step)
    }

    pub fn name<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn name_arg<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_ARG, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn name_cons<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_CONS, node_move, GotoNext::StepInto)?;
        Vis::visit_ident_upper_scoped(visitor, NodeMove::Step)
    }

    pub fn name_field<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_FIELD, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn name_func<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_FUNC, node_move, GotoNext::StepInto)?;
        Vis::visit_ident_lower_scoped(visitor, NodeMove::Step)
    }

    pub fn name_index<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_INDEX, node_move, GotoNext::StepInto)?;
        Vis::visit_ident_scoped(visitor, NodeMove::Step)
    }

    pub fn name_rel<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_REL, node_move, GotoNext::StepInto)?;
        Vis::visit_ident_upper_scoped(visitor, NodeMove::Step)
    }

    pub fn name_trans<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_TRANS, node_move, GotoNext::StepInto)?;
        Vis::visit_ident_scoped(visitor, NodeMove::Step)
    }

    pub fn name_type<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::NAME_TYPE, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::IDENT_LOWER_SCOPED, Vis::visit_ident_lower_scoped),
            (kind::IDENT_UPPER_SCOPED, Vis::visit_ident_upper_scoped),
        ))(visitor, NodeMove::Step)
    }

    pub fn name_var_term<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::NAME_VAR_TERM, node_move, GotoNext::StepInto)?;
        Vis::visit_ident_lower_scoped(visitor, NodeMove::Step)
    }

    pub fn name_var_type<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::NAME_VAR_TYPE, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn pat<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::PAT, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::PAT_CONS, Vis::visit_pat_cons),
            (kind::PAT_TERM_DECL_VAR, Vis::visit_pat_term_decl_var),
            (kind::PAT_LIT, Vis::visit_pat_lit),
            (kind::PAT_TUPLE, Vis::visit_pat_tuple),
            (kind::PAT_TYPE, Vis::visit_pat_type),
            (kind::PAT_WILD, Vis::visit_pat_wild),
        ))(visitor, NodeMove::Step)
    }

    pub fn pat_cons<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::PAT_CONS, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::PAT_CONS_REC, Vis::visit_pat_cons_rec),
            (kind::PAT_CONS_POS, Vis::visit_pat_cons_pos),
        ))(visitor, NodeMove::Step)
    }

    pub fn pat_cons_pos<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::PAT_CONS_POS, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_cons,
            utils::optional(utils::seq((
                utils::token(symbol::LEFT_CURLY_BRACKET),
                utils::optional(utils::seq((
                    Vis::visit_pat,
                    utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_pat))),
                    utils::optional(utils::token(symbol::COMMA)),
                ))),
                utils::token(symbol::RIGHT_CURLY_BRACKET),
            ))),
        ))(visitor, NodeMove::Step)
    }

    pub fn pat_cons_rec<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::PAT_CONS_REC, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_cons,
            utils::token(symbol::LEFT_CURLY_BRACKET),
            utils::optional(utils::seq((
                utils::token(symbol::FULL_STOP),
                Vis::visit_name_field,
                utils::token(symbol::EQUALS_SIGN),
                Vis::visit_pat,
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::token(symbol::FULL_STOP),
                    Vis::visit_name_field,
                    utils::token(symbol::EQUALS_SIGN),
                    Vis::visit_pat,
                ))),
            ))),
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn pat_lit<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::PAT_LIT, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::LIT_BOOL, Vis::visit_lit_bool),
            (kind::LIT_NUM, Vis::visit_lit_num),
            (kind::LIT_STRING, Vis::visit_lit_string),
        ))(visitor, NodeMove::Step)
    }

    pub fn pat_term_decl_var<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::PAT_TERM_DECL_VAR, node_move, GotoNext::StepInto)?;
        utils::seq((utils::optional(utils::token(keyword::VAR)), Vis::visit_name_var_term))(visitor, NodeMove::Step)
    }

    pub fn pat_tuple<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::PAT_TUPLE, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_pat,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_pat))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn pat_type<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::PAT_TYPE, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_pat, utils::token(symbol::COLON), Vis::visit_type_atom))(visitor, NodeMove::Step)
    }

    pub fn pat_wild<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::PAT_WILD, node_move, GotoNext::StepInto)?;
        // token::LOW_LINE(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn rel<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::REL, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::REL_ARGS, Vis::visit_rel_args),
            (kind::REL_ELEM, Vis::visit_rel_elem),
        ))(visitor, NodeMove::Step)
    }

    pub fn rel_args<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::REL_ARGS, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(Vis::visit_rel_role),
            Vis::visit_rel_semantics,
            utils::seq((utils::optional(utils::token(symbol::AMPERSAND)), Vis::visit_name_rel)),
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::optional(Vis::visit_key_primary),
        ))(visitor, NodeMove::Step)
    }

    pub fn rel_elem<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::REL_ELEM, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(Vis::visit_rel_role),
            Vis::visit_rel_semantics,
            utils::seq((utils::optional(utils::token(symbol::AMPERSAND)), Vis::visit_name_rel)),
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            Vis::visit_type_atom,
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
            utils::optional(Vis::visit_key_primary),
        ))(visitor, NodeMove::Step)
    }

    pub fn rel_role<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::REL_ROLE, node_move, GotoNext::StepInto)?;
        utils::choice((
            (keyword::INPUT, utils::token(keyword::INPUT)),
            (keyword::INTERNAL, utils::token(keyword::INTERNAL)),
            (keyword::OUTPUT, utils::token(keyword::OUTPUT)),
        ))(visitor, NodeMove::Step)
    }

    pub fn rel_semantics<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::REL_SEMANTICS, node_move, GotoNext::StepInto)?;
        utils::choice((
            (keyword::RELATION, utils::token(keyword::RELATION)),
            (keyword::STREAM, utils::token(keyword::STREAM)),
            (keyword::MULTISET, utils::token(keyword::MULTISET)),
        ))(visitor, NodeMove::Step)
    }

    pub fn rhs<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::RHS, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::RHS_INSPECT, Vis::visit_rhs_inspect),
            (kind::ATOM, Vis::visit_atom),
            (kind::RHS_ATOM_NEG, Vis::visit_rhs_atom_neg),
            (kind::EXP, Vis::visit_exp),
            (kind::RHS_FLAT_MAP, Vis::visit_rhs_flat_map),
            (kind::RHS_GROUPING, Vis::visit_rhs_grouping),
        ))(visitor, NodeMove::Step)
    }

    pub fn rhs_atom_neg<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::RHS_ATOM_NEG, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::NOT), Vis::visit_atom))(visitor, NodeMove::Step)
    }

    pub fn rhs_flat_map<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::RHS_FLAT_MAP, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::VAR),
            Vis::visit_name_var_term,
            utils::token(symbol::EQUALS_SIGN),
            utils::token(keyword::FLAT_MAP),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn rhs_grouping<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::RHS_GROUPING, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::VAR),
            Vis::visit_name_var_term,
            utils::token(symbol::EQUALS_SIGN),
            Vis::visit_exp,
            utils::token(symbol::FULL_STOP),
            utils::token(keyword::GROUP_BY),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn rhs_inspect<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::RHS_INSPECT, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::INSPECT), Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn rule<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::RULE, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_atom,
            utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_atom))),
            utils::optional(utils::seq((
                utils::token(symbol::COLON_HYPHEN_MINUS),
                Vis::visit_rhs,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_rhs))),
            ))),
            Vis::visit_rule_end,
        ))(visitor, NodeMove::Step)
    }

    pub fn rule_end<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::RULE_END, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn statement<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::STATEMENT, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::STATEMENT_ASSIGN, Vis::visit_statement_assign),
            (kind::STATEMENT_BLOCK, Vis::visit_statement_block),
            (kind::STATEMENT_EMPTY, Vis::visit_statement_empty),
            (kind::STATEMENT_FOR, Vis::visit_statement_for),
            (kind::STATEMENT_IF, Vis::visit_statement_if),
            (kind::STATEMENT_INSERT, Vis::visit_statement_insert),
            (kind::STATEMENT_MATCH, Vis::visit_statement_match),
        ))(visitor, NodeMove::Step)
    }

    pub fn statement_assign<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STATEMENT_ASSIGN, node_move, GotoNext::StepInto)?;
        utils::seq((Vis::visit_exp, utils::token(keyword::IN), Vis::visit_statement))(visitor, NodeMove::Step)
    }

    pub fn statement_block<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STATEMENT_BLOCK, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_CURLY_BRACKET),
            utils::repeat(utils::seq((
                Vis::visit_statement,
                utils::optional(utils::seq((
                    utils::token(symbol::SEMICOLON),
                    utils::optional(Vis::visit_statement),
                ))),
            ))),
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn statement_empty<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STATEMENT_EMPTY, node_move, GotoNext::StepInto)?;
        // token::SKIP(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn statement_for<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STATEMENT_FOR, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::FOR),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_atom,
            utils::optional(utils::seq((utils::token(keyword::IF), Vis::visit_exp))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            Vis::visit_statement,
        ))(visitor, NodeMove::Step)
    }

    pub fn statement_if<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STATEMENT_IF, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::IF),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_PARENTHESIS),
            Vis::visit_statement,
            utils::optional(utils::seq((utils::token(keyword::ELSE), Vis::visit_statement))),
        ))(visitor, NodeMove::Step)
    }

    pub fn statement_insert<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STATEMENT_INSERT, node_move, GotoNext::StepInto)?;
        Vis::visit_atom(visitor, NodeMove::Step)
    }

    pub fn statement_match<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STATEMENT_MATCH, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::MATCH),
            utils::token(symbol::LEFT_PARENTHESIS),
            Vis::visit_exp,
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::token(symbol::LEFT_CURLY_BRACKET),
            utils::optional(utils::seq((
                utils::seq((
                    Vis::visit_pat,
                    utils::token(symbol::RIGHTWARDS_ARROW),
                    Vis::visit_statement,
                )),
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::seq((
                        Vis::visit_pat,
                        utils::token(symbol::RIGHTWARDS_ARROW),
                        Vis::visit_statement,
                    )),
                ))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_CURLY_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn string_quoted<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_QUOTED, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::repeat(utils::choice((
                (kind::STRING_QUOTED_BRANCH_0, Vis::visit_string_quoted_branch_0),
                (kind::STRING_QUOTED_BRANCH_1, Vis::visit_string_quoted_branch_1),
                (kind::INTERPOLATION, Vis::visit_interpolation),
                (kind::ESCAPE_SEQUENCE_INTERPOLATED, Vis::visit_escape_sequence),
            ))),
            utils::token(symbol::QUOTATION_MARK),
        ))(visitor, NodeMove::Step)
    }

    pub fn string_quoted_branch_0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_QUOTED_BRANCH_0, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn string_quoted_branch_1<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_QUOTED_BRANCH_1, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn string_quoted_escaped<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_QUOTED_ESCAPED, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::repeat(utils::choice((
                (kind::STRING_QUOTED_BRANCH_0, Vis::visit_string_quoted_branch_0),
                (kind::STRING_QUOTED_BRANCH_1, Vis::visit_string_quoted_branch_1),
                (kind::INTERPOLATION, Vis::visit_interpolation),
                (
                    kind::ESCAPE_SEQUENCE_INTERPOLATED,
                    Vis::visit_escape_sequence_interpolated,
                ),
            ))),
            utils::token(symbol::QUOTATION_MARK),
        ))(visitor, NodeMove::Step)
    }

    pub fn string_quoted_escaped_branch_0<'tree, Vis>(
        visitor: &mut Vis,
        node_move: NodeMove,
    ) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_QUOTED_ESCAPED_BRANCH_0, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn string_quoted_escaped_branch_1<'tree, Vis>(
        visitor: &mut Vis,
        node_move: NodeMove,
    ) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_QUOTED_ESCAPED_BRANCH_1, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn string_raw<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::STRING_RAW, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn string_raw_interpolated<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_RAW_INTERPOLATED, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::repeat(utils::choice((
                (
                    kind::STRING_RAW_INTERPOLATED_BRANCH_0,
                    Vis::visit_string_raw_interpolated_branch_0,
                ),
                (
                    kind::STRING_RAW_INTERPOLATED_BRANCH_1,
                    Vis::visit_string_raw_interpolated_branch_1,
                ),
                (kind::INTERPOLATION, Vis::visit_interpolation),
            ))),
            utils::token(symbol::VERTICAL_LINE_RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn string_raw_interpolated_branch_0<'tree, Vis>(
        visitor: &mut Vis,
        node_move: NodeMove,
    ) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_RAW_INTERPOLATED_BRANCH_0, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn string_raw_interpolated_branch_1<'tree, Vis>(
        visitor: &mut Vis,
        node_move: NodeMove,
    ) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::STRING_RAW_INTERPOLATED_BRANCH_1, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn transformer<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TRANSFORMER, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::EXTERN),
            utils::token(keyword::TRANSFORMER),
            Vis::visit_name_trans,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg_trans,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg_trans))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::token(symbol::RIGHTWARDS_ARROW),
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg_trans,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg_trans))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn r#type<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::TYPE_BIT, Vis::visit_type_bit),
            (kind::TYPE_SIGNED, Vis::visit_type_signed),
            (kind::TYPE_BIGINT, Vis::visit_type_bigint),
            (kind::TYPE_DOUBLE, Vis::visit_type_double),
            (kind::TYPE_FLOAT, Vis::visit_type_float),
            (kind::TYPE_STRING, Vis::visit_type_string),
            (kind::TYPE_BOOL, Vis::visit_type_bool),
            (kind::TYPE_UNION, Vis::visit_type_union),
            (kind::TYPE_USER, Vis::visit_type_user),
            (kind::TYPE_VAR, Vis::visit_type_var),
            (kind::TYPE_FUN, Vis::visit_type_fun),
            (kind::TYPE_TUPLE, Vis::visit_type_tuple),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_atom<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_ATOM, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::TYPE_BIT, Vis::visit_type_bit),
            (kind::TYPE_SIGNED, Vis::visit_type_signed),
            (kind::TYPE_BIGINT, Vis::visit_type_bigint),
            (kind::TYPE_DOUBLE, Vis::visit_type_double),
            (kind::TYPE_FLOAT, Vis::visit_type_float),
            (kind::TYPE_STRING, Vis::visit_type_string),
            (kind::TYPE_BOOL, Vis::visit_type_bool),
            (kind::TYPE_USER, Vis::visit_type_user),
            (kind::TYPE_VAR, Vis::visit_type_var),
            (kind::TYPE_FUN, Vis::visit_type_fun),
            (kind::TYPE_TUPLE, Vis::visit_type_tuple),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_bigint<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_BIGINT, node_move, GotoNext::StepInto)?;
        // token::BIGINT(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn type_bit<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_BIT, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::BIT),
            utils::token(symbol::LESS_THAN_SIGN),
            Vis::visit_lit_num_dec,
            utils::token(symbol::GREATER_THAN_SIGN),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_bool<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_BOOL, node_move, GotoNext::StepInto)?;
        // token::BOOL(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn type_double<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_DOUBLE, node_move, GotoNext::StepInto)?;
        // token::DOUBLE(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn type_float<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_FLOAT, node_move, GotoNext::StepInto)?;
        // token::FLOAT(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn type_fun<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_FUN, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::TYPE_FUN_BRANCH_0, Vis::visit_type_fun_branch_0),
            (kind::TYPE_FUN_BRANCH_1, Vis::visit_type_fun_branch_1),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_fun_branch_0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_FUN_BRANCH_0, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::FUNCTION),
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                utils::optional(utils::token(keyword::MUT)),
                Vis::visit_type,
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::optional(utils::token(keyword::MUT)),
                    Vis::visit_type,
                ))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::optional(utils::seq((utils::token(symbol::COLON), Vis::visit_type))),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_fun_branch_1<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_FUN_BRANCH_1, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::VERTICAL_LINE),
            utils::optional(utils::seq((
                utils::optional(utils::token(keyword::MUT)),
                Vis::visit_type,
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::optional(utils::token(keyword::MUT)),
                    Vis::visit_type,
                ))),
            ))),
            utils::token(symbol::VERTICAL_LINE),
            utils::optional(utils::seq((utils::token(symbol::COLON), Vis::visit_type))),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_signed<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_SIGNED, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::SIGNED),
            utils::token(symbol::LESS_THAN_SIGN),
            Vis::visit_lit_num_dec,
            utils::token(symbol::GREATER_THAN_SIGN),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_string<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_STRING, node_move, GotoNext::StepInto)?;
        // token::STRING(visitor, NodeMove::Step)
        Ok(())
    }

    pub fn type_trans<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_TRANS, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::TYPE_TRANS_FUN, Vis::visit_type_trans_fun),
            (kind::TYPE_TRANS_REL, Vis::visit_type_trans_rel),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_trans_fun<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_TRANS_FUN, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::FUNCTION),
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg))),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::token(symbol::COLON),
            Vis::visit_type_atom,
        ))(visitor, NodeMove::Step)
    }

    pub fn type_trans_rel<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_TRANS_REL, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::RELATION),
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            Vis::visit_type_atom,
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_tuple<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_TUPLE, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_type_atom,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_type_atom))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_union<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_UNION, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::repeat(utils::seq((Vis::visit_cons, utils::token(symbol::VERTICAL_LINE)))),
            Vis::visit_cons,
        ))(visitor, NodeMove::Step)
    }

    pub fn type_user<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_USER, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_type,
            utils::optional(utils::seq((
                utils::token(symbol::LESS_THAN_SIGN),
                Vis::visit_type,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_type))),
                utils::token(symbol::GREATER_THAN_SIGN),
            ))),
        ))(visitor, NodeMove::Step)
    }

    pub fn type_var<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPE_VAR, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::APOSTROPHE), Vis::visit_misc_pat0))(visitor, NodeMove::Step)
    }

    pub fn typedef<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TYPEDEF, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::TYPEDEF_NORMAL, Vis::visit_typedef_normal),
            (kind::TYPEDEF_EXTERN, Vis::visit_typedef_extern),
        ))(visitor, NodeMove::Step)
    }

    pub fn typedef_extern<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPEDEF_EXTERN, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::EXTERN),
            utils::token(keyword::TYPE),
            Vis::visit_name_type,
            utils::optional(utils::seq((
                utils::token(symbol::LESS_THAN_SIGN),
                Vis::visit_name_var_type,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_name_var_type))),
                utils::token(symbol::GREATER_THAN_SIGN),
            ))),
        ))(visitor, NodeMove::Step)
    }

    pub fn typedef_normal<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPEDEF_NORMAL, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::TYPEDEF),
            Vis::visit_name_type,
            utils::optional(utils::seq((
                utils::token(symbol::LESS_THAN_SIGN),
                Vis::visit_name_var_type,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_name_var_type))),
                utils::token(symbol::GREATER_THAN_SIGN),
            ))),
            utils::token(symbol::EQUALS_SIGN),
            Vis::visit_type,
        ))(visitor, NodeMove::Step)
    }

    pub fn word<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::WORD, node_move, GotoNext::StepInto)?;
        Ok(())
    }
}

#[allow(missing_docs)]
pub struct TrivialVisitor<'tree> {
    walker: NodeWalker<'tree>,
}

impl<'tree> TrivialVisitor<'tree> {
    #[allow(missing_docs)]
    pub fn new(language: Language, node: tree_sitter::Node<'tree>) -> Self {
        let walker = NodeWalker::new(language, node);
        Self { walker }
    }
}

impl<'tree> HasWalker<'tree> for TrivialVisitor<'tree> {
    fn walker(&mut self) -> &mut NodeWalker<'tree> {
        &mut self.walker
    }
}

impl<'tree> Visitor<'tree> for TrivialVisitor<'tree> {
}
