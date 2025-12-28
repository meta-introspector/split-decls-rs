macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_94 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `rust_2024_prelude_collisions` lint detects the usage of trait methods which are ambiguous"] # [doc = " with traits added to the prelude in future editions."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2021,compile_fail"] # [doc = " #![deny(rust_2024_prelude_collisions)]"] # [doc = " trait Meow {"] # [doc = "     fn poll(&self) {}"] # [doc = " }"] # [doc = " impl<T> Meow for T {}"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     core::pin::pin!(async {}).poll();"] # [doc = "     //                        ^^^^^^"] # [doc = "     // This call to try_into matches both Future::poll and Meow::poll as"] # [doc = "     // `Future` has been added to the Rust prelude in 2024 edition."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Rust 2024, introduces two new additions to the standard library's prelude:"] # [doc = " `Future` and `IntoFuture`. This results in an ambiguity as to which method/function"] # [doc = " to call when an existing `poll`/`into_future` method is called via dot-call syntax or"] # [doc = " a `poll`/`into_future` associated function is called directly on a type."] # [doc = ""] pub RUST_2024_PRELUDE_COLLISIONS , Allow , "detects the usage of trait methods which are ambiguous with traits added to the \
        prelude in future editions" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/prelude.html>" , } ; }
    };
}

macro_94!()