// Generated macro for impl_5 (impl)
macro_rules! Depcrate_integerimpl_5 {
() => {
// Module: crate::integer
// Provides: {"impl_5"}
// Dependencies: {}
impl TomlIntegerFormat { # [doc = " Creates a new integer format (decimal)."] pub fn new () -> Self { Self { radix : Radix :: Decimal , } } # [doc = " Sets the format to decimal."] pub fn as_decimal (mut self) -> Self { self . radix = Radix :: Decimal ; self } # [doc = " Sets the format to hexadecimal with all characters in uppercase."] pub fn as_hex_upper (mut self) -> Self { self . radix = Radix :: Hexadecimal { case : HexCase :: Upper , } ; self } # [doc = " Sets the format to hexadecimal with all characters in lowercase."] pub fn as_hex_lower (mut self) -> Self { self . radix = Radix :: Hexadecimal { case : HexCase :: Lower , } ; self } # [doc = " Sets the format to octal."] pub fn as_octal (mut self) -> Self { self . radix = Radix :: Octal ; self } # [doc = " Sets the format to binary."] pub fn as_binary (mut self) -> Self { self . radix = Radix :: Binary ; self } # [doc = " Formats `value` as a TOML integer."] # [doc = ""] # [doc = " Returns `None` if the value cannot be formatted"] # [doc = " (e.g. value is negative and the radix is not decimal)."] pub fn format < N : PartialOrd < i32 > > (self , value : N) -> Option < TomlInteger < N > > where TomlInteger < N > : crate :: WriteTomlValue , { match self . radix { Radix :: Decimal => () , Radix :: Hexadecimal { .. } | Radix :: Octal | Radix :: Binary => { if value < 0 { return None ; } } } Some (TomlInteger { value , format : self , }) } }
};
}
