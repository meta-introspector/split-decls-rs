// Generated macro for util (module)
macro_rules! Depcrateutil {
() => {
// Module: crate
// Provides: {"util"}
// Dependencies: {}
pub mod util { use unicode_segmentation :: UnicodeSegmentation ; use unicode_width :: UnicodeWidthStr ; pub fn sanitize_offset (offset : u16 , num_items : usize , num_displayable_lines : u16) -> u16 { offset . min ((num_items . saturating_sub (num_displayable_lines as usize)) as u16) } # [derive (Default)] pub struct GraphemeCountWriter (pub usize) ; impl std :: io :: Write for GraphemeCountWriter { fn write (& mut self , buf : & [u8]) -> Result < usize , std :: io :: Error > { self . 0 += String :: from_utf8_lossy (buf) . graphemes (true) . count () ; Ok (buf . len ()) } fn flush (& mut self) -> Result < () , std :: io :: Error > { Ok (()) } } pub fn block_width (s : & str) -> u16 { s . width () as u16 } pub mod rect { use tui :: layout :: Rect ; # [doc = " A safe version of Rect::intersection that doesn't suffer from underflows"] pub fn intersect (lhs : Rect , rhs : Rect) -> Rect { let x1 = lhs . x . max (rhs . x) ; let y1 = lhs . y . max (rhs . y) ; let x2 = lhs . right () . min (rhs . right ()) ; let y2 = lhs . bottom () . min (rhs . bottom ()) ; Rect { x : x1 , y : y1 , width : x2 . saturating_sub (x1) , height : y2 . saturating_sub (y1) , } } pub fn offset_x (r : Rect , offset : u16) -> Rect { Rect { x : r . x + offset , width : r . width . saturating_sub (offset) , .. r } } pub fn snap_to_right (bound : Rect , new_width : u16) -> Rect { offset_x (bound , bound . width . saturating_sub (new_width)) } pub fn line_bound (bound : Rect , line : usize) -> Rect { Rect { y : bound . y + line as u16 , height : 1 , .. bound } } } }
};
}
