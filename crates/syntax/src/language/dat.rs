//! Functions for working with the `.dat` grammar.

use crate::{
    error::SyntaxError,
    language::{utils, HasWalker, NodeMove},
    node::{GotoNext, NodeWalker},
};
use ddlog_lsp_languages::language::Language;

pub mod field {
    #![allow(missing_docs)]

    ddlog_lsp_macros::field_ids! {
        language: "ddlog.dat",
        fields: [
        ],
    }
}

#[allow(missing_docs)]
pub mod kind {
    ddlog_lsp_macros::node_kind_ids! {
        language: "ddlog.dat",
        node_kinds: [
            (ROOT, "ROOT", true),
            (ARG, "arg", true),
            (ARG_OPT_TYPE, "arg_opt_type", true),
            (ATOM, "atom", true),
            (ATOM_ELEM, "atom_elem", true),
            (ATOM_POS, "atom_pos", true),
            (ATOM_REC, "atom_rec", true),
            (ATTRIBUTE, "attribute", true),
            (ATTRIBUTES, "attributes", true),
            (CLEAR, "clear", true),
            (COMMAND, "command", true),
            (COMMENT_LINE, "comment_line", true),
            (COMMIT, "commit", true),
            (CONS, "cons", true),
            (CONS_ARG, "cons_arg", true),
            (CONS_ARGS, "cons_args", true),
            (CONS_POS, "cons_pos", true),
            (CONS_REC, "cons_rec", true),
            (DELETE, "delete", true),
            (DELETE_KEY, "delete_key", true),
            (DUMP, "dump", true),
            (DUMP_INDEX, "dump_index", true),
            (ECHO, "echo", true),
            (ESCAPE_SEQUENCE, "escape_sequence", true),
            (ESCAPE_SEQUENCE_INTERPOLATED, "escape_sequence_interpolated", true),
            (EXIT, "exit", true),
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
            (IDENT, "ident", false),
            (IDENT_LOWER, "ident_lower", true),
            (IDENT_LOWER_SCOPED, "ident_lower_scoped", true),
            (IDENT_SCOPED, "ident_scoped", true),
            (IDENT_UPPER, "ident_upper", true),
            (IDENT_UPPER_SCOPED, "ident_upper_scoped", true),
            (INSERT, "insert", true),
            (INSERT_OR_UPDATE, "insert_or_update", true),
            (INTERPOLATION, "interpolation", true),
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
            (LIT_SERIALIZED, "lit_serialized", true),
            (LIT_STRING, "lit_string", true),
            (LIT_VEC, "lit_vec", true),
            (LOG_LEVEL, "log_level", true),
            (MISC_PAT0, "misc_pat0", true),
            (MODIFY, "modify", true),
            (NAME, "name", true),
            (NAME_ARG, "name_arg", true),
            (NAME_CONS, "name_cons", true),
            (NAME_FIELD, "name_field", true),
            (NAME_FUNC, "name_func", true),
            (NAME_INDEX, "name_index", true),
            (NAME_REL, "name_rel", true),
            (NAME_TYPE, "name_type", true),
            (NAME_VAR_TERM, "name_var_term", true),
            (PAT, "pat", true),
            (PAT_CONS, "pat_cons", true),
            (PAT_CONS_POS, "pat_cons_pos", true),
            (PAT_CONS_REC, "pat_cons_rec", true),
            (PAT_LIT, "pat_lit", true),
            (PAT_TERM_DECL_VAR, "pat_term_decl_var", true),
            (PAT_TUPLE, "pat_tuple", true),
            (PAT_TYPE, "pat_type", true),
            (PAT_WILD, "pat_wild", true),
            (PROFILE, "profile", true),
            (QUERY_INDEX, "query_index", true),
            (RECORD, "record", true),
            (RECORD_NAMED, "record_named", true),
            (ROLLBACK, "rollback", true),
            (SERDE_ENCODING, "serde_encoding", true),
            (SLEEP, "sleep", true),
            (START, "start", true),
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
            (TIMESTAMP, "timestamp", true),
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
            (TYPE_VAR_IDENT, "type_var_ident", true),
            (UPDATE, "update", true),
            (UPDATES, "updates", true),
            (UPDATES_END, "updates_end", true),
            (VAL_ARRAY, "val_array", true),
            (VAL_STRUCT, "val_struct", true),
            (VAL_TUPLE, "val_tuple", true),
            (WORD, "word", true),
        ],
    }
}

