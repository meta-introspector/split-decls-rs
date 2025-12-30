// Generated macro for impl_159 (impl)
macro_rules! Depcrate_extra_checksimpl_159 {
() => {
// Module: crate::extra_checks
// Provides: {"impl_159"}
// Dependencies: {}
impl ExtraCheckArg { fn matches (& self , lang : ExtraCheckLang , kind : ExtraCheckKind) -> bool { self . lang == lang && self . kind . map (| k | k == kind) . unwrap_or (true) } # [doc = " Returns `false` if this is an auto arg and the passed filename does not trigger the auto rule"] fn is_non_auto_or_matches (& self , filepath : & str) -> bool { if ! self . auto { return true ; } let exts : & [& str] = match self . lang { ExtraCheckLang :: Py => & [".py"] , ExtraCheckLang :: Cpp => & [".cpp"] , ExtraCheckLang :: Shell => & [".sh"] , ExtraCheckLang :: Js => & [".js" , ".ts"] , ExtraCheckLang :: Spellcheck => { if SPELLCHECK_DIRS . iter () . any (| dir | Path :: new (filepath) . starts_with (dir)) { return true ; } & [] } } ; exts . iter () . any (| ext | filepath . ends_with (ext)) } fn has_supported_kind (& self) -> bool { let Some (kind) = self . kind else { return true ; } ; use ExtraCheckKind :: * ; let supported_kinds : & [_] = match self . lang { ExtraCheckLang :: Py => & [Fmt , Lint] , ExtraCheckLang :: Cpp => & [Fmt] , ExtraCheckLang :: Shell => & [Lint] , ExtraCheckLang :: Spellcheck => & [] , ExtraCheckLang :: Js => & [Lint , Typecheck] , } ; supported_kinds . contains (& kind) } }
};
}
