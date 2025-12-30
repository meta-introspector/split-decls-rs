// Generated macro for impl_333 (impl)
macro_rules! Depcrate_unicode_dataimpl_333 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_333"}
// Dependencies: {}
impl std :: fmt :: Display for UnicodeDataDecompositionTag { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use self :: UnicodeDataDecompositionTag :: * ; let s = match * self { Font => "font" , NoBreak => "noBreak" , Initial => "initial" , Medial => "medial" , Final => "final" , Isolated => "isolated" , Circle => "circle" , Super => "super" , Sub => "sub" , Vertical => "vertical" , Wide => "wide" , Narrow => "narrow" , Small => "small" , Square => "square" , Fraction => "fraction" , Compat => "compat" , } ; write ! (f , "{}" , s) } }
};
}