#[allow(missing_docs)]
pub mod keyword {
    ddlog_lsp_macros::node_kind_ids! {
        language: "ddlog.dat",
        node_kinds: [
            (AND, "and", false),
            (AS, "as", false),
            // (BIGINT, "bigint", false),
            (BIT, "bit", false),
            // (BOOL, "bool", false),
            // (BREAK, "break", false),
            (CLEAR, "clear", false),
            (COMMIT, "commit", false),
            // (CONTINUE, "continue", false),
            (CPU, "cpu", false),
            (DELETE, "delete", false),
            (DELETE_KEY, "delete_key", false),
            // (DOUBLE, "double", false),
            (DUMP, "dump", false),
            (DUMP_CHANGES, "dump_changes", false),
            (DUMP_INDEX, "dump_index", false),
            (ECHO, "echo", false),
            (ELSE, "else", false),
            (EXIT, "exit", false),
            (FALSE, "false", false),
            // (FLOAT, "float", false),
            (FOR, "for", false),
            (FUNCTION, "function", false),
            (IF, "if", false),
            (IN, "in", false),
            (INSERT, "insert", false),
            (INSERT_OR_UPDATE, "insert_or_update", false),
            (JSON, "json", false),
            (LOG_LEVEL, "log_level", false),
            (MATCH, "match", false),
            (MODIFY, "modify", false),
            (MUT, "mut", false),
            (NOT, "not", false),
            (OFF, "off", false),
            (ON, "on", false),
            (OR, "or", false),
            (PROFILE, "profile", false),
            (QUERY_INDEX, "query_index", false),
            (RELATION, "relation", false),
            (RETURN, "return", false),
            (ROLLBACK, "rollback", false),
            (SIGNED, "signed", false),
            (SKIP, "skip", false),
            (SLEEP, "sleep", false),
            (START, "start", false),
            // (STRING, "string", false),
            (TIMESTAMP, "timestamp", false),
            (TRUE, "true", false),
            (VAR, "var", false),
        ]
    }
}

