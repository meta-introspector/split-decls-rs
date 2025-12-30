// Generated macro for impl_332 (impl)
macro_rules! Depcrate_unicode_dataimpl_332 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_332"}
// Dependencies: {}
impl std :: str :: FromStr for UnicodeDataDecompositionTag { type Err = Error ; fn from_str (s : & str) -> Result < UnicodeDataDecompositionTag , Error > { use self :: UnicodeDataDecompositionTag :: * ; Ok (match s { "font" => Font , "noBreak" => NoBreak , "initial" => Initial , "medial" => Medial , "final" => Final , "isolated" => Isolated , "circle" => Circle , "super" => Super , "sub" => Sub , "vertical" => Vertical , "wide" => Wide , "narrow" => Narrow , "small" => Small , "square" => Square , "fraction" => Fraction , "compat" => Compat , _ => return err ! ("invalid decomposition formatting tag: {}" , s) , }) } }
};
}
