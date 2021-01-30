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

        let start_char_idx = {
            let start_offset = self.content.lsp_position_to_utf16_cu(range.start)? as usize;
            self.content.utf16_cu_to_char(start_offset)
        };

        let end_char_idx = {
            let end_offset = self.content.lsp_position_to_utf16_cu(range.end)? as usize;
            self.content.utf16_cu_to_char(end_offset)
        };

        let start_byte = self.content.char_to_byte(start_char_idx);
        let old_end_byte = {
            let end_char = self.content.char(end_char_idx);
            self.content.char_to_byte(end_char_idx) + end_char.len_utf8()
        };
        let new_end_byte = start_byte + text_end_byte_idx;

        let start_position = self.content.byte_to_tree_sitter_point(start_byte);
        let old_end_position = self.content.byte_to_tree_sitter_point(old_end_byte);
        let new_end_position = {
            let mut last_line = change.text.as_str();
            let mut line_count = 0;

            for line in change.text.lines() {
                last_line = line;
                line_count += 1;
            }

            if !change.text.is_empty() {
                line_count -= 1;
            }

            let row = start_position.row() + line_count;
            let column = {
                let padding = if line_count > 0 {
                    0
                } else {
                    start_position.column() as usize
                };
                let result = padding + last_line.as_bytes().len();
                u32::try_from(result).unwrap()
            };
            tree_sitter::Point::new(row, column)
        };

        let input_edit = {
            let start_byte = u32::try_from(start_byte).unwrap();
            let old_end_byte = u32::try_from(old_end_byte).unwrap();
            let new_end_byte = u32::try_from(new_end_byte).unwrap();
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
            start_char_idx,
            end_char_idx,
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