#[allow(missing_docs)]
pub mod symbol {
    ddlog_lsp_macros::node_kind_ids! {
        language: "ddlog.dat",
        node_kinds: [
            (AMPERSAND, "&", false),
            (APOSTROPHE, "'", false),
            (ASTERISK, "*", false),
            (CIRCUMFLEX_ACCENT, "^", false),
            (COLON, ":", false),
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
            (LEFTWARDS_ARROW, "<-", false),
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
            (LOW_LINE, "_", false),
            (NUMBER_SIGN, "#", false),
            (NUMBER_SIGN_LEFT_SQUARE_BRACKET, "#[", false),
            (PERCENT_SIGN, "%", false),
            (PLUS_SIGN, "+", false),
            (PLUS_SIGN_PLUS_SIGN, "++", false),
            (QUESTION_MARK, "?", false),
            (QUOTATION_MARK, "\"", false),
            (RIGHT_CURLY_BRACKET, "}", false),
            (RIGHT_PARENTHESIS, ")", false),
            (RIGHT_SQUARE_BRACKET, "]", false),
            (RIGHTWARDS_ARROW, "->", false),
            (SEMICOLON, ";", false),
            (SOLIDUS, "/", false),
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
                kind::ARG => {
                    self.visit_arg(NodeMove::Init)?;
                    break;
                },
                kind::ARG_OPT_TYPE => {
                    self.visit_arg_opt_type(NodeMove::Init)?;
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
                kind::CLEAR => {
                    self.visit_clear(NodeMove::Init)?;
                    break;
                },
                kind::COMMAND => {
                    self.visit_command(NodeMove::Init)?;
                    break;
                },
                kind::COMMENT_LINE => {
                    self.visit_comment_line(NodeMove::Init)?;
                    break;
                },
                kind::COMMIT => {
                    self.visit_commit(NodeMove::Init)?;
                    break;
                },
                kind::CONS => {
                    self.visit_cons(NodeMove::Init)?;
                    break;
                },
                kind::CONS_ARG => {
                    self.visit_cons_arg(NodeMove::Init)?;
                    break;
                },
                kind::CONS_ARGS => {
                    self.visit_cons_args(NodeMove::Init)?;
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
                kind::DELETE => {
                    self.visit_delete(NodeMove::Init)?;
                    break;
                },
                kind::DELETE_KEY => {
                    self.visit_delete_key(NodeMove::Init)?;
                    break;
                },
                kind::DUMP => {
                    self.visit_dump(NodeMove::Init)?;
                    break;
                },
                kind::DUMP_INDEX => {
                    self.visit_dump_index(NodeMove::Init)?;
                    break;
                },
                kind::ECHO => {
                    self.visit_echo(NodeMove::Init)?;
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
                kind::EXIT => {
                    self.visit_exit(NodeMove::Init)?;
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
                kind::INSERT => {
                    self.visit_insert(NodeMove::Init)?;
                    break;
                },
                kind::INSERT_OR_UPDATE => {
                    self.visit_insert_or_update(NodeMove::Init)?;
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
                kind::LIT_SERIALIZED => {
                    self.visit_lit_serialized(NodeMove::Init)?;
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
                kind::LOG_LEVEL => {
                    self.visit_log_level(NodeMove::Init)?;
                    break;
                },
                kind::MISC_PAT0 => {
                    self.visit_misc_pat0(NodeMove::Init)?;
                    break;
                },
                kind::MODIFY => {
                    self.visit_modify(NodeMove::Init)?;
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
                kind::NAME_TYPE => {
                    self.visit_name_type(NodeMove::Init)?;
                    break;
                },
                kind::NAME_VAR_TERM => {
                    self.visit_name_var_term(NodeMove::Init)?;
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
                kind::PROFILE => {
                    self.visit_profile(NodeMove::Init)?;
                    break;
                },
                kind::QUERY_INDEX => {
                    self.visit_query_index(NodeMove::Init)?;
                    break;
                },
                kind::RECORD => {
                    self.visit_record(NodeMove::Init)?;
                    break;
                },
                kind::RECORD_NAMED => {
                    self.visit_record_named(NodeMove::Init)?;
                    break;
                },
                kind::ROLLBACK => {
                    self.visit_rollback(NodeMove::Init)?;
                    break;
                },
                kind::SERDE_ENCODING => {
                    self.visit_serde_encoding(NodeMove::Init)?;
                    break;
                },
                kind::SLEEP => {
                    self.visit_sleep(NodeMove::Init)?;
                    break;
                },
                kind::START => {
                    self.visit_start(NodeMove::Init)?;
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
                kind::TIMESTAMP => {
                    self.visit_timestamp(NodeMove::Init)?;
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
                kind::UPDATE => {
                    self.visit_update(NodeMove::Init)?;
                    break;
                },
                kind::UPDATES => {
                    self.visit_updates(NodeMove::Init)?;
                    break;
                },
                kind::UPDATES_END => {
                    self.visit_updates_end(NodeMove::Init)?;
                    break;
                },
                kind::VAL_ARRAY => {
                    self.visit_val_array(NodeMove::Init)?;
                    break;
                },
                kind::VAL_STRUCT => {
                    self.visit_val_struct(NodeMove::Init)?;
                    break;
                },
                kind::VAL_TUPLE => {
                    self.visit_val_tuple(NodeMove::Init)?;
                    break;
                },
                kind::WORD => {
                    self.visit_word(NodeMove::Init)?;
                    break;
                },
                // The following are unreachable:
                //
                // kind::STRING_QUOTED => { self.visit_string_quoted(NodeMove::Init)?; break; }
                // kind::STRING_QUOTED_BRANCH_0 => { self.visit_string_quoted_branch_0(NodeMove::Init)?; break; }
                // kind::STRING_QUOTED_ESCAPED => { self.visit_string_quoted_escaped(NodeMove::Init)?; break; }
                // kind::STRING_QUOTED_ESCAPED_BRANCH_0 => { self.visit_string_quoted_escaped_branch_0(NodeMove::Init)?;
                // break; } kind::STRING_QUOTED_ESCAPED_BRANCH_1 => {
                // self.visit_string_quoted_escaped_branch_1(NodeMove::Init)?; break; } kind::STRING_RAW
                // => { self.visit_string_raw(NodeMove::Init)?; break; } kind::STRING_RAW_INTERPOLATED
                // => { self.visit_string_raw_interpolated(NodeMove::Init)?; break; } kind::TYPE_TRANS
                // => { self.visit_type_trans(NodeMove::Init)?; break; } kind::TYPE_TRANS_FUN => {
                // self.visit_type_trans_fun(NodeMove::Init)?; break; } kind::TYPE_TRANS_REL => {
                // self.visit_type_trans_rel(NodeMove::Init)?; break; } kind::TYPE_VAR_IDENT => {
                // self.visit_type_var_ident(NodeMove::Init)?; break; }
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
        visit::ROOT(self, node_move)
    }

    fn visit_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::arg(self, node_move)
    }

    fn visit_arg_opt_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::arg_opt_type(self, node_move)
    }

    fn visit_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::atom(self, node_move)
    }

    fn visit_atom_elem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::atom_elem(self, node_move)
    }

    fn visit_atom_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::atom_pos(self, node_move)
    }

    fn visit_atom_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::atom_rec(self, node_move)
    }

    fn visit_attribute(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::attribute(self, node_move)
    }

    fn visit_attributes(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::attributes(self, node_move)
    }

    fn visit_clear(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::clear(self, node_move)
    }

    fn visit_command(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::command(self, node_move)
    }

    fn visit_comment_line(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::comment_line(self, node_move)
    }

    fn visit_commit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::commit(self, node_move)
    }

    fn visit_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::cons(self, node_move)
    }

    fn visit_cons_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::cons_arg(self, node_move)
    }

    fn visit_cons_args(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::cons_args(self, node_move)
    }

    fn visit_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::cons_pos(self, node_move)
    }

    fn visit_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::cons_rec(self, node_move)
    }

