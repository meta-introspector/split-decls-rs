// Generated macro for impl_1740 (impl)
macro_rules! Depcrate_termios_typesimpl_1740 {
() => {
// Module: crate::termios::types
// Provides: {"impl_1740"}
// Dependencies: {}
impl core :: fmt :: Debug for SpecialCode { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { if self . 0 == 0 { write ! (f , "<undef>") } else if self . 0 < 0x20 { write ! (f , "^{}" , (self . 0 + 0x40) as char) } else if self . 0 == 0x7f { write ! (f , "^?") } else if self . 0 >= 0x80 { write ! (f , "M-") ? ; SpecialCode (self . 0 - 0x80) . fmt (f) } else { write ! (f , "{}" , (self . 0 as char)) } } }
};
}
