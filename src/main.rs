// # Rust modules example

// Most of the time a module means a file, but there are other
// options, see below.

// The modules form a tree from the root of the crate (usually
// `src/main.rs` for a bin, or `src/lib.rs` for a library) down to each module.

// Every child module must be explicitly declared with the `mod` keyword.
// Just having the file present in the tree will not do anything.

// This introduces the module `a` into the current module's child modules,
// using the definition in `./a.rs`.
mod a;

// ## Multi-level modules

// When a module will have its own child modules, there are 2 possible styles

// Style 1 has been in Rust since the beginning, and I actually prefer it as it keeps
// the child module contents in the same sub-directory:

mod multi_level_style_1; // references `multi_level_style_1/mod.rs`

// In directory `./multi_level_style_1/` there is `mod.rs` and
// `child.rs`, with `mod.rs` declaring `child.rs`.

// Style 2 was added in the 2018 edition of Rust and is now officially recommended.
// It was designed to avoid having lots of files named `mod.rs` in the
// project, as well as increase consistency with modules that do not
// have children:

mod multi_level_style_2; // references `multi_level_style_2.rs`, which declares a child in
                         // `multi_level_style_2/child.rs`

// It causes a compilation error when there are 2 files available for
// the same module name in the different styles,
// e.g. if there is `./foo.rs` and `./foo/mod.rs` and a module is declared with `mod foo`.

// ## Inline modules

// A module may also be declared in the parent file:
mod inline {
    // `inline` has its own scope, in particular to refer to other items
    // in main.rs you must refer to them from the crate root like `crate::item`,
    // relative to the current module like `super::item`.

    // Even though we're in `main.rs` this has to be declared `pub`
    // for other items in the top-level `main` module to see it.
    pub fn inline_fn() {
        super::f();
        crate::f();

        inline_private();
    }

    // Only `inline` and its child modules can see `inline_private`.
    fn inline_private() {}
}

fn f() {}

// Inline modules are often used to contain unit tests:

#[cfg(test)] // Only processed when building tests, like with `cargo test` or `cargo build --tests`.
mod tests {
    #[test]
    fn ok() {
        assert_eq!(1 + 1, 2);
    }
}

// ## Advanced usage

// A module declaration can override the file path that it loads:
#[path = "path_override_foo.rs"]
mod path_override;

// The main reason I've seen this is with per-platform conditional compilation of modules:

#[cfg(unix)] // Only processed when the target OS is Unix-like, e.g. MacOS or Linux.
#[path = "unix.rs"]
mod platform;

#[cfg(windows)] // Only processed when the target OS is Windows.
#[path = "windows.rs"]
mod platform;

// This allows code in this module to use items in `platform::*` without caring
// what implementation is going to be included:
fn use_platform() -> &'static str {
    platform::FAMILY
}

// Dependency crates can also be compiled conditionally based on the
// build target, enabled feature flags, and other factors.

fn main() {
    println!("Hello, world! Running on platform family '{}'", use_platform());
    inline::inline_fn();
}
