// SPDX-FileCopyrightText: 2021 ilkecan <ilkecan@protonmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! These functions are useful for inserting a multiline string into an already indented context in
//! another string.

use std::borrow::Cow;

/// Indents every line that is not empty by the given number of spaces, starting from the second
/// line.
///
/// The first line of the string is not indented so that it can be placed after an introduction
/// sequence that has already begun the line.
///
/// # Examples
/// ```rust
/// assert_eq!(format!("  items: {}", indent::indent_by(2, "[\n  foo,\n  bar,\n]\n")),
/// "  items: [
///     foo,
///     bar,
///   ]
/// ")
/// ```
///
/// For the version that also indents the first line, see [indent_all_by].
pub fn indent_by<'a, S>(number_of_spaces: usize, input: S) -> String
where
    S: Into<Cow<'a, str>>,
{
    indent(" ".repeat(number_of_spaces), input, false)
}

/// Indents every line that is not empty with the given prefix, starting from the second line.
///
/// The first line of the string is not indented so that it can be placed after an introduction
/// sequence that has already begun the line.
///
/// # Examples
/// ```rust
/// assert_eq!(format!("items:{}", indent::indent_with("- ", "\nfoo\nbar\n")),
/// "items:
/// - foo
/// - bar
/// ")
/// ```
///
/// For the version that also indents the first line, see [indent_all_with].
pub fn indent_with<'a, S, T>(prefix: S, input: T) -> String
where
    S: Into<Cow<'a, str>>,
    T: Into<Cow<'a, str>>,
{
    indent(prefix, input, false)
}

/// Indents every line that is not empty by the given number of spaces.
///
/// # Examples
/// ```rust
/// assert_eq!(format!("items: [\n{}]\n", indent::indent_all_by(2, "foo,\nbar,\n")),
/// "items: [
///   foo,
///   bar,
/// ]
/// ")
/// ```
///
/// For the version that doesn't indent the first line, see [indent_by].
pub fn indent_all_by<'a, S>(number_of_spaces: usize, input: S) -> String
where
    S: Into<Cow<'a, str>>,
{
    indent(" ".repeat(number_of_spaces), input, true)
}

/// Indents every line that is not empty with the given prefix.
///
/// # Examples
/// ```rust
/// assert_eq!(format!("items:\n{}", indent::indent_all_with("- ", "foo\nbar\n")),
/// "items:
/// - foo
/// - bar
/// ")
/// ```
///
/// For the version that also indents the first line, see [indent_with].
pub fn indent_all_with<'a, S, T>(prefix: S, input: T) -> String
where
    S: Into<Cow<'a, str>>,
    T: Into<Cow<'a, str>>,
{
    indent(prefix, input, true)
}

fn indent<'a, S, T>(prefix: S, input: T, indent_all: bool) -> String
where
    S: Into<Cow<'a, str>>,
    T: Into<Cow<'a, str>>,
{
    let prefix = prefix.into();
    let input = input.into();
    let length = input.len();
    let mut output = String::with_capacity(length + length / 2);

    for (i, line) in input.lines().enumerate() {
        if i > 0 {
            output.push('\n');

            if !line.is_empty() {
                output.push_str(&prefix);
            }
        } else if indent_all && !line.is_empty() {
            output.push_str(&prefix);
        }

        output.push_str(line);
    }

    if input.ends_with('\n') {
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_without_newline() {
        assert_eq!(indent_by(2, "foo"), "foo")
    }

    #[test]
    fn test_single_line_with_newline() {
        assert_eq!(indent_by(2, "foo\n"), "foo\n")
    }

    #[test]
    fn test_multiline_without_newline() {
        assert_eq!(
            indent_by(
                2, "
foo
bar"
            ),
            "
  foo
  bar"
        )
    }

    #[test]
    fn test_multiline_with_newline() {
        assert_eq!(
            indent_by(
                2,
                "
foo
bar
"
            ),
            "
  foo
  bar
"
        )
    }

    #[test]
    fn test_empty_line() {
        assert_eq!(
            indent_by(
                2,
                "
foo

bar
"
            ),
            "
  foo

  bar
"
        )
    }

    #[test]
    fn test_indent_all_by_empty_line() {
        assert_eq!(
            indent_all_by(
                2,
                "
foo

bar"
            ),
            "
  foo

  bar"
        )
    }

    #[test]
    fn test_indent_all_by() {
        assert_eq!(
            indent_all_by(
                2, "foo

bar"
            ),
            "  foo

  bar"
        )
    }

    #[test]
    fn test_indent_with() {
        assert_eq!(
            indent_with(
                "  ",
                "
foo

bar
"
            ),
            "
  foo

  bar
"
        )
    }

    #[test]
    fn test_indent_all_with() {
        assert_eq!(
            indent_all_with(
                "  ", "foo

bar"
            ),
            "  foo

  bar"
        )
    }
}
