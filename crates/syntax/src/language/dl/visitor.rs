use crate::{
    error::SyntaxError,
    language::{dl::kind, HasWalker, NodeMove},
    node::GotoNext,
};

#[allow(missing_docs)]
pub mod default;

#[allow(missing_docs)]
pub mod validating;

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
                kind::LIT_NUM_BRANCH => {
                    self.visit_lit_num_branch(NodeMove::Init)?;
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
                kind::LIT_NUM_BRANCH_2 => {
                    self.visit_lit_num_branch_2(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_3 => {
                    self.visit_lit_num_branch_3(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_4 => {
                    self.visit_lit_num_branch_4(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_5 => {
                    self.visit_lit_num_branch_5(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_6 => {
                    self.visit_lit_num_branch_6(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_7 => {
                    self.visit_lit_num_branch_7(NodeMove::Init)?;
                    break;
                },
                kind::LIT_NUM_BRANCH_8 => {
                    self.visit_lit_num_branch_8(NodeMove::Init)?;
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
        default::ROOT(self, node_move)
    }

    fn visit_annotated_item(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::annotated_item");
        default::annotated_item(self, node_move)
    }

    fn visit_apply(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::apply");
        default::apply(self, node_move)
    }

    fn visit_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::arg");
        default::arg(self, node_move)
    }

    fn visit_arg_opt_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::arg_opt_type");
        default::arg_opt_type(self, node_move)
    }

    fn visit_arg_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::arg_trans");
        default::arg_trans(self, node_move)
    }

    fn visit_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom");
        default::atom(self, node_move)
    }

    fn visit_atom_elem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom_elem");
        default::atom_elem(self, node_move)
    }

    fn visit_atom_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom_pos");
        default::atom_pos(self, node_move)
    }

    fn visit_atom_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::atom_rec");
        default::atom_rec(self, node_move)
    }

    fn visit_attribute(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::attribute");
        default::attribute(self, node_move)
    }

    fn visit_attributes(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::attributes");
        default::attributes(self, node_move)
    }

    fn visit_comment_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::comment_block");
        default::comment_block(self, node_move)
    }

    fn visit_comment_block_inner(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::comment_block_inner");
        default::comment_block_inner(self, node_move)
    }

    fn visit_comment_line(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::comment_line");
        default::comment_line(self, node_move)
    }

    fn visit_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::cons");
        default::cons(self, node_move)
    }

    fn visit_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::cons_pos");
        default::cons_pos(self, node_move)
    }

    fn visit_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::cons_rec");
        default::cons_rec(self, node_move)
    }

    fn visit_escape_sequence(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::escape_sequence");
        default::escape_sequence(self, node_move)
    }

    fn visit_escape_sequence_interpolated(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::escape_sequence_interpolated");
        default::escape_sequence_interpolated(self, node_move)
    }

    fn visit_exp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp");
        default::exp(self, node_move)
    }

    fn visit_exp_add(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_add");
        default::exp_add(self, node_move)
    }

    fn visit_exp_assign(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_assign");
        default::exp_assign(self, node_move)
    }

    fn visit_exp_binding(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_binding");
        default::exp_binding(self, node_move)
    }

    fn visit_exp_bit_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_and");
        default::exp_bit_and(self, node_move)
    }

    fn visit_exp_bit_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_neg");
        default::exp_bit_neg(self, node_move)
    }

    fn visit_exp_bit_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_or");
        default::exp_bit_or(self, node_move)
    }

    fn visit_exp_bit_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_slice");
        default::exp_bit_slice(self, node_move)
    }

    fn visit_exp_bit_xor(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_bit_xor");
        default::exp_bit_xor(self, node_move)
    }

    fn visit_exp_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_block");
        default::exp_block(self, node_move)
    }

    fn visit_exp_break(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_break");
        default::exp_break(self, node_move)
    }

    fn visit_exp_cast(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cast");
        default::exp_cast(self, node_move)
    }

    fn visit_exp_cat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cat");
        default::exp_cat(self, node_move)
    }

    fn visit_exp_cond(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cond");
        default::exp_cond(self, node_move)
    }

    fn visit_exp_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cons_pos");
        default::exp_cons_pos(self, node_move)
    }

    fn visit_exp_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_cons_rec");
        default::exp_cons_rec(self, node_move)
    }

    fn visit_exp_continue(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_continue");
        default::exp_continue(self, node_move)
    }

    fn visit_exp_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_decl_var");
        default::exp_decl_var(self, node_move)
    }

    fn visit_exp_div(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_div");
        default::exp_div(self, node_move)
    }

    fn visit_exp_eq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_eq");
        default::exp_eq(self, node_move)
    }

    fn visit_exp_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_field");
        default::exp_field(self, node_move)
    }

    fn visit_exp_for(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_for");
        default::exp_for(self, node_move)
    }

    fn visit_exp_fun_call(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_fun_call");
        default::exp_fun_call(self, node_move)
    }

    fn visit_exp_fun_call_dot(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_fun_call_dot");
        default::exp_fun_call_dot(self, node_move)
    }

    fn visit_exp_gt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_gt");
        default::exp_gt(self, node_move)
    }

    fn visit_exp_gteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_gteq");
        default::exp_gteq(self, node_move)
    }

    fn visit_exp_lambda(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lambda");
        default::exp_lambda(self, node_move)
    }

    fn visit_exp_lambda_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lambda_branch_0");
        default::exp_lambda_branch_0(self, node_move)
    }

    fn visit_exp_lambda_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lambda_branch_1");
        default::exp_lambda_branch_1(self, node_move)
    }

    fn visit_exp_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lit");
        default::exp_lit(self, node_move)
    }

    fn visit_exp_log_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_and");
        default::exp_log_and(self, node_move)
    }

    fn visit_exp_log_imp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_imp");
        default::exp_log_imp(self, node_move)
    }

    fn visit_exp_log_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_neg");
        default::exp_log_neg(self, node_move)
    }

    fn visit_exp_log_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_log_or");
        default::exp_log_or(self, node_move)
    }

    fn visit_exp_lt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lt");
        default::exp_lt(self, node_move)
    }

    fn visit_exp_lteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_lteq");
        default::exp_lteq(self, node_move)
    }

    fn visit_exp_match(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_match");
        default::exp_match(self, node_move)
    }

    fn visit_exp_mul(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_mul");
        default::exp_mul(self, node_move)
    }

    fn visit_exp_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_neg");
        default::exp_neg(self, node_move)
    }

    fn visit_exp_neq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_neq");
        default::exp_neq(self, node_move)
    }

    fn visit_exp_proj(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_proj");
        default::exp_proj(self, node_move)
    }

    fn visit_exp_proj_digits(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_proj_digits");
        default::exp_proj_digits(self, node_move)
    }

    fn visit_exp_ref(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_ref");
        default::exp_ref(self, node_move)
    }

    fn visit_exp_rem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_rem");
        default::exp_rem(self, node_move)
    }

    fn visit_exp_return(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_return");
        default::exp_return(self, node_move)
    }

    fn visit_exp_seq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_seq");
        default::exp_seq(self, node_move)
    }

    fn visit_exp_shl(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_shl");
        default::exp_shl(self, node_move)
    }

    fn visit_exp_shr(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_shr");
        default::exp_shr(self, node_move)
    }

    fn visit_exp_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_slice");
        default::exp_slice(self, node_move)
    }

    fn visit_exp_sub(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_sub");
        default::exp_sub(self, node_move)
    }

    fn visit_exp_try(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_try");
        default::exp_try(self, node_move)
    }

    fn visit_exp_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_tuple");
        default::exp_tuple(self, node_move)
    }

    fn visit_exp_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_type");
        default::exp_type(self, node_move)
    }

    fn visit_exp_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::exp_wild");
        default::exp_wild(self, node_move)
    }

    fn visit_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::field");
        default::field(self, node_move)
    }

    fn visit_function(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function");
        default::function(self, node_move)
    }

    fn visit_function_extern(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_extern");
        default::function_extern(self, node_move)
    }

    fn visit_function_normal(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_normal");
        default::function_normal(self, node_move)
    }

    fn visit_function_normal_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_normal_branch_0");
        default::function_normal_branch_0(self, node_move)
    }

    fn visit_function_normal_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::function_normal_branch_1");
        default::function_normal_branch_1(self, node_move)
    }

    fn visit_ident(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident");
        default::ident(self, node_move)
    }

    fn visit_ident_lower(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_lower");
        default::ident_lower(self, node_move)
    }

    fn visit_ident_lower_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_lower_scoped");
        default::ident_lower_scoped(self, node_move)
    }

    fn visit_ident_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_scoped");
        default::ident_scoped(self, node_move)
    }

    fn visit_ident_upper(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_upper");
        default::ident_upper(self, node_move)
    }

    fn visit_ident_upper_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::ident_upper_scoped");
        default::ident_upper_scoped(self, node_move)
    }

    fn visit_import(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::import");
        default::import(self, node_move)
    }

    fn visit_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::index");
        default::index(self, node_move)
    }

    fn visit_interpolation(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::interpolation");
        default::interpolation(self, node_move)
    }

    fn visit_item(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::item");
        default::item(self, node_move)
    }

    fn visit_key_primary(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::key_primary");
        default::key_primary(self, node_move)
    }

    fn visit_lit_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_bool");
        default::lit_bool(self, node_move)
    }

    fn visit_lit_map(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_map");
        default::lit_map(self, node_move)
    }

    fn visit_lit_num(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num");
        default::lit_num(self, node_move)
    }

    fn visit_lit_num_branch(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch");
        default::lit_num_branch(self, node_move)
    }

    fn visit_lit_num_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_0");
        default::lit_num_branch_0(self, node_move)
    }

    fn visit_lit_num_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_1");
        default::lit_num_branch_1(self, node_move)
    }

    fn visit_lit_num_branch_2(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_2");
        default::lit_num_branch_2(self, node_move)
    }

    fn visit_lit_num_branch_3(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_3");
        default::lit_num_branch_3(self, node_move)
    }

    fn visit_lit_num_branch_4(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_4");
        default::lit_num_branch_4(self, node_move)
    }

    fn visit_lit_num_branch_5(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_5");
        default::lit_num_branch_5(self, node_move)
    }

    fn visit_lit_num_branch_6(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_6");
        default::lit_num_branch_6(self, node_move)
    }

    fn visit_lit_num_branch_7(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_7");
        default::lit_num_branch_7(self, node_move)
    }

    fn visit_lit_num_branch_8(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_branch_8");
        default::lit_num_branch_8(self, node_move)
    }

    fn visit_lit_num_bin(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_bin");
        default::lit_num_bin(self, node_move)
    }

    fn visit_lit_num_dec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_dec");
        default::lit_num_dec(self, node_move)
    }

    fn visit_lit_num_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_float");
        default::lit_num_float(self, node_move)
    }

    fn visit_lit_num_hex(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_hex");
        default::lit_num_hex(self, node_move)
    }

    fn visit_lit_num_oct(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_num_oct");
        default::lit_num_oct(self, node_move)
    }

    fn visit_lit_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_string");
        default::lit_string(self, node_move)
    }

    fn visit_lit_vec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::lit_vec");
        default::lit_vec(self, node_move)
    }

    fn visit_misc_pat0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::misc_pat0");
        default::misc_pat0(self, node_move)
    }

    fn visit_module_alias(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::module_alias");
        default::module_alias(self, node_move)
    }

    fn visit_module_path(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::module_path");
        default::module_path(self, node_move)
    }

    fn visit_name(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name");
        default::name(self, node_move)
    }

    fn visit_name_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_arg");
        default::name_arg(self, node_move)
    }

    fn visit_name_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_cons");
        default::name_cons(self, node_move)
    }

    fn visit_name_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_field");
        default::name_field(self, node_move)
    }

    fn visit_name_func(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_func");
        default::name_func(self, node_move)
    }

    fn visit_name_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_index");
        default::name_index(self, node_move)
    }

    fn visit_name_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_rel");
        default::name_rel(self, node_move)
    }

    fn visit_name_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_trans");
        default::name_trans(self, node_move)
    }

    fn visit_name_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_type");
        default::name_type(self, node_move)
    }

    fn visit_name_var_term(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_var_term");
        default::name_var_term(self, node_move)
    }

    fn visit_name_var_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::name_var_type");
        default::name_var_type(self, node_move)
    }

    fn visit_pat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat");
        default::pat(self, node_move)
    }

    fn visit_pat_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_cons");
        default::pat_cons(self, node_move)
    }

    fn visit_pat_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_cons_pos");
        default::pat_cons_pos(self, node_move)
    }

    fn visit_pat_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_cons_rec");
        default::pat_cons_rec(self, node_move)
    }

    fn visit_pat_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_lit");
        default::pat_lit(self, node_move)
    }

    fn visit_pat_term_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_term_decl_var");
        default::pat_term_decl_var(self, node_move)
    }

    fn visit_pat_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_tuple");
        default::pat_tuple(self, node_move)
    }

    fn visit_pat_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_type");
        default::pat_type(self, node_move)
    }

    fn visit_pat_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::pat_wild");
        default::pat_wild(self, node_move)
    }

    fn visit_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel");
        default::rel(self, node_move)
    }

    fn visit_rel_args(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_args");
        default::rel_args(self, node_move)
    }

    fn visit_rel_elem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_elem");
        default::rel_elem(self, node_move)
    }

    fn visit_rel_role(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_role");
        default::rel_role(self, node_move)
    }

    fn visit_rel_semantics(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rel_semantics");
        default::rel_semantics(self, node_move)
    }

    fn visit_rhs(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs");
        default::rhs(self, node_move)
    }

    fn visit_rhs_atom_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_atom_neg");
        default::rhs_atom_neg(self, node_move)
    }

    fn visit_rhs_flat_map(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_flat_map");
        default::rhs_flat_map(self, node_move)
    }

    fn visit_rhs_grouping(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_grouping");
        default::rhs_grouping(self, node_move)
    }

    fn visit_rhs_inspect(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rhs_inspect");
        default::rhs_inspect(self, node_move)
    }

    fn visit_rule(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rule");
        default::rule(self, node_move)
    }

    fn visit_rule_end(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::rule_end");
        default::rule_end(self, node_move)
    }

    fn visit_statement(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement");
        default::statement(self, node_move)
    }

    fn visit_statement_assign(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_assign");
        default::statement_assign(self, node_move)
    }

    fn visit_statement_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_block");
        default::statement_block(self, node_move)
    }

    fn visit_statement_empty(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_empty");
        default::statement_empty(self, node_move)
    }

    fn visit_statement_for(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_for");
        default::statement_for(self, node_move)
    }

    fn visit_statement_if(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_if");
        default::statement_if(self, node_move)
    }

    fn visit_statement_insert(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_insert");
        default::statement_insert(self, node_move)
    }

    fn visit_statement_match(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::statement_match");
        default::statement_match(self, node_move)
    }

    fn visit_string_quoted(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted");
        default::string_quoted(self, node_move)
    }

    fn visit_string_quoted_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_branch_0");
        default::string_quoted_branch_0(self, node_move)
    }

    fn visit_string_quoted_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_branch_0");
        default::string_quoted_branch_1(self, node_move)
    }

    fn visit_string_quoted_escaped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_escaped");
        default::string_quoted_escaped(self, node_move)
    }

    fn visit_string_quoted_escaped_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_escaped_branch_0");
        default::string_quoted_escaped_branch_0(self, node_move)
    }

    fn visit_string_quoted_escaped_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_quoted_escaped_branch_1");
        default::string_quoted_escaped_branch_1(self, node_move)
    }

    fn visit_string_raw(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw");
        default::string_raw(self, node_move)
    }

    fn visit_string_raw_interpolated(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw_interpolated");
        default::string_raw_interpolated(self, node_move)
    }

    fn visit_string_raw_interpolated_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw_interpolated_branch_0");
        default::string_raw_interpolated_branch_0(self, node_move)
    }

    fn visit_string_raw_interpolated_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::string_raw_interpolated_branch_1");
        default::string_raw_interpolated_branch_1(self, node_move)
    }

    fn visit_transformer(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::transformer");
        default::transformer(self, node_move)
    }

    fn visit_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type");
        default::r#type(self, node_move)
    }

    fn visit_type_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_atom");
        default::type_atom(self, node_move)
    }

    fn visit_type_bigint(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_bigint");
        default::type_bigint(self, node_move)
    }

    fn visit_type_bit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_bit");
        default::type_bit(self, node_move)
    }

    fn visit_type_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_bool");
        default::type_bool(self, node_move)
    }

    fn visit_type_double(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_double");
        default::type_double(self, node_move)
    }

    fn visit_type_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_float");
        default::type_float(self, node_move)
    }

    fn visit_type_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_fun");
        default::type_fun(self, node_move)
    }

    fn visit_type_fun_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_fun_branch_0");
        default::type_fun_branch_0(self, node_move)
    }

    fn visit_type_fun_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_fun_branch_1");
        default::type_fun_branch_1(self, node_move)
    }

    fn visit_type_signed(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_signed");
        default::type_signed(self, node_move)
    }

    fn visit_type_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_string");
        default::type_string(self, node_move)
    }

    fn visit_type_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_trans");
        default::type_trans(self, node_move)
    }

    fn visit_type_trans_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_trans_fun");
        default::type_trans_fun(self, node_move)
    }

    fn visit_type_trans_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_trans_rel");
        default::type_trans_rel(self, node_move)
    }

    fn visit_type_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_tuple");
        default::type_tuple(self, node_move)
    }

    fn visit_type_union(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_union");
        default::type_union(self, node_move)
    }

    fn visit_type_user(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_user");
        default::type_user(self, node_move)
    }

    fn visit_type_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::type_var");
        default::type_var(self, node_move)
    }

    fn visit_typedef(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::typedef");
        default::typedef(self, node_move)
    }

    fn visit_typedef_extern(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::typedef_extern");
        default::typedef_extern(self, node_move)
    }

    fn visit_typedef_normal(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::typedef_normal");
        default::typedef_normal(self, node_move)
    }

    fn visit_word(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        // log::info!("visit::word");
        default::word(self, node_move)
    }
}
