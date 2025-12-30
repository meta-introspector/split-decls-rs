// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl FromStr for Color { type Err = ParseColorError ; fn from_str (s : & str) -> Result < Color , ParseColorError > { match & * s . to_lowercase () { "black" => Ok (Color :: Black) , "blue" => Ok (Color :: Blue) , "green" => Ok (Color :: Green) , "red" => Ok (Color :: Red) , "cyan" => Ok (Color :: Cyan) , "magenta" => Ok (Color :: Magenta) , "yellow" => Ok (Color :: Yellow) , "white" => Ok (Color :: White) , _ => Color :: from_str_numeric (s) , } } }
};
}
