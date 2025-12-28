macro_rules! macro_110 {
    () => {
        declare_lint ! { # [doc = " The `invalid_macro_export_arguments` lint detects cases where `#[macro_export]` is being used with invalid arguments."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(invalid_macro_export_arguments)]"] # [doc = ""] # [doc = " #[macro_export(invalid_parameter)]"] # [doc = " macro_rules! myMacro {"] # [doc = "    () => {"] # [doc = "         // [...]"] # [doc = "    }"] # [doc = " }"] # [doc = ""] # [doc = " #[macro_export(too, many, items)]"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The only valid argument is `#[macro_export(local_inner_macros)]` or no argument (`#[macro_export]`)."] # [doc = " You can't have multiple arguments in a `#[macro_export(..)]`, or mention arguments other than `local_inner_macros`."] # [doc = ""] pub INVALID_MACRO_EXPORT_ARGUMENTS , Warn , "\"invalid_parameter\" isn't a valid argument for `#[macro_export]`" , }
    };
}

macro_110!();