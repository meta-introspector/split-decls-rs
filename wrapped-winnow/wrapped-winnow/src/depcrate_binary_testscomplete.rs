// Generated macro for complete (module)
macro_rules! Depcrate_binary_testscomplete {
() => {
// Module: crate::binary::tests
// Provides: {"complete"}
// Dependencies: {}
mod complete { use super :: * ; # [test] fn i8_tests () { assert_parse ! (i8 . parse_peek (& [0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (i8 . parse_peek (& [0x7f] [..]) , str ! [[r#"
Ok(
    (
        [],
        127,
    ),
)

"#]] . raw ()) ; assert_parse ! (i8 . parse_peek (& [0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (i8 . parse_peek (& [0x80] [..]) , str ! [[r#"
Ok(
    (
        [],
        -128,
    ),
)

"#]] . raw ()) ; } # [test] fn be_i8_tests () { assert_parse ! (be_i8 . parse_peek (& [0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i8 . parse_peek (& [0x7f] [..]) , str ! [[r#"
Ok(
    (
        [],
        127,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i8 . parse_peek (& [0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i8 . parse_peek (& [0x80] [..]) , str ! [[r#"
Ok(
    (
        [],
        -128,
    ),
)

"#]] . raw ()) ; } # [test] fn be_i16_tests () { assert_parse ! (be_i16 . parse_peek (& [0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i16 . parse_peek (& [0x7f , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        32767,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i16 . parse_peek (& [0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i16 . parse_peek (& [0x80 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        -32768,
    ),
)

"#]] . raw ()) ; } # [test] fn be_u24_tests () { assert_parse ! (be_u24 . parse_peek (& [0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_u24 . parse_peek (& [0x00 , 0xFF , 0xFF] [..]) , str ! [[r#"
Ok(
    (
        [],
        65535,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_u24 . parse_peek (& [0x12 , 0x34 , 0x56] [..]) , str ! [[r#"
Ok(
    (
        [],
        1193046,
    ),
)

"#]] . raw ()) ; } # [test] fn be_i24_tests () { assert_parse ! (be_i24 . parse_peek (& [0xFF , 0xFF , 0xFF] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i24 . parse_peek (& [0xFF , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        -65536,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i24 . parse_peek (& [0xED , 0xCB , 0xAA] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1193046,
    ),
)

"#]] . raw ()) ; } # [test] fn be_i32_tests () { assert_parse ! (be_i32 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i32 . parse_peek (& [0x7f , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        2147483647,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i32 . parse_peek (& [0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i32 . parse_peek (& [0x80 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        -2147483648,
    ),
)

"#]] . raw ()) ; } # [test] fn be_i64_tests () { assert_parse ! (be_i64 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i64 . parse_peek (& [0x7f , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        9223372036854775807,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i64 . parse_peek (& [0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i64 . parse_peek (& [0x80 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        -9223372036854775808,
    ),
)

"#]] . raw ()) ; } # [test] fn be_i128_tests () { assert_parse ! (be_i128 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i128 . parse_peek (& [0x7f , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        170141183460469231731687303715884105727,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i128 . parse_peek (& [0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_i128 . parse_peek (& [0x80 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        -170141183460469231731687303715884105728,
    ),
)

"#]] . raw ()) ; } # [test] fn le_i8_tests () { assert_parse ! (le_i8 . parse_peek (& [0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i8 . parse_peek (& [0x7f] [..]) , str ! [[r#"
Ok(
    (
        [],
        127,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i8 . parse_peek (& [0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i8 . parse_peek (& [0x80] [..]) , str ! [[r#"
Ok(
    (
        [],
        -128,
    ),
)

"#]] . raw ()) ; } # [test] fn le_i16_tests () { assert_parse ! (le_i16 . parse_peek (& [0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i16 . parse_peek (& [0xff , 0x7f] [..]) , str ! [[r#"
Ok(
    (
        [],
        32767,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i16 . parse_peek (& [0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i16 . parse_peek (& [0x00 , 0x80] [..]) , str ! [[r#"
Ok(
    (
        [],
        -32768,
    ),
)

"#]] . raw ()) ; } # [test] fn le_u24_tests () { assert_parse ! (le_u24 . parse_peek (& [0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_u24 . parse_peek (& [0xFF , 0xFF , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        65535,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_u24 . parse_peek (& [0x56 , 0x34 , 0x12] [..]) , str ! [[r#"
Ok(
    (
        [],
        1193046,
    ),
)

"#]] . raw ()) ; } # [test] fn le_i24_tests () { assert_parse ! (le_i24 . parse_peek (& [0xFF , 0xFF , 0xFF] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i24 . parse_peek (& [0x00 , 0x00 , 0xFF] [..]) , str ! [[r#"
Ok(
    (
        [],
        -65536,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i24 . parse_peek (& [0xAA , 0xCB , 0xED] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1193046,
    ),
)

"#]] . raw ()) ; } # [test] fn le_i32_tests () { assert_parse ! (le_i32 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i32 . parse_peek (& [0xff , 0xff , 0xff , 0x7f] [..]) , str ! [[r#"
Ok(
    (
        [],
        2147483647,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i32 . parse_peek (& [0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i32 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x80] [..]) , str ! [[r#"
Ok(
    (
        [],
        -2147483648,
    ),
)

"#]] . raw ()) ; } # [test] fn le_i64_tests () { assert_parse ! (le_i64 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i64 . parse_peek (& [0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0x7f] [..]) , str ! [[r#"
Ok(
    (
        [],
        9223372036854775807,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i64 . parse_peek (& [0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i64 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x80] [..]) , str ! [[r#"
Ok(
    (
        [],
        -9223372036854775808,
    ),
)

"#]] . raw ()) ; } # [test] fn le_i128_tests () { assert_parse ! (le_i128 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i128 . parse_peek (& [0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0x7f] [..]) , str ! [[r#"
Ok(
    (
        [],
        170141183460469231731687303715884105727,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i128 . parse_peek (& [0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff] [..]) , str ! [[r#"
Ok(
    (
        [],
        -1,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_i128 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x80] [..]) , str ! [[r#"
Ok(
    (
        [],
        -170141183460469231731687303715884105728,
    ),
)

"#]] . raw ()) ; } # [test] fn be_f32_tests () { assert_parse ! (be_f32 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0.0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_f32 . parse_peek (& [0x4d , 0x31 , 0x1f , 0xd8] [..]) , str ! [[r#"
Ok(
    (
        [],
        185728380.0,
    ),
)

"#]] . raw ()) ; } # [test] fn be_f64_tests () { assert_parse ! (be_f64 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0.0,
    ),
)

"#]] . raw ()) ; assert_parse ! (be_f64 . parse_peek (& [0x41 , 0xa6 , 0x23 , 0xfb , 0x10 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        185728392.0,
    ),
)

"#]] . raw ()) ; } # [test] fn le_f32_tests () { assert_parse ! (le_f32 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0.0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_f32 . parse_peek (& [0xd8 , 0x1f , 0x31 , 0x4d] [..]) , str ! [[r#"
Ok(
    (
        [],
        185728380.0,
    ),
)

"#]] . raw ()) ; } # [test] fn le_f64_tests () { assert_parse ! (le_f64 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00] [..]) , str ! [[r#"
Ok(
    (
        [],
        0.0,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_f64 . parse_peek (& [0x00 , 0x00 , 0x00 , 0x10 , 0xfb , 0x23 , 0xa6 , 0x41] [..]) , str ! [[r#"
Ok(
    (
        [],
        185728392.0,
    ),
)

"#]] . raw ()) ; } # [test] fn configurable_endianness () { use crate :: binary :: Endianness ; fn be_tst16 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , u16 > { u16 (Endianness :: Big) . parse_next (i) } fn le_tst16 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , u16 > { u16 (Endianness :: Little) . parse_next (i) } assert_parse ! (be_tst16 . parse_peek (& [0x80 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        32768,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_tst16 . parse_peek (& [0x80 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        128,
    ),
)

"#]] . raw ()) ; fn be_tst32 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , u32 > { u32 (Endianness :: Big) . parse_next (i) } fn le_tst32 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , u32 > { u32 (Endianness :: Little) . parse_next (i) } assert_parse ! (be_tst32 . parse_peek (& [0x12 , 0x00 , 0x60 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        302014464,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_tst32 . parse_peek (& [0x12 , 0x00 , 0x60 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        6291474,
    ),
)

"#]] . raw ()) ; fn be_tst64 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , u64 > { u64 (Endianness :: Big) . parse_next (i) } fn le_tst64 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , u64 > { u64 (Endianness :: Little) . parse_next (i) } assert_parse ! (be_tst64 . parse_peek (& [0x12 , 0x00 , 0x60 , 0x00 , 0x12 , 0x00 , 0x80 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        1297142246100992000,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_tst64 . parse_peek (& [0x12 , 0x00 , 0x60 , 0x00 , 0x12 , 0x00 , 0x80 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        36028874334666770,
    ),
)

"#]] . raw ()) ; fn be_tsti16 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , i16 > { i16 (Endianness :: Big) . parse_next (i) } fn le_tsti16 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , i16 > { i16 (Endianness :: Little) . parse_next (i) } assert_parse ! (be_tsti16 . parse_peek (& [0x00 , 0x80]) , str ! [[r#"
Ok(
    (
        [],
        128,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_tsti16 . parse_peek (& [0x00 , 0x80]) , str ! [[r#"
Ok(
    (
        [],
        -32768,
    ),
)

"#]] . raw ()) ; fn be_tsti32 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , i32 > { i32 (Endianness :: Big) . parse_next (i) } fn le_tsti32 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , i32 > { i32 (Endianness :: Little) . parse_next (i) } assert_parse ! (be_tsti32 . parse_peek (& [0x00 , 0x12 , 0x60 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        1204224,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_tsti32 . parse_peek (& [0x00 , 0x12 , 0x60 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        6296064,
    ),
)

"#]] . raw ()) ; fn be_tsti64 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , i64 > { i64 (Endianness :: Big) . parse_next (i) } fn le_tsti64 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , i64 > { i64 (Endianness :: Little) . parse_next (i) } assert_parse ! (be_tsti64 . parse_peek (& [0x00 , 0xFF , 0x60 , 0x00 , 0x12 , 0x00 , 0x80 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        71881672479506432,
    ),
)

"#]] . raw ()) ; assert_parse ! (le_tsti64 . parse_peek (& [0x00 , 0xFF , 0x60 , 0x00 , 0x12 , 0x00 , 0x80 , 0x00]) , str ! [[r#"
Ok(
    (
        [],
        36028874334732032,
    ),
)

"#]] . raw ()) ; } }
};
}