    fn visit_delete(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::delete(self, node_move)
    }

    fn visit_delete_key(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::delete_key(self, node_move)
    }

    fn visit_dump(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::dump(self, node_move)
    }

    fn visit_dump_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::dump_index(self, node_move)
    }

    fn visit_echo(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::echo(self, node_move)
    }

    fn visit_escape_sequence(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::escape_sequence(self, node_move)
    }

    fn visit_escape_sequence_interpolated(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::escape_sequence_interpolated(self, node_move)
    }

    fn visit_exit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exit(self, node_move)
    }

    fn visit_exp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp(self, node_move)
    }

    fn visit_exp_add(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_add(self, node_move)
    }

    fn visit_exp_assign(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_assign(self, node_move)
    }

    fn visit_exp_binding(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_binding(self, node_move)
    }

    fn visit_exp_bit_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_bit_and(self, node_move)
    }

    fn visit_exp_bit_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_bit_neg(self, node_move)
    }

    fn visit_exp_bit_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_bit_or(self, node_move)
    }

    fn visit_exp_bit_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_bit_slice(self, node_move)
    }

    fn visit_exp_bit_xor(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_bit_xor(self, node_move)
    }

    fn visit_exp_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_block(self, node_move)
    }

    fn visit_exp_break(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_break(self, node_move)
    }

    fn visit_exp_cast(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_cast(self, node_move)
    }

    fn visit_exp_cat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_cat(self, node_move)
    }

    fn visit_exp_cond(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_cond(self, node_move)
    }

    fn visit_exp_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_cons_pos(self, node_move)
    }

    fn visit_exp_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_cons_rec(self, node_move)
    }

    fn visit_exp_continue(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_continue(self, node_move)
    }

    fn visit_exp_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_decl_var(self, node_move)
    }

    fn visit_exp_div(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_div(self, node_move)
    }

    fn visit_exp_eq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_eq(self, node_move)
    }

    fn visit_exp_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_field(self, node_move)
    }

    fn visit_exp_for(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_for(self, node_move)
    }

    fn visit_exp_fun_call(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_fun_call(self, node_move)
    }

    fn visit_exp_fun_call_dot(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_fun_call_dot(self, node_move)
    }

    fn visit_exp_gt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_gt(self, node_move)
    }

    fn visit_exp_gteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_gteq(self, node_move)
    }

    fn visit_exp_lambda(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_lambda(self, node_move)
    }

    fn visit_exp_lambda_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_lambda_branch_0(self, node_move)
    }

    fn visit_exp_lambda_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_lambda_branch_1(self, node_move)
    }

    fn visit_exp_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_lit(self, node_move)
    }

    fn visit_exp_log_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_log_and(self, node_move)
    }

    fn visit_exp_log_imp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_log_imp(self, node_move)
    }

    fn visit_exp_log_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_log_neg(self, node_move)
    }

    fn visit_exp_log_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_log_or(self, node_move)
    }

    fn visit_exp_lt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_lt(self, node_move)
    }

    fn visit_exp_lteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_lteq(self, node_move)
    }

    fn visit_exp_match(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_match(self, node_move)
    }

    fn visit_exp_mul(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_mul(self, node_move)
    }

    fn visit_exp_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_neg(self, node_move)
    }

    fn visit_exp_neq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_neq(self, node_move)
    }

    fn visit_exp_proj(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_proj(self, node_move)
    }

    fn visit_exp_proj_digits(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_proj_digits(self, node_move)
    }

    fn visit_exp_ref(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_ref(self, node_move)
    }

    fn visit_exp_rem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_rem(self, node_move)
    }

    fn visit_exp_return(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_return(self, node_move)
    }

    fn visit_exp_seq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_seq(self, node_move)
    }

    fn visit_exp_shl(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_shl(self, node_move)
    }

    fn visit_exp_shr(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_shr(self, node_move)
    }

    fn visit_exp_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_slice(self, node_move)
    }

    fn visit_exp_sub(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_sub(self, node_move)
    }

    fn visit_exp_try(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_try(self, node_move)
    }

    fn visit_exp_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_tuple(self, node_move)
    }

    fn visit_exp_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_type(self, node_move)
    }

    fn visit_exp_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_wild(self, node_move)
    }

    fn visit_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::exp_field(self, node_move)
    }

    fn visit_ident(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::ident(self, node_move)
    }

    fn visit_ident_lower(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::ident_lower(self, node_move)
    }

    fn visit_ident_lower_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::ident_lower_scoped(self, node_move)
    }

    fn visit_ident_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::ident_scoped(self, node_move)
    }

    fn visit_ident_upper(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::ident_upper(self, node_move)
    }

    fn visit_ident_upper_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::ident_upper_scoped(self, node_move)
    }

    fn visit_insert(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::insert(self, node_move)
    }

    fn visit_insert_or_update(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::insert_or_update(self, node_move)
    }

    fn visit_interpolation(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::interpolation(self, node_move)
    }

    fn visit_lit_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_bool(self, node_move)
    }

    fn visit_lit_map(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_map(self, node_move)
    }

    fn visit_lit_num(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num(self, node_move)
    }

    fn visit_lit_num_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_0(self, node_move)
    }

    fn visit_lit_num_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_1(self, node_move)
    }

    fn visit_lit_num_branch_10(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_10(self, node_move)
    }

    fn visit_lit_num_branch_11(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_11(self, node_move)
    }

    fn visit_lit_num_branch_12(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_12(self, node_move)
    }

    fn visit_lit_num_branch_13(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_13(self, node_move)
    }

    fn visit_lit_num_branch_14(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_14(self, node_move)
    }

    fn visit_lit_num_branch_15(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_15(self, node_move)
    }

    fn visit_lit_num_branch_16(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_16(self, node_move)
    }

    fn visit_lit_num_branch_17(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_17(self, node_move)
    }

    fn visit_lit_num_branch_18(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_branch_18(self, node_move)
    }

    fn visit_lit_num_bin(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_bin(self, node_move)
    }

    fn visit_lit_num_dec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_dec(self, node_move)
    }

    fn visit_lit_num_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_float(self, node_move)
    }

    fn visit_lit_num_hex(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_hex(self, node_move)
    }

    fn visit_lit_num_oct(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_num_oct(self, node_move)
    }

    fn visit_lit_serialized(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_serialized(self, node_move)
    }

    fn visit_lit_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_string(self, node_move)
    }

    fn visit_lit_vec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::lit_vec(self, node_move)
    }

    fn visit_log_level(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::log_level(self, node_move)
    }

    fn visit_misc_pat0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::misc_pat0(self, node_move)
    }

    fn visit_modify(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::modify(self, node_move)
    }

    fn visit_name(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name(self, node_move)
    }

    fn visit_name_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_arg(self, node_move)
    }

    fn visit_name_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_cons(self, node_move)
    }

    fn visit_name_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_field(self, node_move)
    }

    fn visit_name_func(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_func(self, node_move)
    }

    fn visit_name_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_index(self, node_move)
    }

    fn visit_name_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_rel(self, node_move)
    }

    fn visit_name_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_type(self, node_move)
    }

    fn visit_name_var_term(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::name_var_term(self, node_move)
    }

    fn visit_pat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat(self, node_move)
    }

    fn visit_pat_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_cons(self, node_move)
    }

    fn visit_pat_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_cons_pos(self, node_move)
    }

    fn visit_pat_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_cons_rec(self, node_move)
    }

    fn visit_pat_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_lit(self, node_move)
    }

    fn visit_pat_term_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_term_decl_var(self, node_move)
    }

    fn visit_pat_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_tuple(self, node_move)
    }

    fn visit_pat_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_type(self, node_move)
    }

    fn visit_pat_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::pat_wild(self, node_move)
    }

    fn visit_profile(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::profile(self, node_move)
    }

    fn visit_query_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::query_index(self, node_move)
    }

    fn visit_record(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::record(self, node_move)
    }

    fn visit_record_named(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::record_named(self, node_move)
    }

    fn visit_rollback(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::rollback(self, node_move)
    }

    fn visit_serde_encoding(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::serde_encoding(self, node_move)
    }

    fn visit_sleep(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::sleep(self, node_move)
    }

    fn visit_start(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::start(self, node_move)
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

    fn visit_timestamp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::timestamp(self, node_move)
    }

    fn visit_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::r#type(self, node_move)
    }

    fn visit_type_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_atom(self, node_move)
    }

    fn visit_type_bigint(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_bigint(self, node_move)
    }

    fn visit_type_bit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_bit(self, node_move)
    }

    fn visit_type_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_bool(self, node_move)
    }

    fn visit_type_double(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_double(self, node_move)
    }

    fn visit_type_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_float(self, node_move)
    }

    fn visit_type_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_fun(self, node_move)
    }

    fn visit_type_fun_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_fun_branch_0(self, node_move)
    }

    fn visit_type_fun_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_fun_branch_1(self, node_move)
    }

    fn visit_type_signed(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_signed(self, node_move)
    }

    fn visit_type_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_string(self, node_move)
    }

    fn visit_type_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_trans(self, node_move)
    }

    fn visit_type_trans_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_trans_fun(self, node_move)
    }

    fn visit_type_trans_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_trans_rel(self, node_move)
    }

    fn visit_type_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_tuple(self, node_move)
    }

    fn visit_type_union(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_union(self, node_move)
    }

    fn visit_type_user(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_user(self, node_move)
    }

    fn visit_type_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::type_var(self, node_move)
    }

    fn visit_update(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::update(self, node_move)
    }

    fn visit_updates(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::updates(self, node_move)
    }

    fn visit_updates_end(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::updates_end(self, node_move)
    }

    fn visit_val_array(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::val_array(self, node_move)
    }

    fn visit_val_struct(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::val_struct(self, node_move)
    }

    fn visit_val_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::val_tuple(self, node_move)
    }

    fn visit_word(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        visit::word(self, node_move)
    }
}

