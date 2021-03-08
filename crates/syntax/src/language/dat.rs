//! Functions for working with the `.dat` grammar.

pub mod field {
    #![allow(missing_docs)]

    ddlog_lsp_macros::field_ids! {
        language: "ddlog.dat",
        fields: [
        ],
    }
}

pub mod kind {
    #![allow(missing_docs)]

    ddlog_lsp_macros::node_kind_ids! {
        language: "ddlog.dat",
        node_kinds: [
            (ATOM_ELEM, "atom_elem", true),
            (ATOM_POS, "atom_pos", true),
            (ATOM_REC, "atom_rec", true),
            (ATOM, "atom", true),
            (CLEAR, "clear", true),
            (COMMAND, "command", true),
            (COMMENT_LINE, "comment_line", true),
            (COMMIT, "commit", true),
            (CONS_ARG, "cons_arg", true),
            (CONS_ARGS, "cons_args", true),
            (DELETE_KEY, "delete_key", true),
            (DELETE, "delete", true),
            (DUMP_INDEX, "dump_index", true),
            (DUMP, "dump", true),
            (ECHO, "echo", true),
            (EXIT, "exit", true),
            (EXP, "exp", true),
            (INSERT_OR_UPDATE, "insert_or_update", true),
            (INSERT, "insert", true),
            (LIT_NUM_HEX, "lit_num_hex", true),
            (LIT_SERIALIZED, "lit_serialized", true),
            (LIT_STRING, "lit_string", true),
            (LOG_LEVEL, "log_level", true),
            (MODIFY, "modify", true),
            (PROFILE, "profile", true),
            (QUERY_INDEX, "query_index", true),
            (RECORD_NAMED, "record_named", true),
            (RECORD, "record", true),
            (ROLLBACK, "rollback", true),
            (ROOT, "root", true),
            (SERDE_ENCODING, "serde_encoding", true),
            (SLEEP, "sleep", true),
            (START, "start", true),
            (TIMESTAMP, "timestamp", true),
            (UPDATE, "update", true),
            (UPDATES, "updates", true),
            (VAL_ARRAY, "val_array", true),
            (VAL_STRUCT, "val_struct", true),
            (VAL_TUPLE, "val_tuple", true),
            (WORD, "word", true),
        ],
    }
}
