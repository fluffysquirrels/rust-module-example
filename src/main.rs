// Mute some warnings by unused code in the examples.
#![allow(dead_code, unused_imports)]

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

// ## Name resolution
mod name_resolution {
    // Everything is private by default in Rust, including modules.
    // `private_inner` is visible in `name_resolution` and its child
    // modules, but not by `name_resolution`'s parents.
    mod private_inner {

        // `a` is visible to `private_inner` and its child modules, but nowhere else.
        fn a() {}

        // `pub` keyword makes an item exported by its parent module,
        // so `b` is visible to any module that can see
        // `private_inner`, but `main` cannot see `private_inner`, so it still
        // cannot see `b`.
        pub fn b() {}
    }

    fn test_private_inner() {
        // Access a child module's exported item with the syntax `${child_module}::{item}`:
        private_inner::b();

        // This will not compile, because `a` is not exported with `pub`:
        // private_inner::a();
    }

    // `main` sees this module and everything exported by it.
    pub mod public_inner {
        pub fn a() {}
    }
}

// ## Imports with `use`

// Items visible in a scope can be imported into that scope with the `use` keyword:
mod use_examples {

    mod use_inner {
        pub fn a() {}
        pub fn b() {}
    }

    // `use_inner::a` is now in scope as `a` in `use_examples`.
    use use_inner::a;

    fn test_use() {
        // `use_inner::a` can be called explicitly.
        use_inner::a();

        // But because we imported it with `use` we can also call it simply as `a`.
        a();

        // Function definitions also have a scope, so can contain `use` statements,
        // which work as you'd expect.
        use use_inner::b;
        b();
    }

    mod use_wildcard {
        pub fn not() {}
        pub fn my() {}
        pub fn favourite() {}
    }

    // `use` also supports wildcards.
    // I personally don't like this because with multiple wildcards it's more difficult
    // to know where a particular item comes from.
    use use_wildcard::*;

    fn test_use_wildcard() {
        not();
        my();
        favourite();
    }

    // `use` supports renaming, useful to avoid name clashes:

    mod use_rename {
        pub fn a() {}
    }

    // This would be a compile error, because we already imported `use_inner::a` above.
    // use use_rename::a;

    // This imports `use_rename::a` as `a_renamed`.
    use use_rename::a as a_renamed;

    // `use` supports a nested syntax which avoids repetition in imports:

    mod use_nested_1 {
        pub mod use_nested_2 {
            pub fn g() {}
            pub fn h() {}
        }

        pub mod use_nested_3 {
            pub fn g() {}
            pub fn i() {}
        }

        pub fn j() {}
    }

    use use_nested_1::{
        use_nested_2::{g, h},
        use_nested_3::{g as use_nested_3_g, i},
        j
    };

    fn test_use_nested() {
        g();
        use_nested_3_g();
        h();
        i();
        j();
    }

    // ### Advanced: re-exporting.

    // Items visible in a scope can be exported by that scope with the `pub use` keywords:
    mod inner_1 {
        mod inner_2 {
            pub fn x() {}
        }

        // `inner_2::x` is now imported into `inner_1`'s scope as `x`, but it has also been
        // exported because of the `pub`.
        pub use inner_2::x;
    }

    fn test_pub_use() {
        // This doesn't compile, because `inner_2` is not exported by `inner_1`.
        // inner_1::inner_2::x();

        // This works fine and refers to the same function, because
        // `inner_1` exported `inner_2::x` with `pub use`.
        inner_1::x();
    }

    // ----

    // `use` and `pub use` can be applied to almost any item in a module, including:
    //
    // * `const`
    // * `enum`
    // * `fn`
    // * `mod`
    // * `static`
    // * `struct`
    // * `trait`
    // * `type`

    // Macros have some different rules, I may come back to explain those another time.
}

fn main() {
    println!("Hello, world! Running on platform family '{}'", use_platform());
    inline::inline_fn();

    name_resolution::public_inner::a();
}
