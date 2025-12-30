// Generated macro for read (function)
macro_rules! Depcrate_fsread {
() => {
// Module: crate::fs
// Provides: {"read"}
// Dependencies: {}
# [doc = " Reads the entire contents of a file into a bytes vector."] # [doc = ""] # [doc = " This is a convenience function for using [`File::open`] and [`read_to_end`]"] # [doc = " with fewer imports and without an intermediate variable."] # [doc = ""] # [doc = " [`read_to_end`]: Read::read_to_end"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error if `path` does not already exist."] # [doc = " Other errors may also be returned according to [`OpenOptions::open`]."] # [doc = ""] # [doc = " While reading from the file, this function handles [`io::ErrorKind::Interrupted`]"] # [doc = " with automatic retries. See [io::Read] documentation for details."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {"] # [doc = "     let data: Vec<u8> = fs::read(\"image.jpg\")?;"] # [doc = "     assert_eq!(data[0..3], [0xFF, 0xD8, 0xFF]);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "fs_read_write_bytes" , since = "1.26.0")] pub fn read < P : AsRef < Path > > (path : P) -> io :: Result < Vec < u8 > > { fn inner (path : & Path) -> io :: Result < Vec < u8 > > { let mut file = File :: open (path) ? ; let size = file . metadata () . map (| m | usize :: try_from (m . len ()) . unwrap_or (usize :: MAX)) . ok () ; let mut bytes = Vec :: try_with_capacity (size . unwrap_or (0)) ? ; io :: default_read_to_end (& mut file , & mut bytes , size) ? ; Ok (bytes) } inner (path . as_ref ()) }
};
}
