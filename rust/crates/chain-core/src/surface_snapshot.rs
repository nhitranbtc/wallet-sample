//! Minimal source-text parser used by the FFI surface snapshot test.
//!
//! The FFI bridge (`crates/ffi-bridge/src/api.rs`) exposes a hand-picked
//! set of `pub fn` entrypoints. Each adapter-layer change must keep that
//! surface stable; this helper gives the snapshot test a cheap way to
//! enumerate the public functions without depending on `syn` or a
//! toolchain plugin.

/// Scan `src` for lines that begin with `pub fn ` (after leading
/// whitespace) and return the function names.
///
/// Only the bare names are returned; signatures, generics, and
/// attributes are stripped. Lines that mention `pub fn` inside a string
/// literal or a comment are not handled specifically — the FFI file is
/// kept under our control and is expected to be free of such noise.
pub fn parse_pub_fns(src: &str) -> Vec<String> {
    const MARKER: &str = "pub fn ";

    src.lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let after = trimmed.strip_prefix(MARKER)?;
            let name = after
                .split(|c: char| c == '(' || c == '<' || c == ' ' || c == '\t')
                .next()
                .unwrap_or("")
                .trim();
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_names_in_order() {
        let src = r#"
            pub fn alpha() {}
            pub(crate) fn hidden() {}
            pub fn beta() -> u32 { 0 }
            fn private() {}
            pub fn gamma<T>(x: T) -> T { x }
        "#;
        let names = parse_pub_fns(src);
        assert_eq!(names, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn returns_empty_when_no_pub_fns() {
        let src = "fn only_private() {}\nstruct S;\n";
        assert!(parse_pub_fns(src).is_empty());
    }
}