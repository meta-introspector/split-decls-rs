// Generated macro for generate_file (function)
macro_rules! Depcrategenerate_file {
() => {
// Module: crate
// Provides: {"generate_file"}
// Dependencies: {}
fn generate_file (generated_input : input :: GeneratorInput , mut out : Box < dyn Write > ,) -> std :: io :: Result < () > { write ! (out , r#"// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen-arm/spec/` and run the following command to re-generate this file:
//
// ```
// cargo run --bin=stdarch-gen-arm -- crates/stdarch-gen-arm/spec
// ```
#![allow(improper_ctypes)]

#[cfg(test)]
use stdarch_test::assert_instr;

use super::*;{uses_neon}

"# , uses_neon = if generated_input . ctx . uses_neon_types { "\nuse crate::core_arch::arch::aarch64::*;" } else { "" } ,) ? ; let intrinsics = generated_input . intrinsics ; format_code (out , quote ! { # (# intrinsics) * }) ? ; Ok (()) }
};
}
