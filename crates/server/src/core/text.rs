use crate::core::{self, RopeExt};
use std::convert::{TryFrom, TryInto};

pub struct Text {
    pub language: core::Language,
    pub content: ropey::Rope,
}

impl Text {
    pub fn new(
        language_id: impl TryInto<core::Language, Error = anyhow::Error>,
        text: impl AsRef<str>,
    ) -> anyhow::Result<Self> {
        let text = text.as_ref();
        let language = language_id.try_into()?;
        let content = ropey::Rope::from_str(text);
        Ok(Text { language, content })
    }

    pub fn build_edit<'a>(&self, change: &'a lsp::TextDocumentContentChangeEvent) -> anyhow::Result<TextEdit<'a>> {
        let text = change.text.as_str();
        let text_bytes = text.as_bytes();
        let text_end_byte_idx = text_bytes.len();

        let range = if let Some(range) = change.range {
            range
        } else {
            let start = self.content.byte_to_lsp_position(0);
            let end = self.content.byte_to_lsp_position(text_end_byte_idx);
            lsp::Range { start, end }
        };

        let start = self.content.lsp_position_to_core(range.start)?;
        let old_end = self.content.lsp_position_to_core(range.end)?;

        let new_end_byte = start.byte as usize + text_end_byte_idx;
        let new_end_position = self.content.byte_to_tree_sitter_point(new_end_byte)?;

        let input_edit = {
            let start_byte = start.byte;
            let old_end_byte = old_end.byte;
            let new_end_byte = u32::try_from(new_end_byte)?;
            let start_position = start.point;
            let old_end_position = old_end.point;
            tree_sitter::InputEdit::new(
                start_byte,
                old_end_byte,
                new_end_byte,
                &start_position,
                &old_end_position,
                &new_end_position,
            )
        };

        Ok(TextEdit {
            input_edit,
            start_char_idx: start.char as usize,
            end_char_idx: old_end.char as usize,
            text,
        })
    }

    pub fn apply_edit(&mut self, edit: &TextEdit) {
        self.content.remove(edit.start_char_idx .. edit.end_char_idx);
        if !edit.text.is_empty() {
            self.content.insert(edit.start_char_idx, &edit.text);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub char: u32,
    pub byte: u32,
    pub code: u32,
    pub point: tree_sitter::Point,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEdit<'a> {
    pub input_edit: tree_sitter::InputEdit,
    pub start_char_idx: usize,
    pub end_char_idx: usize,
    pub text: &'a str,
}

impl<'a> TextEdit<'a> {
    pub fn range(&self) -> tree_sitter::Range {
        let start_byte = self.input_edit.start_byte();
        let end_byte = self.input_edit.new_end_byte();
        let start_point = &self.input_edit.start_position();
        let end_point = &self.input_edit.new_end_position();
        tree_sitter::Range::new(start_byte, end_byte, start_point, end_point)
    }
}
