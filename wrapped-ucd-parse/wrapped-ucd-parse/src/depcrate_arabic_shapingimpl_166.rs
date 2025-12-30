// Generated macro for impl_166 (impl)
macro_rules! Depcrate_arabic_shapingimpl_166 {
() => {
// Module: crate::arabic_shaping
// Provides: {"impl_166"}
// Dependencies: {}
impl std :: str :: FromStr for JoiningType { type Err = Error ; fn from_str (s : & str) -> Result < JoiningType , Error > { match s { "R" => Ok (JoiningType :: RightJoining) , "L" => Ok (JoiningType :: LeftJoining) , "D" => Ok (JoiningType :: DualJoining) , "C" => Ok (JoiningType :: JoinCausing) , "U" => Ok (JoiningType :: NonJoining) , "T" => Ok (JoiningType :: Transparent) , _ => err ! ("unrecognized joining type: '{}' \
                 (must be one of R, L, D, C, U or T)" , s) , } } }
};
}
