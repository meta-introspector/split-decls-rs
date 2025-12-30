// Generated macro for check_method (function)
macro_rules! Depcratecheck_method {
() => {
// Module: crate
// Provides: {"check_method"}
// Dependencies: {}
# [track_caller] pub fn check_method < Arguments : EncodeArguments , Return : EncodeReturn > (cls : & AnyClass , sel : Sel , _expected_encoding : & str ,) { let Some (method) = cls . instance_method (sel) else { return ; } ; if let Err (err) = cls . verify_sel :: < Arguments , Return > (sel) { if cls . name () == c"AVAudioIONode" && sel == sel ! (audioUnit) { return ; } if cls . name () == c"AVAudioUnit" && sel == sel ! (audioUnit) { return ; } if (cls . name () == c"NSCollectionLayoutSection" || cls . name () == c"NSCollectionLayoutItem") && (sel == sel ! (contentInsets) || sel == sel ! (setContentInsets :)) { return ; } panic ! ("could not verify selector {sel}\n    {err}") ; } }
};
}
