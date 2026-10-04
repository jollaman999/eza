// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
//! The values that `ls` prints before each file name outside the long view.

use nu_ansi_term::Style;

use crate::fs::File;
#[cfg(unix)]
use crate::fs::FileTarget;
use crate::output::cell::TextCell;
use crate::theme::Theme;

/// What to print before each file name in the grid and lines views. Like
/// `ls`, `-s` prints the allocated size and `-Z` the security context, in
/// that order.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
pub struct Options {
    /// How to print the allocated size, if at all.
    pub blocks: Option<BlockFormat>,

    /// Whether to print the security context.
    pub context: bool,
}

/// How to print the allocated size of a file.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BlockFormat {
    /// The number of 1024-byte blocks, like `ls -s`.
    Kibibytes,

    /// A size with a binary prefix, like `ls -sh`.
    HumanReadable,
}

impl Options {
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.blocks.is_none() && !self.context
    }

    /// Renders the prefix of every file, in the same order. Like `ls`, each
    /// value is right-aligned to the widest value of its kind and followed
    /// by a space.
    #[must_use]
    pub fn render(self, files: &[File<'_>], theme: &Theme) -> Vec<String> {
        let mut columns: Vec<Vec<TextCell>> = Vec::new();

        if let Some(format) = self.blocks {
            let cells = files
                .iter()
                .map(|file| TextCell::paint(Style::default(), format.render(allocated(file))))
                .collect();
            columns.push(cells);
        }

        if self.context {
            let cells = files
                .iter()
                .map(|file| file.security_context().render(theme))
                .collect();
            columns.push(cells);
        }

        let mut prefixes = vec![String::new(); files.len()];
        for cells in &columns {
            pad_column(&mut prefixes, cells);
        }
        prefixes
    }
}

/// Appends a column of cells to the prefixes, right-aligned.
fn pad_column(prefixes: &mut [String], cells: &[TextCell]) {
    let width = cells.iter().map(|cell| *cell.width).max().unwrap_or(0);
    for (prefix, cell) in prefixes.iter_mut().zip(cells) {
        prefix.push_str(&" ".repeat(width - *cell.width));
        prefix.push_str(&cell.contents.strings().to_string());
        prefix.push(' ');
    }
}

impl BlockFormat {
    /// Formats a number of allocated bytes like `ls -s`, rounding up.
    #[must_use]
    pub fn render(self, bytes: u64) -> String {
        match self {
            Self::Kibibytes => bytes.div_ceil(1024).to_string(),
            Self::HumanReadable => human_readable(bytes),
        }
    }
}

/// Formats a size like `ls -h`: plain below 1024, and otherwise with a
/// binary prefix, one decimal below 10, rounding up.
fn human_readable(bytes: u64) -> String {
    const PREFIXES: [char; 8] = ['K', 'M', 'G', 'T', 'P', 'E', 'Z', 'Y'];

    if bytes < 1024 {
        return bytes.to_string();
    }

    let bytes = u128::from(bytes);
    let mut unit: u128 = 1024;
    let mut index = 0;
    while bytes >= unit * 1024 && index + 1 < PREFIXES.len() {
        unit *= 1024;
        index += 1;
    }

    let tenths = (bytes * 10).div_ceil(unit);
    if tenths < 100 {
        return format!("{}.{}{}", tenths / 10, tenths % 10, PREFIXES[index]);
    }

    let whole = bytes.div_ceil(unit);
    if whole >= 1024 && index + 1 < PREFIXES.len() {
        return format!("1.0{}", PREFIXES[index + 1]);
    }
    format!("{whole}{}", PREFIXES[index])
}

/// The number of bytes allocated to a file on disk, following symlinks when
/// dereferencing them.
#[cfg(unix)]
fn allocated(file: &File<'_>) -> u64 {
    use std::os::unix::fs::MetadataExt;

    if file.deref_links && file.is_link() {
        return match file.link_target_recurse() {
            FileTarget::Ok(target) => allocated(&target),
            _ => 0,
        };
    }
    file.metadata()
        .map_or(0, |metadata| metadata.blocks() * 512)
}

#[cfg(not(unix))]
fn allocated(file: &File<'_>) -> u64 {
    file.metadata().map_or(0, std::fs::Metadata::len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kibibytes_round_up() {
        for (bytes, expected) in [(0, "0"), (512, "1"), (1024, "1"), (4096, "4"), (4608, "5")] {
            assert_eq!(BlockFormat::Kibibytes.render(bytes), expected, "{bytes}");
        }
    }

    #[test]
    fn human_readable_like_ls() {
        for (bytes, expected) in [
            (0, "0"),
            (512, "512"),
            (1024, "1.0K"),
            (1536, "1.5K"),
            (4096, "4.0K"),
            (10_240, "10K"),
            (10_752, "11K"),
            (1_048_064, "1.0M"),
            (1_048_576, "1.0M"),
            (1_572_864, "1.5M"),
            (5_368_709_120, "5.0G"),
        ] {
            assert_eq!(
                BlockFormat::HumanReadable.render(bytes),
                expected,
                "{bytes}"
            );
        }
    }

    #[test]
    fn pad_column_right_aligns() {
        let mut prefixes = vec![String::new(); 2];
        let cell = |text: &str| TextCell::paint(Style::default(), String::from(text));
        pad_column(&mut prefixes, &[cell("4.0K"), cell("0")]);
        pad_column(&mut prefixes, &[cell("?"), cell("?")]);
        assert_eq!(prefixes, vec!["4.0K ? ", "   0 ? "]);
    }

    #[test]
    fn empty_options() {
        assert!(Options::default().is_empty());
        assert!(
            !Options {
                blocks: None,
                context: true
            }
            .is_empty()
        );
    }
}
