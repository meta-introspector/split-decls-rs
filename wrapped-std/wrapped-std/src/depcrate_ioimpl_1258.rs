// Generated macro for impl_1258 (impl)
macro_rules! Depcrate_ioimpl_1258 {
() => {
// Module: crate::io
// Provides: {"impl_1258"}
// Dependencies: {}
# [stable (feature = "seek_io_take" , since = "1.89.0")] impl < T : Seek > Seek for Take < T > { fn seek (& mut self , pos : SeekFrom) -> Result < u64 > { let new_position = match pos { SeekFrom :: Start (v) => Some (v) , SeekFrom :: Current (v) => self . position () . checked_add_signed (v) , SeekFrom :: End (v) => self . len . checked_add_signed (v) , } ; let new_position = match new_position { Some (v) if v <= self . len => v , _ => return Err (ErrorKind :: InvalidInput . into ()) , } ; while new_position != self . position () { if let Some (offset) = new_position . checked_signed_diff (self . position ()) { self . inner . seek_relative (offset) ? ; self . limit = self . limit . wrapping_sub (offset as u64) ; break ; } let offset = if new_position > self . position () { i64 :: MAX } else { i64 :: MIN } ; self . inner . seek_relative (offset) ? ; self . limit = self . limit . wrapping_sub (offset as u64) ; } Ok (new_position) } fn stream_len (& mut self) -> Result < u64 > { Ok (self . len) } fn stream_position (& mut self) -> Result < u64 > { Ok (self . position ()) } fn seek_relative (& mut self , offset : i64) -> Result < () > { if ! self . position () . checked_add_signed (offset) . is_some_and (| p | p <= self . len) { return Err (ErrorKind :: InvalidInput . into ()) ; } self . inner . seek_relative (offset) ? ; self . limit = self . limit . wrapping_sub (offset as u64) ; Ok (()) } }
};
}
