// Generated macro for test (module)
macro_rules! Depcrate_crc32test {
() => {
// Module: crate::crc32
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_empty_reader () { let data : & [u8] = b"" ; let mut buf = [0 ; 1] ; let mut reader = Crc32Reader :: new (data , 0 , false) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; let mut reader = Crc32Reader :: new (data , 1 , false) ; assert ! (reader . read (& mut buf) . unwrap_err () . to_string () . contains ("Invalid checksum")) ; } # [test] fn test_byte_by_byte () { let data : & [u8] = b"1234" ; let mut buf = [0 ; 1] ; let mut reader = Crc32Reader :: new (data , 0x9be3e0a3 , false) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 1) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 1) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 1) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 1) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; } # [test] fn test_zero_read () { let data : & [u8] = b"1234" ; let mut buf = [0 ; 5] ; let mut reader = Crc32Reader :: new (data , 0x9be3e0a3 , false) ; assert_eq ! (reader . read (& mut buf [.. 0]) . unwrap () , 0) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 4) ; } }
};
}
