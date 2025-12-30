// Generated macro for Color (enum)
macro_rules! DepcrateColor {
() => {
// Module: crate
// Provides: {"Color"}
// Dependencies: {}
# [doc = " The set of available colors for the terminal foreground/background."] # [doc = ""] # [doc = " The `Ansi256` and `Rgb` colors will only output the correct codes when"] # [doc = " paired with the `Ansi` `WriteColor` implementation."] # [doc = ""] # [doc = " The `Ansi256` and `Rgb` color types are not supported when writing colors"] # [doc = " on Windows using the console. If they are used on Windows, then they are"] # [doc = " silently ignored and no colors will be emitted."] # [doc = ""] # [doc = " This set may expand over time."] # [doc = ""] # [doc = " This type has a `FromStr` impl that can parse colors from their human"] # [doc = " readable form. The format is as follows:"] # [doc = ""] # [doc = " 1. Any of the explicitly listed colors in English. They are matched"] # [doc = "    case insensitively."] # [doc = " 2. A single 8-bit integer, in either decimal or hexadecimal format."] # [doc = " 3. A triple of 8-bit integers separated by a comma, where each integer is"] # [doc = "    in decimal or hexadecimal format."] # [doc = ""] # [doc = " Hexadecimal numbers are written with a `0x` prefix."] # [allow (missing_docs)] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum Color { Black , Blue , Green , Red , Cyan , Magenta , Yellow , White , Ansi256 (u8) , Rgb (u8 , u8 , u8) , # [doc (hidden)] __Nonexhaustive , }
};
}
