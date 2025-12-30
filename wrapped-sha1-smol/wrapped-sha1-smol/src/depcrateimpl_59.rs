// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: ser :: Serialize for Digest { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { fn to_hex (num : u8) -> u8 { b"0123456789abcdef" [num as usize] } let mut hex_str = [0u8 ; 40] ; let mut c = 0 ; for state in self . data . state . iter () { for off in 0 .. 4 { let byte = (state >> (8 * (3 - off))) as u8 ; hex_str [c] = to_hex (byte >> 4) ; hex_str [c + 1] = to_hex (byte & 0xf) ; c += 2 ; } } serializer . serialize_str (unsafe { str :: from_utf8_unchecked (& hex_str [..]) }) } }
};
}
