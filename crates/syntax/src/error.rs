use ddlog_lsp_languages::language::Language;
use lsp_text::RopeExt;

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxError<D> {
    language: Language,
    range: tree_sitter::Range,
    r#type: SyntaxErrorType,
    data: D,
}

#[allow(missing_docs)]
impl<D> SyntaxError<D> {
    pub fn choice_failure(language: Language, range: tree_sitter::Range, choices: Vec<u16>, data: D) -> Self {
        let r#type = SyntaxErrorType::ChoiceError { language, choices };
        Self {
            language,
            range,
            r#type,
            data,
        }
    }

    pub fn walker_done_early(language: Language, range: tree_sitter::Range, data: D) -> Self {
        let r#type = SyntaxErrorType::WalkerDoneEarlyError;
        Self {
            language,
            range,
            r#type,
            data,
        }
    }

    pub fn walker_move_error(language: Language, range: tree_sitter::Range, data: D) -> Self {
        let r#type = SyntaxErrorType::WalkerMoveError;
        Self {
            language,
            range,
            r#type,
            data,
        }
    }

    pub fn node_mismatch_error(
        language: Language,
        range: tree_sitter::Range,
        dest_kind: u16,
        want_kind: u16,
        data: D,
    ) -> Self {
        let r#type = SyntaxErrorType::NodeMismatchError {
            language,
            dest_kind,
            want_kind,
        };
        Self {
            language,
            range,
            r#type,
            data,
        }
    }

    pub fn node_missing_error(language: Language, range: tree_sitter::Range, want_kind: u16, data: D) -> Self {
        let r#type = SyntaxErrorType::NodeMissingError { language, want_kind };
        Self {
            language,
            range,
            r#type,
            data,
        }
    }

    #[allow(clippy::field_reassign_with_default)]
    pub fn to_lsp_diagnostic(&self, uri: &lsp::Url, content: &ropey::Rope) -> lsp::Diagnostic {
        let range = content.tree_sitter_range_to_lsp_range(self.range);
        let severity = Some(lsp::DiagnosticSeverity::ERROR);
        match &self.r#type {
            SyntaxErrorType::ChoiceError { language, choices } => {
                let language: tree_sitter::Language = (*language).into();
                let choices = choices
                    .iter()
                    .map(|id| language.node_kind_for_id(*id).unwrap().into_owned())
                    .collect::<Vec<_>>()
                    .join(", ");
                let mut diagnostic = lsp::Diagnostic::default();
                diagnostic.message = format!("syntax error: expected one of [{}]", choices);
                diagnostic.range = range;
                diagnostic.severity = severity;
                diagnostic
            },
            SyntaxErrorType::WalkerDoneEarlyError => {
                let mut diagnostic = lsp::Diagnostic::default();
                diagnostic.message = String::from("syntax error: internal error (parsing terminated too early)");
                diagnostic.range = range;
                diagnostic.severity = severity;
                diagnostic
            },
            SyntaxErrorType::WalkerMoveError => {
                let mut diagnostic = lsp::Diagnostic::default();
                diagnostic.message = String::from("syntax error: internal error (failed to move to next node)");
                diagnostic.range = range;
                diagnostic.severity = severity;
                diagnostic
            },
            SyntaxErrorType::NodeMismatchError {
                language,
                dest_kind,
                want_kind,
            } => {
                let language: tree_sitter::Language = (*language).into();
                let mut diagnostic = lsp::Diagnostic::default();
                diagnostic.message = String::from("syntax node mismatch error");
                diagnostic.range = range;
                diagnostic.severity = severity;
                diagnostic.related_information = {
                    let dest_info = {
                        let location = {
                            let uri = uri.clone();
                            let range = range;
                            lsp::Location { uri, range }
                        };
                        let message = format!("found: {}", language.node_kind_for_id(*dest_kind).unwrap());
                        lsp::DiagnosticRelatedInformation { location, message }
                    };
                    let want_info = {
                        let location = {
                            let uri = uri.clone();
                            let range = range;
                            lsp::Location { uri, range }
                        };
                        let message = format!("expected: {}", language.node_kind_for_id(*want_kind).unwrap());
                        lsp::DiagnosticRelatedInformation { location, message }
                    };
                    Some(vec![dest_info, want_info])
                };
                diagnostic
            },
            SyntaxErrorType::NodeMissingError { language, want_kind } => {
                let language: tree_sitter::Language = (*language).into();
                let mut diagnostic = lsp::Diagnostic::default();
                diagnostic.message = String::from("SyntaxErrorType::NodeMissingError");
                diagnostic.range = range;
                diagnostic.severity = severity;
                diagnostic.related_information = {
                    let want_info = {
                        let location = {
                            let uri = uri.clone();
                            let range = range;
                            lsp::Location { uri, range }
                        };
                        let message = format!("expected: {}", language.node_kind_for_id(*want_kind).unwrap());
                        lsp::DiagnosticRelatedInformation { location, message }
                    };
                    Some(vec![want_info])
                };
                diagnostic
            },
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, PartialEq)]
pub enum SyntaxErrorType {
    ChoiceError {
        language: Language,
        choices: Vec<u16>,
    },
    WalkerDoneEarlyError,
    WalkerMoveError,
    NodeMismatchError {
        language: Language,
        dest_kind: u16,
        want_kind: u16,
    },
    NodeMissingError {
        language: Language,
        want_kind: u16,
    },
}

impl std::fmt::Debug for SyntaxErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SyntaxErrorType::ChoiceError { language, choices } => {
                let language: tree_sitter::Language = (*language).into();
                f.debug_struct("ChoiceError")
                    .field(
                        "choices",
                        &choices
                            .iter()
                            .map(|id| language.node_kind_for_id(*id).unwrap().into_owned())
                            .collect::<Vec<_>>(),
                    )
                    .finish()
            },
            SyntaxErrorType::WalkerDoneEarlyError => f.debug_tuple("WalkerDoneEarlyError").finish(),
            SyntaxErrorType::WalkerMoveError => f.debug_tuple("WalkerMoveError").finish(),
            SyntaxErrorType::NodeMismatchError {
                language,
                dest_kind,
                want_kind,
            } => {
                let language: tree_sitter::Language = (*language).into();
                f.debug_struct("NodeMismatchError")
                    .field("dest_kind", &language.node_kind_for_id(*dest_kind).unwrap())
                    .field("want_kind", &language.node_kind_for_id(*want_kind).unwrap())
                    .finish()
            },
            SyntaxErrorType::NodeMissingError { language, want_kind } => {
                let language: tree_sitter::Language = (*language).into();
                f.debug_struct("NodeMissingError")
                    .field("want_kind", &language.node_kind_for_id(*want_kind).unwrap())
                    .finish()
            },
        }
    }
}
