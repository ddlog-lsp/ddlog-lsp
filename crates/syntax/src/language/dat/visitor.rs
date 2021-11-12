use crate::{
    error::SyntaxError,
    language::{dat::kind, HasWalker, NodeMove},
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
        default::ROOT(self, node_move)
    }

    fn visit_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::arg(self, node_move)
    }

    fn visit_arg_opt_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::arg_opt_type(self, node_move)
    }

    fn visit_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::atom(self, node_move)
    }

    fn visit_atom_elem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::atom_elem(self, node_move)
    }

    fn visit_atom_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::atom_pos(self, node_move)
    }

    fn visit_atom_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::atom_rec(self, node_move)
    }

    fn visit_attribute(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::attribute(self, node_move)
    }

    fn visit_attributes(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::attributes(self, node_move)
    }

    fn visit_clear(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::clear(self, node_move)
    }

    fn visit_command(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::command(self, node_move)
    }

    fn visit_comment_line(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::comment_line(self, node_move)
    }

    fn visit_commit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::commit(self, node_move)
    }

    fn visit_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::cons(self, node_move)
    }

    fn visit_cons_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::cons_arg(self, node_move)
    }

    fn visit_cons_args(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::cons_args(self, node_move)
    }

    fn visit_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::cons_pos(self, node_move)
    }

    fn visit_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::cons_rec(self, node_move)
    }

    fn visit_delete(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::delete(self, node_move)
    }

    fn visit_delete_key(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::delete_key(self, node_move)
    }

    fn visit_dump(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::dump(self, node_move)
    }

    fn visit_dump_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::dump_index(self, node_move)
    }

    fn visit_echo(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::echo(self, node_move)
    }

    fn visit_escape_sequence(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::escape_sequence(self, node_move)
    }

    fn visit_escape_sequence_interpolated(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::escape_sequence_interpolated(self, node_move)
    }

    fn visit_exit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exit(self, node_move)
    }

    fn visit_exp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp(self, node_move)
    }

    fn visit_exp_add(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_add(self, node_move)
    }

    fn visit_exp_assign(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_assign(self, node_move)
    }

    fn visit_exp_binding(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_binding(self, node_move)
    }

    fn visit_exp_bit_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_bit_and(self, node_move)
    }

    fn visit_exp_bit_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_bit_neg(self, node_move)
    }

    fn visit_exp_bit_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_bit_or(self, node_move)
    }

    fn visit_exp_bit_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_bit_slice(self, node_move)
    }

    fn visit_exp_bit_xor(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_bit_xor(self, node_move)
    }

    fn visit_exp_block(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_block(self, node_move)
    }

    fn visit_exp_break(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_break(self, node_move)
    }

    fn visit_exp_cast(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_cast(self, node_move)
    }

    fn visit_exp_cat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_cat(self, node_move)
    }

    fn visit_exp_cond(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_cond(self, node_move)
    }

    fn visit_exp_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_cons_pos(self, node_move)
    }

    fn visit_exp_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_cons_rec(self, node_move)
    }

    fn visit_exp_continue(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_continue(self, node_move)
    }

    fn visit_exp_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_decl_var(self, node_move)
    }

    fn visit_exp_div(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_div(self, node_move)
    }

    fn visit_exp_eq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_eq(self, node_move)
    }

    fn visit_exp_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_field(self, node_move)
    }

    fn visit_exp_for(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_for(self, node_move)
    }

    fn visit_exp_fun_call(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_fun_call(self, node_move)
    }

    fn visit_exp_fun_call_dot(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_fun_call_dot(self, node_move)
    }

    fn visit_exp_gt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_gt(self, node_move)
    }

    fn visit_exp_gteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_gteq(self, node_move)
    }

    fn visit_exp_lambda(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_lambda(self, node_move)
    }

    fn visit_exp_lambda_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_lambda_branch_0(self, node_move)
    }

    fn visit_exp_lambda_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_lambda_branch_1(self, node_move)
    }

    fn visit_exp_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_lit(self, node_move)
    }

    fn visit_exp_log_and(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_log_and(self, node_move)
    }

    fn visit_exp_log_imp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_log_imp(self, node_move)
    }

    fn visit_exp_log_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_log_neg(self, node_move)
    }

    fn visit_exp_log_or(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_log_or(self, node_move)
    }

    fn visit_exp_lt(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_lt(self, node_move)
    }

    fn visit_exp_lteq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_lteq(self, node_move)
    }

    fn visit_exp_match(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_match(self, node_move)
    }

    fn visit_exp_mul(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_mul(self, node_move)
    }

    fn visit_exp_neg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_neg(self, node_move)
    }

    fn visit_exp_neq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_neq(self, node_move)
    }

    fn visit_exp_proj(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_proj(self, node_move)
    }

    fn visit_exp_proj_digits(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_proj_digits(self, node_move)
    }

    fn visit_exp_ref(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_ref(self, node_move)
    }

    fn visit_exp_rem(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_rem(self, node_move)
    }

    fn visit_exp_return(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_return(self, node_move)
    }

    fn visit_exp_seq(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_seq(self, node_move)
    }

    fn visit_exp_shl(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_shl(self, node_move)
    }

    fn visit_exp_shr(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_shr(self, node_move)
    }

    fn visit_exp_slice(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_slice(self, node_move)
    }

    fn visit_exp_sub(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_sub(self, node_move)
    }

    fn visit_exp_try(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_try(self, node_move)
    }

    fn visit_exp_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_tuple(self, node_move)
    }

    fn visit_exp_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_type(self, node_move)
    }

    fn visit_exp_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_wild(self, node_move)
    }

    fn visit_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::exp_field(self, node_move)
    }

    fn visit_ident(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::ident(self, node_move)
    }

    fn visit_ident_lower(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::ident_lower(self, node_move)
    }

    fn visit_ident_lower_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::ident_lower_scoped(self, node_move)
    }

    fn visit_ident_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::ident_scoped(self, node_move)
    }

    fn visit_ident_upper(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::ident_upper(self, node_move)
    }

    fn visit_ident_upper_scoped(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::ident_upper_scoped(self, node_move)
    }

    fn visit_insert(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::insert(self, node_move)
    }

    fn visit_insert_or_update(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::insert_or_update(self, node_move)
    }

    fn visit_interpolation(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::interpolation(self, node_move)
    }

    fn visit_lit_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_bool(self, node_move)
    }

    fn visit_lit_map(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_map(self, node_move)
    }

    fn visit_lit_num(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num(self, node_move)
    }

    fn visit_lit_num_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_0(self, node_move)
    }

    fn visit_lit_num_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_1(self, node_move)
    }

    fn visit_lit_num_branch_10(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_10(self, node_move)
    }

    fn visit_lit_num_branch_11(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_11(self, node_move)
    }

    fn visit_lit_num_branch_12(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_12(self, node_move)
    }

    fn visit_lit_num_branch_13(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_13(self, node_move)
    }

    fn visit_lit_num_branch_14(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_14(self, node_move)
    }

    fn visit_lit_num_branch_15(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_15(self, node_move)
    }

    fn visit_lit_num_branch_16(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_16(self, node_move)
    }

    fn visit_lit_num_branch_17(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_17(self, node_move)
    }

    fn visit_lit_num_branch_18(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_branch_18(self, node_move)
    }

    fn visit_lit_num_bin(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_bin(self, node_move)
    }

    fn visit_lit_num_dec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_dec(self, node_move)
    }

    fn visit_lit_num_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_float(self, node_move)
    }

    fn visit_lit_num_hex(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_hex(self, node_move)
    }

    fn visit_lit_num_oct(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_num_oct(self, node_move)
    }

    fn visit_lit_serialized(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_serialized(self, node_move)
    }

    fn visit_lit_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_string(self, node_move)
    }

    fn visit_lit_vec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::lit_vec(self, node_move)
    }

    fn visit_log_level(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::log_level(self, node_move)
    }

    fn visit_misc_pat0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::misc_pat0(self, node_move)
    }

    fn visit_modify(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::modify(self, node_move)
    }

    fn visit_name(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name(self, node_move)
    }

    fn visit_name_arg(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_arg(self, node_move)
    }

    fn visit_name_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_cons(self, node_move)
    }

    fn visit_name_field(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_field(self, node_move)
    }

    fn visit_name_func(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_func(self, node_move)
    }

    fn visit_name_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_index(self, node_move)
    }

    fn visit_name_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_rel(self, node_move)
    }

    fn visit_name_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_type(self, node_move)
    }

    fn visit_name_var_term(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::name_var_term(self, node_move)
    }

    fn visit_pat(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat(self, node_move)
    }

    fn visit_pat_cons(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_cons(self, node_move)
    }

    fn visit_pat_cons_pos(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_cons_pos(self, node_move)
    }

    fn visit_pat_cons_rec(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_cons_rec(self, node_move)
    }

    fn visit_pat_lit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_lit(self, node_move)
    }

    fn visit_pat_term_decl_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_term_decl_var(self, node_move)
    }

    fn visit_pat_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_tuple(self, node_move)
    }

    fn visit_pat_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_type(self, node_move)
    }

    fn visit_pat_wild(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::pat_wild(self, node_move)
    }

    fn visit_profile(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::profile(self, node_move)
    }

    fn visit_query_index(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::query_index(self, node_move)
    }

    fn visit_record(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::record(self, node_move)
    }

    fn visit_record_named(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::record_named(self, node_move)
    }

    fn visit_rollback(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::rollback(self, node_move)
    }

    fn visit_serde_encoding(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::serde_encoding(self, node_move)
    }

    fn visit_sleep(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::sleep(self, node_move)
    }

    fn visit_start(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::start(self, node_move)
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

    fn visit_timestamp(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::timestamp(self, node_move)
    }

    fn visit_type(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::r#type(self, node_move)
    }

    fn visit_type_atom(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_atom(self, node_move)
    }

    fn visit_type_bigint(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_bigint(self, node_move)
    }

    fn visit_type_bit(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_bit(self, node_move)
    }

    fn visit_type_bool(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_bool(self, node_move)
    }

    fn visit_type_double(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_double(self, node_move)
    }

    fn visit_type_float(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_float(self, node_move)
    }

    fn visit_type_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_fun(self, node_move)
    }

    fn visit_type_fun_branch_0(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_fun_branch_0(self, node_move)
    }

    fn visit_type_fun_branch_1(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_fun_branch_1(self, node_move)
    }

    fn visit_type_signed(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_signed(self, node_move)
    }

    fn visit_type_string(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_string(self, node_move)
    }

    fn visit_type_trans(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_trans(self, node_move)
    }

    fn visit_type_trans_fun(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_trans_fun(self, node_move)
    }

    fn visit_type_trans_rel(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_trans_rel(self, node_move)
    }

    fn visit_type_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_tuple(self, node_move)
    }

    fn visit_type_union(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_union(self, node_move)
    }

    fn visit_type_user(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_user(self, node_move)
    }

    fn visit_type_var(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::type_var(self, node_move)
    }

    fn visit_update(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::update(self, node_move)
    }

    fn visit_updates(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::updates(self, node_move)
    }

    fn visit_updates_end(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::updates_end(self, node_move)
    }

    fn visit_val_array(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::val_array(self, node_move)
    }

    fn visit_val_struct(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::val_struct(self, node_move)
    }

    fn visit_val_tuple(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::val_tuple(self, node_move)
    }

    fn visit_word(&mut self, node_move: NodeMove) -> Result<(), SyntaxError<()>> {
        default::word(self, node_move)
    }
}
