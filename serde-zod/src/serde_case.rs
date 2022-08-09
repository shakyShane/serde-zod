//! Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the
//! case of the source (e.g. `my-field`, `MY_FIELD`).

// See https://users.rust-lang.org/t/psa-dealing-with-warning-unused-import-std-ascii-asciiext-in-today-s-nightly/13726
#[allow(deprecated, unused_imports)]
use std::ascii::AsciiExt;

use std::fmt::{self, Debug, Display};

use self::LocalRenameRule::*;

/// The different possible ways to change case of fields in a struct, or variants in an enum.
#[derive(Copy, Clone, PartialEq)]
pub enum LocalRenameRule {
    /// Don't apply a default rename rule.
    #[allow(dead_code)]
    None,
    /// Rename direct children to "lowercase" style.
    LowerCase,
    /// Rename direct children to "UPPERCASE" style.
    UpperCase,
    /// Rename direct children to "PascalCase" style, as typically used for
    /// enum variants.
    PascalCase,
    /// Rename direct children to "camelCase" style.
    CamelCase,
    /// Rename direct children to "snake_case" style, as commonly used for
    /// fields.
    SnakeCase,
    /// Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly
    /// used for constants.
    ScreamingSnakeCase,
    /// Rename direct children to "kebab-case" style.
    KebabCase,
    /// Rename direct children to "SCREAMING-KEBAB-CASE" style.
    ScreamingKebabCase,
}

static LOCAL_RENAME_RULES: &[(&str, LocalRenameRule)] = &[
    ("lowercase", LowerCase),
    ("UPPERCASE", UpperCase),
    ("PascalCase", PascalCase),
    ("camelCase", CamelCase),
    ("snake_case", SnakeCase),
    ("SCREAMING_SNAKE_CASE", ScreamingSnakeCase),
    ("kebab-case", KebabCase),
    ("SCREAMING-KEBAB-CASE", ScreamingKebabCase),
];

impl LocalRenameRule {
    #[allow(dead_code)]
    pub fn from_str(rename_all_str: &str) -> Result<Self, LocalParseError> {
        for (name, rule) in LOCAL_RENAME_RULES {
            if rename_all_str == *name {
                return Ok(*rule);
            }
        }
        Err(LocalParseError {
            unknown: rename_all_str,
        })
    }

    /// Apply a renaming rule to an enum variant, returning the version expected in the source.
    #[allow(dead_code)]
    pub fn local_apply_to_variant(&self, variant: &str) -> String {
        match *self {
            None | PascalCase => variant.to_owned(),
            LowerCase => variant.to_ascii_lowercase(),
            UpperCase => variant.to_ascii_uppercase(),
            CamelCase => variant[..1].to_ascii_lowercase() + &variant[1..],
            SnakeCase => {
                let mut snake = String::new();
                for (i, ch) in variant.char_indices() {
                    if i > 0 && ch.is_uppercase() {
                        snake.push('_');
                    }
                    snake.push(ch.to_ascii_lowercase());
                }
                snake
            }
            ScreamingSnakeCase => SnakeCase
                .local_apply_to_variant(variant)
                .to_ascii_uppercase(),
            KebabCase => SnakeCase.local_apply_to_variant(variant).replace('_', "-"),
            ScreamingKebabCase => ScreamingSnakeCase
                .local_apply_to_variant(variant)
                .replace('_', "-"),
        }
    }

    /// Apply a renaming rule to a struct field, returning the version expected in the source.
    #[allow(dead_code)]
    pub fn local_apply_to_field(&self, field: &str) -> String {
        match *self {
            None | LowerCase | SnakeCase => field.to_owned(),
            UpperCase => field.to_ascii_uppercase(),
            PascalCase => {
                let mut pascal = String::new();
                let mut capitalize = true;
                for ch in field.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        pascal.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        pascal.push(ch);
                    }
                }
                pascal
            }
            CamelCase => {
                let pascal = PascalCase.local_apply_to_field(field);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            ScreamingSnakeCase => field.to_ascii_uppercase(),
            KebabCase => field.replace('_', "-"),
            ScreamingKebabCase => ScreamingSnakeCase
                .local_apply_to_field(field)
                .replace('_', "-"),
        }
    }
}

pub struct LocalParseError<'a> {
    unknown: &'a str,
}

impl<'a> Display for LocalParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unknown rename rule `rename_all = ")?;
        Debug::fmt(self.unknown, f)?;
        f.write_str("`, expected one of ")?;
        for (i, (name, _rule)) in LOCAL_RENAME_RULES.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            Debug::fmt(name, f)?;
        }
        Ok(())
    }
}

#[test]
fn local_rename_variants() {
    for &(original, lower, upper, camel, snake, screaming, kebab, screaming_kebab) in &[
        (
            "Outcome", "outcome", "OUTCOME", "outcome", "outcome", "OUTCOME", "outcome", "OUTCOME",
        ),
        (
            "VeryTasty",
            "verytasty",
            "VERYTASTY",
            "veryTasty",
            "very_tasty",
            "VERY_TASTY",
            "very-tasty",
            "VERY-TASTY",
        ),
        ("A", "a", "A", "a", "a", "A", "a", "A"),
        ("Z42", "z42", "Z42", "z42", "z42", "Z42", "z42", "Z42"),
    ] {
        assert_eq!(None.local_apply_to_variant(original), original);
        assert_eq!(LowerCase.local_apply_to_variant(original), lower);
        assert_eq!(UpperCase.local_apply_to_variant(original), upper);
        assert_eq!(PascalCase.local_apply_to_variant(original), original);
        assert_eq!(CamelCase.local_apply_to_variant(original), camel);
        assert_eq!(SnakeCase.local_apply_to_variant(original), snake);
        assert_eq!(
            ScreamingSnakeCase.local_apply_to_variant(original),
            screaming
        );
        assert_eq!(KebabCase.local_apply_to_variant(original), kebab);
        assert_eq!(
            ScreamingKebabCase.local_apply_to_variant(original),
            screaming_kebab
        );
    }
}

#[test]
fn local_rename_fields() {
    for &(original, upper, pascal, camel, screaming, kebab, screaming_kebab) in &[
        (
            "outcome", "OUTCOME", "Outcome", "outcome", "OUTCOME", "outcome", "OUTCOME",
        ),
        (
            "very_tasty",
            "VERY_TASTY",
            "VeryTasty",
            "veryTasty",
            "VERY_TASTY",
            "very-tasty",
            "VERY-TASTY",
        ),
        ("a", "A", "A", "a", "A", "a", "A"),
        ("z42", "Z42", "Z42", "z42", "Z42", "z42", "Z42"),
    ] {
        assert_eq!(None.local_apply_to_field(original), original);
        assert_eq!(UpperCase.local_apply_to_field(original), upper);
        assert_eq!(PascalCase.local_apply_to_field(original), pascal);
        assert_eq!(CamelCase.local_apply_to_field(original), camel);
        assert_eq!(SnakeCase.local_apply_to_field(original), original);
        assert_eq!(ScreamingSnakeCase.local_apply_to_field(original), screaming);
        assert_eq!(KebabCase.local_apply_to_field(original), kebab);
        assert_eq!(
            ScreamingKebabCase.local_apply_to_field(original),
            screaming_kebab
        );
    }
}
