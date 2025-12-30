// Generated macro for impl_164 (impl)
macro_rules! Depcrate_tendrilimpl_164 {
() => {
// Module: crate::tendril
// Provides: {"impl_164"}
// Dependencies: {}
impl < A > Tendril < fmt :: UTF8 , A > where A : Atomicity , { # [doc = " Encode from UTF-8 into some other character encoding."] # [doc = ""] # [doc = " See the [rust-encoding docs](https://lifthrasiir.github.io/rust-encoding/encoding/)"] # [doc = " for more information."] # [inline] pub fn encode (& self , encoding : EncodingRef , trap : EncoderTrap) -> Result < Tendril < fmt :: Bytes , A > , Cow < 'static , str > > { let mut ret = Tendril :: new () ; encoding . encode_to (& * self , trap , & mut ret) . map (| _ | ret) } # [doc = " Push a character onto the end."] # [inline] pub fn push_char (& mut self , c : char) { unsafe { let mut utf_8 : [u8 ; 4] = mem :: uninitialized () ; let bytes_written = { let mut buffer = & mut utf_8 [..] ; write ! (buffer , "{}" , c) . ok () . expect ("Tendril::push_char: internal error") ; debug_assert ! (buffer . len () <= 4) ; 4 - buffer . len () } ; self . push_bytes_without_validating (unsafe_slice (& utf_8 , 0 , bytes_written)) ; } } # [doc = " Create a `Tendril` from a single character."] # [inline] pub fn from_char (c : char) -> Tendril < fmt :: UTF8 , A > { let mut t : Tendril < fmt :: UTF8 , A > = Tendril :: new () ; t . push_char (c) ; t } # [doc = " Helper for the `format_tendril!` macro."] # [inline] pub fn format (args : strfmt :: Arguments) -> Tendril < fmt :: UTF8 , A > { use std :: fmt :: Write ; let mut output : Tendril < fmt :: UTF8 , A > = Tendril :: new () ; let _ = write ! (& mut output , "{}" , args) ; output } }
};
}
