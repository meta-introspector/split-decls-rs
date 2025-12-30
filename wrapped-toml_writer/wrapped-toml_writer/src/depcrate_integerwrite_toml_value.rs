// Generated macro for write_toml_value (function)
macro_rules! Depcrate_integerwrite_toml_value {
() => {
// Module: crate::integer
// Provides: {"write_toml_value"}
// Dependencies: {}
fn write_toml_value < N : Display + fmt :: UpperHex + fmt :: LowerHex + fmt :: Octal + fmt :: Binary , W : crate :: TomlWrite + ? Sized , > (value : N , format : & TomlIntegerFormat , writer : & mut W ,) -> fmt :: Result { match format . radix { Radix :: Decimal => write ! (writer , "{value}") ? , Radix :: Hexadecimal { case } => match case { HexCase :: Upper => write ! (writer , "0x{value:X}") ? , HexCase :: Lower => write ! (writer , "0x{value:x}") ? , } , Radix :: Octal => write ! (writer , "0o{value:o}") ? , Radix :: Binary => write ! (writer , "0b{value:b}") ? , } Ok (()) }
};
}
