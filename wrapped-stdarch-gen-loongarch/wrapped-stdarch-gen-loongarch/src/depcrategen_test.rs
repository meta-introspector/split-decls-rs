// Generated macro for gen_test (function)
macro_rules! Depcrategen_test {
() => {
// Module: crate
// Provides: {"gen_test"}
// Dependencies: {}
fn gen_test (in_file : String , ext_name : & str) -> io :: Result < () > { let f = File :: open (in_file . clone ()) . unwrap_or_else (| _ | panic ! ("Failed to open {in_file}")) ; let f = BufReader :: new (f) ; let target : TargetFeature = TargetFeature :: new (ext_name) ; let mut para_num ; let mut current_name : Option < String > = None ; let mut asm_fmts : Vec < String > = Vec :: new () ; let mut impl_function_str = String :: new () ; let mut call_function_str = String :: new () ; let mut out = String :: new () ; out . push_str (& format ! (r#"/*
 * This code is automatically generated. DO NOT MODIFY.
 *
 * Instead, modify `{in_file}` and run the following command to re-generate this file:
 *
 * ```
 * OUT_DIR=`pwd`/crates/stdarch-gen-loongarch cargo run -p stdarch-gen-loongarch -- {in_file} test
 * ```
 */

#include <stdio.h>
#include <stdint.h>
#include <lsxintrin.h>
#include <lasxintrin.h>

union v16qi
{{
    __m128i v;
    int64_t i64[2];
    int8_t i8[16];
}};

union v32qi
{{
    __m256i v;
    int64_t i64[4];
    int8_t i8[32];
}};

union v8hi
{{
    __m128i v;
    int64_t i64[2];
    int16_t i16[8];
}};

union v16hi
{{
    __m256i v;
    int64_t i64[4];
    int16_t i16[16];
}};

union v4si
{{
    __m128i v;
    int64_t i64[2];
    int32_t i32[4];
}};

union v8si
{{
    __m256i v;
    int64_t i64[4];
    int32_t i32[8];
}};

union v2di
{{
    __m128i v;
    int64_t i64[2];
}};

union v4di
{{
    __m256i v;
    int64_t i64[4];
}};

union uv16qi
{{
    __m128i v;
    uint64_t i64[2];
    uint8_t i8[16];
}};

union uv32qi
{{
    __m256i v;
    uint64_t i64[4];
    uint8_t i8[32];
}};

union uv8hi
{{
    __m128i v;
    uint64_t i64[2];
    uint16_t i16[8];
}};

union uv16hi
{{
    __m256i v;
    uint64_t i64[4];
    uint16_t i16[16];
}};

union uv4si
{{
    __m128i v;
    uint64_t i64[2];
    uint32_t i32[4];
}};

union uv8si
{{
    __m256i v;
    uint64_t i64[4];
    uint32_t i32[8];
}};

union uv2di
{{
    __m128i v;
    uint64_t i64[2];
}};

union uv4di
{{
    __m256i v;
    uint64_t i64[4];
}};

union v4sf
{{
    __m128 v;
    int64_t i64[2];
    uint32_t i32[2];
    float f32[4];
}};

union v8sf
{{
    __m256 v;
    int64_t i64[4];
    uint32_t i32[4];
    float f32[8];
}};

union v2df
{{
    __m128d v;
    uint64_t i64[2];
    double f64[2];
}};

union v4df
{{
    __m256d v;
    uint64_t i64[4];
    double f64[4];
}};
"#)) ; for line in f . lines () { let line = line . unwrap () ; if line . is_empty () { continue ; } if let Some (name) = line . strip_prefix ("name = ") { current_name = Some (String :: from (name)) ; } else if line . starts_with ("asm-fmts = ") { asm_fmts = line [10 ..] . split (',') . map (| v | v . trim () . to_string ()) . collect () ; } else if line . starts_with ("data-types = ") { let current_name = current_name . clone () . unwrap () ; let data_types : Vec < & str > = line . get (12 ..) . unwrap () . split (',') . map (| e | e . trim ()) . collect () ; let in_t ; let out_t ; if data_types . len () == 2 { in_t = [data_types [1] , "NULL" , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 1 ; } else if data_types . len () == 3 { in_t = [data_types [1] , data_types [2] , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 2 ; } else if data_types . len () == 4 { in_t = [data_types [1] , data_types [2] , data_types [3] , "NULL"] ; out_t = data_types [0] ; para_num = 3 ; } else if data_types . len () == 5 { in_t = [data_types [1] , data_types [2] , data_types [3] , data_types [4]] ; out_t = data_types [0] ; para_num = 4 ; } else { panic ! ("DEBUG: line: {0} len: {1}" , line , data_types . len ()) ; } let (link_function , function) = gen_test_body (& current_name , & asm_fmts , & in_t , out_t , para_num , target) ; impl_function_str . push_str (& link_function) ; call_function_str . push_str (& function) ; } } out . push_str (& impl_function_str) ; out . push ('\n') ; out . push_str ("int main(int argc, char *argv[])\n") ; out . push_str ("{\n") ; out . push_str ("    printf(\"// This code is automatically generated. DO NOT MODIFY.\\n\");\n") ; out . push_str ("    printf(\"// See crates/stdarch-gen-loongarch/README.md\\n\\n\");\n") ; out . push_str ("    printf(\"use crate::{\\n\");\n") ; out . push_str ("    printf(\"    core_arch::{loongarch64::*, simd::*},\\n\");\n") ; out . push_str ("    printf(\"    mem::transmute,\\n\");\n") ; out . push_str ("    printf(\"};\\n\");\n") ; out . push_str ("    printf(\"use stdarch_test::simd_test;\\n\");\n") ; out . push_str (& call_function_str) ; out . push_str ("    return 0;\n") ; out . push ('}') ; let out_dir_path : PathBuf = PathBuf :: from (env :: var ("OUT_DIR") . unwrap ()) ; std :: fs :: create_dir_all (& out_dir_path) ? ; let mut f = File :: create (out_dir_path . join (format ! ("{ext_name}.c"))) ? ; f . write_all (out . as_bytes ()) ? ; Ok (()) }
};
}
