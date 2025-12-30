// Generated macro for test_cfgs (macro)
macro_rules! Depcratetest_cfgs {
() => {
// Module: crate
// Provides: {"test_cfgs"}
// Dependencies: {}
macro_rules ! test_cfgs { ($ ($ cfg : ident ,) *) => { $ ({ let cfg_desc = format ! ("cfg!({})" , stringify ! ($ cfg)) ; if cfg ! ($ cfg) { println ! ("Enabled:    {}" , cfg_desc) ; } else { println ! ("Disabled:   {}" , cfg_desc) ; } }) * } ; }
};
}
