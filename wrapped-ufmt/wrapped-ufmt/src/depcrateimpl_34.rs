// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl HexOptions { # [doc = " applies the various padding/prefix options while writing the `payload` string"] pub fn with_stuff < W : uWrite + ? Sized > (& self , fmt : & mut Formatter < '_ , W > , payload : & str ,) -> Result < () , < W as uWrite > :: Error > { let pad_before = self . ox_prefix && self . pad_char == b' ' ; let pad = self . pad_length as isize - (if self . ox_prefix { 2 } else { 0 } + payload . len ()) as isize ; let do_pad = | fmt : & mut Formatter < '_ , W > , pad : isize | -> Result < () , < W as uWrite > :: Error > { if pad > 0 { for _ in 0 .. pad { fmt . write_str (unsafe { str :: from_utf8_unchecked (& [self . pad_char]) }) ? ; } } Ok (()) } ; let do_prefix = | fmt : & mut Formatter < '_ , W > , go : bool , upper_case : bool | -> Result < () , < W as uWrite > :: Error > { if go { fmt . write_str (if upper_case { "0X" } else { "0x" }) } else { Ok (()) } } ; if pad_before { do_pad (fmt , pad) ? ; do_prefix (fmt , self . ox_prefix , self . upper_case) ? ; } else { do_prefix (fmt , self . ox_prefix , self . upper_case) ? ; do_pad (fmt , pad) ? ; } fmt . write_str (payload) } }
};
}
