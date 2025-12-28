macro_rules! macro_74 {
    () => {
        declare_lint ! { # [doc = " The `ineffective_unstable_trait_impl` lint detects `#[unstable]` attributes which are not used."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![feature(staged_api)]"] # [doc = ""] # [doc = " #[derive(Clone)]"] # [doc = " #[stable(feature = \"x\", since = \"1\")]"] # [doc = " struct S {}"] # [doc = ""] # [doc = " #[unstable(feature = \"y\", issue = \"none\")]"] # [doc = " impl Copy for S {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " `staged_api` does not currently support using a stability attribute on `impl` blocks."] # [doc = " `impl`s are always stable if both the type and trait are stable, and always unstable otherwise."] pub INEFFECTIVE_UNSTABLE_TRAIT_IMPL , Deny , "detects `#[unstable]` on stable trait implementations for stable types" }
    };
}

macro_74!()