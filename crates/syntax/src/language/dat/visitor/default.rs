use crate::{
    error::SyntaxError,
    language::{
        dat::{keyword, kind, symbol, visitor::Visitor},
        utils,
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

pub fn escape_sequence_interpolated<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
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
        (kind::LIT_NUM_DEC, Vis::visit_lit_num_dec),
        (kind::LIT_NUM_BRANCH, Vis::visit_lit_num_branch),
    ))(visitor, NodeMove::Step)
}

pub fn lit_num_branch<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_1, node_move, GotoNext::StepInto)?;
    utils::seq((
        utils::optional(Vis::visit_lit_num_dec),
        utils::choice((
            (kind::LIT_NUM_BRANCH_0, Vis::visit_lit_num_branch_0),
            (kind::LIT_NUM_BRANCH_1, Vis::visit_lit_num_branch_1),
            (kind::LIT_NUM_BRANCH_2, Vis::visit_lit_num_branch_2),
            (kind::LIT_NUM_BRANCH_3, Vis::visit_lit_num_branch_3),
            (kind::LIT_NUM_BRANCH_4, Vis::visit_lit_num_branch_4),
            (kind::LIT_NUM_BRANCH_5, Vis::visit_lit_num_branch_5),
            (kind::LIT_NUM_BRANCH_6, Vis::visit_lit_num_branch_6),
            (kind::LIT_NUM_BRANCH_7, Vis::visit_lit_num_branch_7),
            (kind::LIT_NUM_BRANCH_8, Vis::visit_lit_num_branch_8),
        )),
    ))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_1, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_BIN), Vis::visit_lit_num_bin))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_1<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_1, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_DEC), Vis::visit_lit_num_dec))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_2<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_2, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_FLOAT), Vis::visit_lit_num_float))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_3<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_3, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_HEX), Vis::visit_lit_num_hex))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_4<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_4, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_OCT), Vis::visit_lit_num_oct))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_5<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_5, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_S_BIN), Vis::visit_lit_num_bin))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_6<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_6, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_S_DEC), Vis::visit_lit_num_dec))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_7<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_7, node_move, GotoNext::StepInto)?;
    utils::seq((utils::token(symbol::LIT_S_HEX), Vis::visit_lit_num_hex))(visitor, NodeMove::Step)
}

pub fn lit_num_branch_8<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::LIT_NUM_BRANCH_8, node_move, GotoNext::StepInto)?;
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

pub fn string_quoted_escaped_branch_0<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
where
    Vis: Visitor<'tree> + ?Sized,
{
    visitor
        .walker()
        .step(kind::STRING_QUOTED_ESCAPED_BRANCH_0, node_move, GotoNext::StepInto)?;
    Ok(())
}

pub fn string_quoted_escaped_branch_1<'tree, Vis>(visitor: &mut Vis, node_move: NodeMove) -> Result<(), SyntaxError<()>>
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
