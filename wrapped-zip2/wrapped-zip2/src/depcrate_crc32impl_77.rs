// Generated macro for impl_77 (impl)
macro_rules! Depcrate_crc32impl_77 {
() => {
// Module: crate::crc32
// Provides: {"impl_77"}
// Dependencies: {}
impl < R : Read > Read for Crc32Reader < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let count = self . inner . read (buf) ? ; if self . enabled { if count == 0 && ! buf . is_empty () && ! self . check_matches () { return Err (invalid_checksum ()) ; } self . hasher . update (& buf [.. count]) ; } Ok (count) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { let start = buf . len () ; let n = self . inner . read_to_end (buf) ? ; if self . enabled { self . hasher . update (& buf [start ..]) ; if ! self . check_matches () { return Err (invalid_checksum ()) ; } } Ok (n) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { let start = buf . len () ; let n = self . inner . read_to_string (buf) ? ; if self . enabled { self . hasher . update (& buf . as_bytes () [start ..]) ; if ! self . check_matches () { return Err (invalid_checksum ()) ; } } Ok (n) } }
};
}
