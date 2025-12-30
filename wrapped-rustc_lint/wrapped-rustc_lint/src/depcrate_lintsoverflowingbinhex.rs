// Generated macro for OverflowingBinHex (struct)
macro_rules! Depcrate_lintsOverflowingBinHex {
() => {
// Module: crate::lints
// Provides: {"OverflowingBinHex"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_overflowing_bin_hex)] pub (crate) struct OverflowingBinHex < 'a > { pub ty : & 'a str , pub lit : String , pub dec : u128 , pub actually : String , # [subdiagnostic] pub sign : OverflowingBinHexSign , # [subdiagnostic] pub sub : Option < OverflowingBinHexSub < 'a > > , # [subdiagnostic] pub sign_bit_sub : Option < OverflowingBinHexSignBitSub < 'a > > , }
};
}
