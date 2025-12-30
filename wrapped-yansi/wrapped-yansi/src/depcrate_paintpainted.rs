// Generated macro for Painted (struct)
macro_rules! Depcrate_paintPainted {
() => {
// Module: crate::paint
// Provides: {"Painted"}
// Dependencies: {}
# [doc = " An arbitrary value with a [`Style`] applied to it."] # [doc = ""] # [doc = " A `Painted` can be directly formatted. This results in the internal"] # [doc = " [`value`](Self::value) being formatted as specified and ANSI code styling"] # [doc = " sequences corresponding to [`style`](Self::style) being prefixed and"] # [doc = " suffixed as necessary. Both the global and local [`Condition`] affects"] # [doc = " whether styling sequences are actually emitted: both must evaluated to true."] # [doc = " Otherwise, no styling sequences are emitted."] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::{Paint, Condition};"] # [doc = ""] # [doc = " println!(\"Hello, {}!\", \"world\".red().underline().blink());"] # [doc = " // > Hello, world! # world is red, underlined, and blinking"] # [doc = ""] # [doc = " let v = format!(\"{}\", \"world\".red().underline().blink());"] # [doc = " assert_eq!(v, \"\\u{1b}[4;5;31mworld\\u{1b}[0m\");"] # [doc = " println!(\"{}\", v); // > world # world is red, underlined, and blinking"] # [doc = ""] # [doc = " let v = format!(\"{}\", \"world\".red().underline().blink().whenever(Condition::NEVER));"] # [doc = " assert_eq!(v, \"world\");"] # [doc = " ```"] # [derive (Copy , Clone)] pub struct Painted < T > { # [doc = " The value to be styled."] pub value : T , # [doc = " The style to apply."] pub style : Style , }
};
}
