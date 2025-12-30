// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_io_cursorimpl_1030 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1030"}
// Dependencies: {}
impl < T > Cursor < T > where T : AsMut < [u8] > , { # [doc = " Splits the underlying slice at the cursor position and returns them"] # [doc = " mutably."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(cursor_split)]"] # [doc = " use std::io::Cursor;"] # [doc = ""] # [doc = " let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);"] # [doc = ""] # [doc = " assert_eq!(buff.split_mut(), ([].as_mut_slice(), [1, 2, 3, 4, 5].as_mut_slice()));"] # [doc = ""] # [doc = " buff.set_position(2);"] # [doc = " assert_eq!(buff.split_mut(), ([1, 2].as_mut_slice(), [3, 4, 5].as_mut_slice()));"] # [doc = ""] # [doc = " buff.set_position(6);"] # [doc = " assert_eq!(buff.split_mut(), ([1, 2, 3, 4, 5].as_mut_slice(), [].as_mut_slice()));"] # [doc = " ```"] # [unstable (feature = "cursor_split" , issue = "86369")] pub fn split_mut (& mut self) -> (& mut [u8] , & mut [u8]) { let slice = self . inner . as_mut () ; let pos = self . pos . min (slice . len () as u64) ; slice . split_at_mut (pos as usize) } }
};
}
