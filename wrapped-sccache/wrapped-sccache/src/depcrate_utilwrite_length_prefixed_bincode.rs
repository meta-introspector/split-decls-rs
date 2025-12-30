// Generated macro for write_length_prefixed_bincode (function)
macro_rules! Depcrate_utilwrite_length_prefixed_bincode {
() => {
// Module: crate::util
// Provides: {"write_length_prefixed_bincode"}
// Dependencies: {}
# [doc = " Write `data` to `writer` with bincode serialization, prefixed by a `u32` length."] pub fn write_length_prefixed_bincode < W , S > (mut writer : W , data : S) -> Result < () > where W : Write , S : Serialize , { let bytes = bincode :: serialize (& data) ? ; let mut len = [0 ; 4] ; BigEndian :: write_u32 (& mut len , bytes . len () as u32) ; writer . write_all (& len) ? ; writer . write_all (& bytes) ? ; writer . flush () ? ; Ok (()) }
};
}
