// Generated macro for PREAMBLE (static)
macro_rules! Depcrate_load_store_testsPREAMBLE {
() => {
// Module: crate::load_store_tests
// Provides: {"PREAMBLE"}
// Dependencies: {}
static PREAMBLE : LazyLock < String > = LazyLock :: new (| | { format ! (r#"#![allow(unused)]

use super::*;
use std::boxed::Box;
use std::convert::{{TryFrom, TryInto}};
use std::sync::LazyLock;
use std::vec::Vec;
use stdarch_test::simd_test;

static F32_DATA: LazyLock<[f32; {LEN_F32} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_F32} * {NUM_VECS})
        .map(|i| i as f32)
        .collect::<Vec<_>>()
        .try_into()
        .expect("f32 data incorrectly initialised")
}});
static F64_DATA: LazyLock<[f64; {LEN_F64} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_F64} * {NUM_VECS})
        .map(|i| i as f64)
        .collect::<Vec<_>>()
        .try_into()
        .expect("f64 data incorrectly initialised")
}});
static I8_DATA: LazyLock<[i8; {LEN_I8} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_I8} * {NUM_VECS})
        .map(|i| ((i + 128) % 256 - 128) as i8)
        .collect::<Vec<_>>()
        .try_into()
        .expect("i8 data incorrectly initialised")
}});
static I16_DATA: LazyLock<[i16; {LEN_I16} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_I16} * {NUM_VECS})
        .map(|i| i as i16)
        .collect::<Vec<_>>()
        .try_into()
        .expect("i16 data incorrectly initialised")
}});
static I32_DATA: LazyLock<[i32; {LEN_I32} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_I32} * {NUM_VECS})
        .map(|i| i as i32)
        .collect::<Vec<_>>()
        .try_into()
        .expect("i32 data incorrectly initialised")
}});
static I64_DATA: LazyLock<[i64; {LEN_I64} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_I64} * {NUM_VECS})
        .map(|i| i as i64)
        .collect::<Vec<_>>()
        .try_into()
        .expect("i64 data incorrectly initialised")
}});
static U8_DATA: LazyLock<[u8; {LEN_U8} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_U8} * {NUM_VECS})
        .map(|i| i as u8)
        .collect::<Vec<_>>()
        .try_into()
        .expect("u8 data incorrectly initialised")
}});
static U16_DATA: LazyLock<[u16; {LEN_U16} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_U16} * {NUM_VECS})
        .map(|i| i as u16)
        .collect::<Vec<_>>()
        .try_into()
        .expect("u16 data incorrectly initialised")
}});
static U32_DATA: LazyLock<[u32; {LEN_U32} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_U32} * {NUM_VECS})
        .map(|i| i as u32)
        .collect::<Vec<_>>()
        .try_into()
        .expect("u32 data incorrectly initialised")
}});
static U64_DATA: LazyLock<[u64; {LEN_U64} * {NUM_VECS}]> = LazyLock::new(|| {{
    (0..{LEN_U64} * {NUM_VECS})
        .map(|i| i as u64)
        .collect::<Vec<_>>()
        .try_into()
        .expect("u64 data incorrectly initialised")
}});

#[target_feature(enable = "sve")]
fn assert_vector_matches_f32(vector: svfloat32_t, expected: svfloat32_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b32(), defined));
    let cmp = svcmpne_f32(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_f64(vector: svfloat64_t, expected: svfloat64_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b64(), defined));
    let cmp = svcmpne_f64(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_i8(vector: svint8_t, expected: svint8_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b8(), defined));
    let cmp = svcmpne_s8(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_i16(vector: svint16_t, expected: svint16_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b16(), defined));
    let cmp = svcmpne_s16(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_i32(vector: svint32_t, expected: svint32_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b32(), defined));
    let cmp = svcmpne_s32(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_i64(vector: svint64_t, expected: svint64_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b64(), defined));
    let cmp = svcmpne_s64(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_u8(vector: svuint8_t, expected: svuint8_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b8(), defined));
    let cmp = svcmpne_u8(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_u16(vector: svuint16_t, expected: svuint16_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b16(), defined));
    let cmp = svcmpne_u16(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_u32(vector: svuint32_t, expected: svuint32_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b32(), defined));
    let cmp = svcmpne_u32(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}

#[target_feature(enable = "sve")]
fn assert_vector_matches_u64(vector: svuint64_t, expected: svuint64_t) {{
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b64(), defined));
    let cmp = svcmpne_u64(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}}
"#) }) ;
};
}
