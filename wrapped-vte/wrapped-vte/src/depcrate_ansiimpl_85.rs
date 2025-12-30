// Generated macro for impl_85 (impl)
macro_rules! Depcrate_ansiimpl_85 {
() => {
// Module: crate::ansi
// Provides: {"impl_85"}
// Dependencies: {}
impl StandardCharset { # [doc = " Switch/Map character to the active charset. Ascii is the common case and"] # [doc = " for that we want to do as little as possible."] # [inline] pub fn map (self , c : char) -> char { match self { StandardCharset :: Ascii => c , StandardCharset :: SpecialCharacterAndLineDrawing => match c { '_' => ' ' , '`' => '◆' , 'a' => '▒' , 'b' => '\u{2409}' , 'c' => '\u{240c}' , 'd' => '\u{240d}' , 'e' => '\u{240a}' , 'f' => '°' , 'g' => '±' , 'h' => '\u{2424}' , 'i' => '\u{240b}' , 'j' => '┘' , 'k' => '┐' , 'l' => '┌' , 'm' => '└' , 'n' => '┼' , 'o' => '⎺' , 'p' => '⎻' , 'q' => '─' , 'r' => '⎼' , 's' => '⎽' , 't' => '├' , 'u' => '┤' , 'v' => '┴' , 'w' => '┬' , 'x' => '│' , 'y' => '≤' , 'z' => '≥' , '{' => 'π' , '|' => '≠' , '}' => '£' , '~' => '·' , _ => c , } , } } }
};
}