#[allow(unused)]
#[allow(missing_docs)]
pub mod visit {
    use crate::{
        error::SyntaxError,
        language::{
            dat::{keyword, kind, symbol, utils, Visitor},
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
        utils::repeat::eof(Vis::visit_command)(visitor, NodeMove::Step)
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
            Vis::visit_name_rel,
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
            Vis::visit_name_rel,
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

    pub fn clear<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::CLEAR, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::CLEAR),
            Vis::visit_name_rel,
            utils::token(symbol::SEMICOLON),
        ))(visitor, NodeMove::Step)
    }

    pub fn command<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::COMMAND, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::CLEAR, Vis::visit_clear),
            (kind::COMMIT, Vis::visit_commit),
            (kind::DUMP, Vis::visit_dump),
            (kind::DUMP_INDEX, Vis::visit_dump_index),
            (kind::ECHO, Vis::visit_echo),
            (kind::EXIT, Vis::visit_exit),
            (kind::LOG_LEVEL, Vis::visit_log_level),
            (kind::PROFILE, Vis::visit_profile),
            (kind::QUERY_INDEX, Vis::visit_query_index),
            (kind::ROLLBACK, Vis::visit_rollback),
            (kind::SLEEP, Vis::visit_sleep),
            (kind::START, Vis::visit_start),
            (kind::TIMESTAMP, Vis::visit_timestamp),
            (kind::UPDATES, Vis::visit_updates),
        ))(visitor, NodeMove::Step)
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

    pub fn commit<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::COMMIT, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::COMMIT),
            utils::optional(utils::token(keyword::DUMP_CHANGES)),
            utils::token(symbol::SEMICOLON),
        ))(visitor, NodeMove::Step)
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

    pub fn cons_arg<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::CONS_ARG, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_cons_arg,
            utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_cons_arg))),
        ))(visitor, NodeMove::Step)
    }

    pub fn cons_args<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::CONS_ARGS, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::RECORD_NAMED, Vis::visit_record_named),
            (kind::RECORD, Vis::visit_record),
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

    pub fn delete<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::DELETE, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::DELETE), Vis::visit_atom))(visitor, NodeMove::Step)
    }

    pub fn delete_key<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::DELETE_KEY, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::DELETE_KEY), Vis::visit_name_rel, Vis::visit_exp))(visitor, NodeMove::Step)
    }

    pub fn dump<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::DUMP, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::DUMP),
            utils::optional(Vis::visit_name_rel),
            utils::token(symbol::SEMICOLON),
        ))(visitor, NodeMove::Step)
    }

    pub fn dump_index<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::DUMP_INDEX, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::DUMP_INDEX),
            Vis::visit_name_index,
            utils::token(symbol::SEMICOLON),
        ))(visitor, NodeMove::Step)
    }

    pub fn echo<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ECHO, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::ECHO),
            Vis::visit_misc_pat0,
            utils::token(symbol::SEMICOLON),
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

    pub fn exit<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXIT, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::EXIT), utils::token(symbol::SEMICOLON)))(visitor, NodeMove::Step)
    }

    pub fn exp<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::EXP, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::LIT_SERIALIZED, Vis::visit_lit_serialized),
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
            (kind::EXP_CONTINUE, Vis::visit_exp_continue),
            (kind::EXP_CONS_POS, Vis::visit_exp_cons_pos),
            (kind::EXP_CONS_REC, Vis::visit_exp_cons_rec),
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
                Vis::visit_name_cons,
                utils::token(symbol::EQUALS_SIGN),
                Vis::visit_exp,
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::token(symbol::FULL_STOP),
                    Vis::visit_name_cons,
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
        visitor.walker().step(kind::EXP_EQ, node_move, GotoNext::StepInto)?;
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
        utils::token(symbol::LOW_LINE)(visitor, NodeMove::Step)
    }

    pub fn field<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::FIELD, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::optional(Vis::visit_attributes),
            Vis::visit_name_cons,
            utils::token(symbol::COLON),
            Vis::visit_type_atom,
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

    pub fn insert<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::INSERT, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::INSERT), Vis::visit_atom))(visitor, NodeMove::Step)
    }

    pub fn insert_or_update<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::INSERT_OR_UPDATE, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::INSERT_OR_UPDATE), Vis::visit_atom))(visitor, NodeMove::Step)
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

    pub fn lit_serialized<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::LIT_SERIALIZED, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(symbol::COMMERCIAL_AT), Vis::visit_serde_encoding))(visitor, NodeMove::Step)
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

    pub fn log_level<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::LOG_LEVEL, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::LOG_LEVEL), Vis::visit_misc_pat0))(visitor, NodeMove::Step)
    }

    pub fn misc_pat0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::MISC_PAT0, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn modify<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::MODIFY, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::MODIFY),
            Vis::visit_name_rel,
            Vis::visit_record,
            utils::token(symbol::LEFTWARDS_ARROW),
            Vis::visit_record,
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
                Vis::visit_name_cons,
                utils::token(symbol::EQUALS_SIGN),
                Vis::visit_pat,
                utils::repeat(utils::seq((
                    utils::token(symbol::COMMA),
                    utils::token(symbol::FULL_STOP),
                    Vis::visit_name_cons,
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
        utils::token(symbol::LOW_LINE)(visitor, NodeMove::Step)
    }

    pub fn profile<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::PROFILE, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::PROFILE),
            utils::optional(utils::seq((
                utils::token(keyword::CPU),
                utils::choice((
                    (keyword::ON, utils::token(keyword::ON)),
                    (keyword::OFF, utils::token(keyword::OFF)),
                )),
            ))),
            utils::token(symbol::SEMICOLON),
        ))(visitor, NodeMove::Step)
    }

    pub fn query_index<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::QUERY_INDEX, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::QUERY_INDEX),
            Vis::visit_name_index,
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_arg,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_arg))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
            utils::token(symbol::SEMICOLON),
        ))(visitor, NodeMove::Step)
    }

    pub fn record<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::RECORD, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::LIT_BOOL, Vis::visit_lit_bool),
            (kind::LIT_STRING, Vis::visit_lit_string),
            (kind::LIT_SERIALIZED, Vis::visit_lit_serialized),
            (kind::VAL_TUPLE, Vis::visit_val_tuple),
            (kind::VAL_ARRAY, Vis::visit_val_array),
            (kind::VAL_STRUCT, Vis::visit_val_struct),
            (kind::LIT_NUM_FLOAT, Vis::visit_lit_num_float),
            (kind::LIT_NUM_DEC, Vis::visit_lit_num_dec),
            (kind::LIT_NUM_HEX, Vis::visit_lit_num_hex),
        ))(visitor, NodeMove::Step)
    }

    pub fn record_named<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::RECORD_NAMED, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::FULL_STOP),
            Vis::visit_name_cons,
            utils::token(symbol::EQUALS_SIGN),
            Vis::visit_record,
        ))(visitor, NodeMove::Step)
    }

    pub fn rollback<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::ROLLBACK, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::ROLLBACK), utils::token(symbol::SEMICOLON)))(visitor, NodeMove::Step)
    }

    pub fn serde_encoding<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::SERDE_ENCODING, node_move, GotoNext::StepInto)?;
        utils::token(keyword::JSON)(visitor, NodeMove::Step)
    }

    pub fn sleep<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::SLEEP, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(keyword::SLEEP),
            Vis::visit_misc_pat0,
            utils::token(symbol::SEMICOLON),
        ))(visitor, NodeMove::Step)
    }

    pub fn start<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::START, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::START), utils::token(symbol::SEMICOLON)))(visitor, NodeMove::Step)
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

    pub fn timestamp<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::TIMESTAMP, node_move, GotoNext::StepInto)?;
        utils::seq((utils::token(keyword::TIMESTAMP), utils::token(symbol::SEMICOLON)))(visitor, NodeMove::Step)
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
        utils::seq((utils::token(symbol::APOSTROPHE), type_var_ident))(visitor, NodeMove::Step)
    }

    pub fn type_var_ident<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::TYPE_VAR_IDENT, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn update<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::UPDATE, node_move, GotoNext::StepInto)?;
        utils::choice((
            (kind::DELETE, Vis::visit_delete),
            (kind::DELETE_KEY, Vis::visit_delete_key),
            (kind::INSERT, Vis::visit_insert),
            (kind::INSERT_OR_UPDATE, Vis::visit_insert_or_update),
            (kind::MODIFY, Vis::visit_modify),
        ))(visitor, NodeMove::Step)
    }

    pub fn updates<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::UPDATES, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_update,
            utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_update))),
            Vis::visit_updates_end,
        ))(visitor, NodeMove::Step)
    }

    pub fn updates_end<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor
            .walker()
            .step(kind::UPDATES_END, node_move, GotoNext::StepInto)?;
        Ok(())
    }

    pub fn val_array<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::VAL_ARRAY, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_SQUARE_BRACKET),
            utils::optional(utils::seq((
                Vis::visit_record,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_record))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_SQUARE_BRACKET),
        ))(visitor, NodeMove::Step)
    }

    pub fn val_struct<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::VAL_STRUCT, node_move, GotoNext::StepInto)?;
        utils::seq((
            Vis::visit_name_rel,
            utils::optional(utils::seq((
                utils::token(symbol::LEFT_CURLY_BRACKET),
                Vis::visit_cons_args,
                utils::token(symbol::RIGHT_CURLY_BRACKET),
            ))),
        ))(visitor, NodeMove::Step)
    }

    pub fn val_tuple<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: Visitor<'tree> + ?Sized,
    {
        visitor.walker().step(kind::VAL_TUPLE, node_move, GotoNext::StepInto)?;
        utils::seq((
            utils::token(symbol::LEFT_PARENTHESIS),
            utils::optional(utils::seq((
                Vis::visit_record,
                utils::repeat(utils::seq((utils::token(symbol::COMMA), Vis::visit_record))),
                utils::optional(utils::token(symbol::COMMA)),
            ))),
            utils::token(symbol::RIGHT_PARENTHESIS),
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
