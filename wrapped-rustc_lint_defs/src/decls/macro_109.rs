macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_109 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `dependency_on_unit_never_type_fallback` lint detects cases where code compiles with"] # [doc = " [never type fallback] being [`()`], but will stop compiling with fallback being [`!`]."] # [doc = ""] # [doc = " [never type fallback]: https://doc.rust-lang.org/nightly/core/primitive.never.html#never-type-fallback"] # [doc = " [`!`]: https://doc.rust-lang.org/core/primitive.never.html"] # [doc = " [`()`]: https://doc.rust-lang.org/core/primitive.unit.html"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail,edition2021"] # [doc = " #![deny(dependency_on_unit_never_type_fallback)]"] # [doc = " fn main() {"] # [doc = "     if true {"] # [doc = "         // return has type `!` which, is some cases, causes never type fallback"] # [doc = "         return"] # [doc = "     } else {"] # [doc = "         // the type produced by this call is not specified explicitly,"] # [doc = "         // so it will be inferred from the previous branch"] # [doc = "         Default::default()"] # [doc = "     };"] # [doc = "     // depending on the fallback, this may compile (because `()` implements `Default`),"] # [doc = "     // or it may not (because `!` does not implement `Default`)"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Due to historic reasons never type fallback was `()`, meaning that `!` got spontaneously"] # [doc = " coerced to `()`. There are plans to change that, but they may make the code such as above"] # [doc = " not compile. Instead of depending on the fallback, you should specify the type explicitly:"] # [doc = " ```"] # [doc = " if true {"] # [doc = "     return"] # [doc = " } else {"] # [doc = "     // type is explicitly specified, fallback can't hurt us no more"] # [doc = "     <() as Default>::default()"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " See [Tracking Issue for making `!` fall back to `!`](https://github.com/rust-lang/rust/issues/123748)."] pub DEPENDENCY_ON_UNIT_NEVER_TYPE_FALLBACK , Warn , "never type fallback affecting unsafe function calls" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionAndFutureReleaseError (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/never-type-fallback.html>" , report_in_deps : true , } ; report_in_external_macro }
    };
}

macro_109!()