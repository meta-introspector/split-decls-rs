// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " If you don't want to use the ShortVec newtype, you can do ShortVec"] # [doc = " serialization on an ordinary vector with the following field annotation:"] # [doc = ""] # [doc = " #[serde(with = \"short_vec\")]"] # [doc = ""] pub fn serialize < S : Serializer , T : Serialize > (elements : & [T] , serializer : S ,) -> Result < S :: Ok , S :: Error > { let mut seq = serializer . serialize_tuple (1) ? ; let len = elements . len () ; if len > u16 :: MAX as usize { return Err (ser :: Error :: custom ("length larger than u16")) ; } let short_len = ShortU16 (len as u16) ; seq . serialize_element (& short_len) ? ; for element in elements { seq . serialize_element (element) ? ; } seq . end () }
};
}
