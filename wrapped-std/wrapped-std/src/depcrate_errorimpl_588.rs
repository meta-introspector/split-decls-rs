// Generated macro for impl_588 (impl)
macro_rules! Depcrate_errorimpl_588 {
() => {
// Module: crate::error
// Provides: {"impl_588"}
// Dependencies: {}
impl < T > Write for Indented < '_ , T > where T : Write , { fn write_str (& mut self , s : & str) -> fmt :: Result { for (i , line) in s . split ('\n') . enumerate () { if i > 0 { self . inner . write_char ('\n') ? ; self . inner . write_str ("      ") ? ; } self . inner . write_str (line) ? ; } Ok (()) } }
};
}
