// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_io_cursorimpl_1029 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1029"}
// Dependencies: {}
impl < T > Cursor < T > where T : AsRef < [u8] > , { # [doc = " Splits the underlying slice at the cursor position and returns them."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(cursor_split)]"] # [doc = " use std::io::Cursor;"] # [doc = ""] # [doc = " let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);"] # [doc = ""] # [doc = " assert_eq!(buff.split(), ([].as_slice(), [1, 2, 3, 4, 5].as_slice()));"] # [doc = ""] # [doc = " buff.set_position(2);"] # [doc = " assert_eq!(buff.split(), ([1, 2].as_slice(), [3, 4, 5].as_slice()));"] # [doc = ""] # [doc = " buff.set_position(6);"] # [doc = " assert_eq!(buff.split(), ([1, 2, 3, 4, 5].as_slice(), [].as_slice()));"] # [doc = " ```"] # [unstable (feature = "cursor_split" , issue = "86369")] pub fn split (& self) -> (& [u8] , & [u8]) { let slice = self . inner . as_ref () ; let pos = self . pos . min (slice . len () as u64) ; slice . split_at (pos as usize) } }
};
}
