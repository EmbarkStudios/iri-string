//! Utilities.
#![allow(dead_code)]

use RawKind::*;

/// Raw kind (exclusive).
#[derive(Clone, Copy, PartialEq, Eq)]
enum RawKind {
    /// Invalid string.
    Invalid,
    /// IRI.
    Iri,
    /// Absolute IRI.
    IriAbsolute,
    /// Relative IRI.
    IriRelative,
    /// URI.
    Uri,
    /// Absolute URI.
    UriAbsolute,
    /// Relative URI.
    UriRelative,
}

impl RawKind {
    fn spec_is(self, spec: Spec) -> bool {
        match spec {
            Spec::Uri => match self {
                Self::Uri | Self::UriAbsolute | Self::UriRelative => true,
                _ => false,
            },
            Spec::Iri => self != Self::Invalid,
        }
    }

    fn kind_is(self, kind: Kind) -> bool {
        match kind {
            Kind::Absolute => match self {
                Self::UriAbsolute | Self::IriAbsolute => true,
                _ => false,
            },
            Kind::Normal => match self {
                Self::UriAbsolute | Self::Uri | Self::IriAbsolute | Self::Iri => true,
                _ => false,
            },
            Kind::Reference => self != Self::Invalid,
            Kind::Relative => match self {
                Self::UriRelative | Self::IriRelative => true,
                _ => false,
            },
        }
    }

    fn is(self, spec: Spec, kind: Kind) -> bool {
        self.spec_is(spec) && self.kind_is(kind)
    }
}

/// Strings.
/// ```
/// # use iri_string::types::IriReferenceStr;
/// // `<` and `>` cannot directly appear in an IRI reference.
/// assert!(IriReferenceStr::new("<not allowed>").is_err());
/// // Broken percent encoding cannot appear in an IRI reference.
/// assert!(IriReferenceStr::new("%").is_err());
/// assert!(IriReferenceStr::new("%GG").is_err());
/// ```
const STRINGS: &[(RawKind, &str)] = &[
    (UriAbsolute, "https://user:pass@example.com:8080"),
    (UriAbsolute, "https://example.com/"),
    (UriAbsolute, "https://example.com/foo?bar=baz"),
    (Uri, "https://example.com/foo?bar=baz#qux"),
    (UriAbsolute, "foo:bar"),
    (UriAbsolute, "foo:"),
    (UriAbsolute, "foo:/"),
    (UriAbsolute, "foo://"),
    (UriAbsolute, "foo:///"),
    (UriAbsolute, "foo:////"),
    (UriAbsolute, "foo://///"),
    (UriRelative, "foo"),
    (UriRelative, "foo/bar"),
    (UriRelative, "foo//bar"),
    (UriRelative, "/"),
    (UriRelative, "/foo"),
    (UriRelative, "/foo/bar"),
    (UriRelative, "//foo/bar"),
    (UriRelative, "/foo//bar"),
    (UriRelative, "?"),
    (UriRelative, "???"),
    (UriRelative, "?foo"),
    (UriRelative, "#"),
    (UriRelative, "#foo"),
    (Invalid, "##"),
    (Invalid, "fragment#cannot#have#hash#char"),
    // `<` cannot appear in an IRI reference.
    (Invalid, "<"),
    // `>` cannot appear in an IRI reference.
    (Invalid, ">"),
    // `<` and `>` cannot appear in an IRI reference.
    (Invalid, "lt<and-gt>not-allowed"),
    // Incomplete percent encoding.
    (Invalid, "%"),
    (Invalid, "%0"),
    (Invalid, "%f"),
    (Invalid, "%F"),
    // Invalid percent encoding.
    (Invalid, "%0g"),
    (Invalid, "%0G"),
    (Invalid, "%GG"),
    (Invalid, "%G0"),
];

/// Spec.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Spec {
    /// URI.
    Uri,
    /// IRI and URI.
    Iri,
}

/// Kind.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Absolute IRI / URI.
    Absolute,
    /// IRI / URI.
    Normal,
    /// IRI / URI reference.
    Reference,
    /// Relative IRI / URI reference.
    Relative,
}

pub fn positive(spec: Spec, kind: Kind) -> impl Iterator<Item = &'static str> {
    STRINGS
        .iter()
        .filter(move |(raw_kind, _)| raw_kind.is(spec, kind))
        .map(|(_, s)| *s)
}

pub fn negative(spec: Spec, kind: Kind) -> impl Iterator<Item = &'static str> {
    STRINGS
        .iter()
        .filter(move |(raw_kind, _)| !raw_kind.is(spec, kind))
        .map(|(_, s)| *s)
}