// Generated macro for parse_extra_field (function)
macro_rules! Depcrate_readparse_extra_field {
() => {
// Module: crate::read
// Provides: {"parse_extra_field"}
// Dependencies: {}
pub (crate) fn parse_extra_field (file : & mut ZipFileData) -> ZipResult < Option < Arc < Vec < u8 > > > > { let Some (ref extra_field) = file . extra_field else { return Ok (None) ; } ; let extra_field = extra_field . clone () ; let mut processed_extra_field = extra_field . clone () ; let len = extra_field . len () ; let mut reader = io :: Cursor :: new (& * * extra_field) ; let mut position = reader . position () as usize ; while (position) < len { let old_position = position ; let remove = parse_single_extra_field (file , & mut reader , position as u64 , false) ? ; position = reader . position () as usize ; if remove { let remaining = len - (position - old_position) ; if remaining == 0 { return Ok (None) ; } let mut new_extra_field = Vec :: with_capacity (remaining) ; new_extra_field . extend_from_slice (& extra_field [0 .. old_position]) ; new_extra_field . extend_from_slice (& extra_field [position ..]) ; processed_extra_field = Arc :: new (new_extra_field) ; } } Ok (Some (processed_extra_field)) }
};
}
