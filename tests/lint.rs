#![allow(incomplete_features)]
#![feature(generators, impl_trait_in_bindings)]
#![warn(future_incompatible, nonstandard_style, rust_2018_compatibility, rust_2018_idioms, unused)]
#![forbid(unsafe_code)]
#![allow(unknown_lints)] // for old compilers
#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
// unused_crate_dependencies: unrelated
// unsafe_code: forbidden
// unsafe_block_in_unsafe_fn: unstable
// unstable_features: deprecated: https://doc.rust-lang.org/beta/rustc/lints/listing/allowed-by-default.html#unstable-features
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![warn(clippy::restriction)]
#![allow(clippy::blanket_clippy_restriction_lints)] // this is a test, so enable all restriction lints intentionally.
#![allow(clippy::implicit_return, clippy::let_underscore_must_use)]

// Check interoperability with rustc and clippy lints.

pub mod basic {
    use futures_async_stream::{stream, try_stream};
    use futures_core::stream::Stream;

    include!("include/basic.rs");
}

#[allow(box_pointers)]
#[allow(clippy::restriction)]
#[test]
fn check_lint_list() {
    use std::{env, process::Command, str};

    (|| -> Result<(), Box<dyn std::error::Error>> {
        let current = include_str!("lint.txt");
        let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
        let output = Command::new(rustc).args(&["-W", "help"]).output()?;
        let new = str::from_utf8(&output.stdout)?;
        assert_eq!(current, new);
        Ok(())
    })()
    .unwrap_or_else(|e| panic!("{}", e));
}
