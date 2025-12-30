// Generated macro for test (module)
macro_rules! Depcrate_yamltest {
() => {
// Module: crate::yaml
// Provides: {"test"}
// Dependencies: {}
# [cfg (all (test , feature = "encoding"))] mod test { use super :: { YAMLDecodingTrap , Yaml , YamlDecoder } ; # [test] fn test_read_bom () { let s = b"\xef\xbb\xbf---
a: 1
b: 2.2
c: [1, 2]
" ; let out = YamlDecoder :: read (s as & [u8]) . decode () . unwrap () ; let doc = & out [0] ; assert_eq ! (doc ["a"] . as_i64 () . unwrap () , 1i64) ; assert ! ((doc ["b"] . as_f64 () . unwrap () - 2.2f64) . abs () <= f64 :: EPSILON) ; assert_eq ! (doc ["c"] [1] . as_i64 () . unwrap () , 2i64) ; assert ! (doc ["d"] [0] . is_badvalue ()) ; } # [test] fn test_read_utf16le () { let s = b"\xff\xfe-\x00-\x00-\x00
\x00a\x00:\x00 \x001\x00
\x00b\x00:\x00 \x002\x00.\x002\x00
\x00c\x00:\x00 \x00[\x001\x00,\x00 \x002\x00]\x00
\x00" ; let out = YamlDecoder :: read (s as & [u8]) . decode () . unwrap () ; let doc = & out [0] ; println ! ("GOT: {doc:?}") ; assert_eq ! (doc ["a"] . as_i64 () . unwrap () , 1i64) ; assert ! ((doc ["b"] . as_f64 () . unwrap () - 2.2f64) <= f64 :: EPSILON) ; assert_eq ! (doc ["c"] [1] . as_i64 () . unwrap () , 2i64) ; assert ! (doc ["d"] [0] . is_badvalue ()) ; } # [test] fn test_read_utf16be () { let s = b"\xfe\xff\x00-\x00-\x00-\x00
\x00a\x00:\x00 \x001\x00
\x00b\x00:\x00 \x002\x00.\x002\x00
\x00c\x00:\x00 \x00[\x001\x00,\x00 \x002\x00]\x00
" ; let out = YamlDecoder :: read (s as & [u8]) . decode () . unwrap () ; let doc = & out [0] ; println ! ("GOT: {doc:?}") ; assert_eq ! (doc ["a"] . as_i64 () . unwrap () , 1i64) ; assert ! ((doc ["b"] . as_f64 () . unwrap () - 2.2f64) . abs () <= f64 :: EPSILON) ; assert_eq ! (doc ["c"] [1] . as_i64 () . unwrap () , 2i64) ; assert ! (doc ["d"] [0] . is_badvalue ()) ; } # [test] fn test_read_utf16le_nobom () { let s = b"-\x00-\x00-\x00
\x00a\x00:\x00 \x001\x00
\x00b\x00:\x00 \x002\x00.\x002\x00
\x00c\x00:\x00 \x00[\x001\x00,\x00 \x002\x00]\x00
\x00" ; let out = YamlDecoder :: read (s as & [u8]) . decode () . unwrap () ; let doc = & out [0] ; println ! ("GOT: {doc:?}") ; assert_eq ! (doc ["a"] . as_i64 () . unwrap () , 1i64) ; assert ! ((doc ["b"] . as_f64 () . unwrap () - 2.2f64) . abs () <= f64 :: EPSILON) ; assert_eq ! (doc ["c"] [1] . as_i64 () . unwrap () , 2i64) ; assert ! (doc ["d"] [0] . is_badvalue ()) ; } # [test] fn test_read_trap () { let s = b"---
a\xa9: 1
b: 2.2
c: [1, 2]
" ; let out = YamlDecoder :: read (s as & [u8]) . encoding_trap (YAMLDecodingTrap :: Ignore) . decode () . unwrap () ; let doc = & out [0] ; println ! ("GOT: {doc:?}") ; assert_eq ! (doc ["a"] . as_i64 () . unwrap () , 1i64) ; assert ! ((doc ["b"] . as_f64 () . unwrap () - 2.2f64) . abs () <= f64 :: EPSILON) ; assert_eq ! (doc ["c"] [1] . as_i64 () . unwrap () , 2i64) ; assert ! (doc ["d"] [0] . is_badvalue ()) ; } # [test] fn test_or () { assert_eq ! (Yaml :: Null . or (Yaml :: Integer (3)) , Yaml :: Integer (3)) ; assert_eq ! (Yaml :: Integer (3) . or (Yaml :: Integer (7)) , Yaml :: Integer (3)) ; } }
};
}
