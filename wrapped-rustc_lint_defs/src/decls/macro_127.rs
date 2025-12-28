macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_127 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `out_of_scope_macro_calls` lint detects `macro_rules` called when they are not in scope,"] # [doc = " above their definition, which may happen in key-value attributes."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [cfg_attr (not (bootstrap) , doc = "```rust,compile_fail")] # [cfg_attr (bootstrap , doc = "```rust")] # [doc = " #![doc = in_root!()]"] # [doc = ""] # [doc = " macro_rules! in_root { () => { \"\" } }"] # [doc = ""] # [doc = " fn main() {}"] # [cfg_attr (not (bootstrap) , doc = "```")] # [cfg_attr (bootstrap , doc = "```")] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The scope in which a `macro_rules` item is visible starts at that item and continues"] # [doc = " below it. This is more similar to `let` than to other items, which are in scope both above"] # [doc = " and below their definition."] # [doc = " Due to a bug `macro_rules` were accidentally in scope inside some key-value attributes"] # [doc = " above their definition. The lint catches such cases."] # [doc = " To address the issue turn the `macro_rules` into a regularly scoped item by importing it"] # [doc = " with `use`."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this to a"] # [doc = " hard error in the future."] # [doc = ""] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub OUT_OF_SCOPE_MACRO_CALLS , Deny , "detects out of scope calls to `macro_rules` in key-value attributes" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #124535 <https://github.com/rust-lang/rust/issues/124535>" , report_in_deps : true , } ; }
    };
}

macro_127!();