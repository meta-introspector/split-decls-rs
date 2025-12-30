// Generated macro for ColorChoice (enum)
macro_rules! DepcrateColorChoice {
() => {
// Module: crate
// Provides: {"ColorChoice"}
// Dependencies: {}
# [doc = " ColorChoice represents the color preferences of an end user."] # [doc = ""] # [doc = " The `Default` implementation for this type will select `Auto`, which tries"] # [doc = " to do the right thing based on the current environment."] # [doc = ""] # [doc = " The `FromStr` implementation for this type converts a lowercase kebab-case"] # [doc = " string of the variant name to the corresponding variant. Any other string"] # [doc = " results in an error."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum ColorChoice { # [doc = " Try very hard to emit colors. This includes emitting ANSI colors"] # [doc = " on Windows if the console API is unavailable."] Always , # [doc = " AlwaysAnsi is like Always, except it never tries to use anything other"] # [doc = " than emitting ANSI color codes."] AlwaysAnsi , # [doc = " Try to use colors, but don't force the issue. If the console isn't"] # [doc = " available on Windows, or if TERM=dumb, or if `NO_COLOR` is defined, for"] # [doc = " example, then don't use colors."] Auto , # [doc = " Never emit colors."] Never , }
};
}
