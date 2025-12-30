// Generated macro for Version (enum)
macro_rules! DepcrateVersion {
() => {
// Module: crate
// Provides: {"Version"}
// Dependencies: {}
# [doc = " The version of the UUID, denoting the generating algorithm."] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [Version Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.2)"] # [derive (Clone , Copy , Debug , PartialEq)] # [non_exhaustive] # [repr (u8)] pub enum Version { # [doc = " The \"nil\" (all zeros) UUID."] Nil = 0u8 , # [doc = " Version 1: Timestamp and node ID."] Mac = 1 , # [doc = " Version 2: DCE Security."] Dce = 2 , # [doc = " Version 3: MD5 hash."] Md5 = 3 , # [doc = " Version 4: Random."] Random = 4 , # [doc = " Version 5: SHA-1 hash."] Sha1 = 5 , # [doc = " Version 6: Sortable Timestamp and node ID."] SortMac = 6 , # [doc = " Version 7: Timestamp and random."] SortRand = 7 , # [doc = " Version 8: Custom."] Custom = 8 , # [doc = " The \"max\" (all ones) UUID."] Max = 0xff , }
};
}
