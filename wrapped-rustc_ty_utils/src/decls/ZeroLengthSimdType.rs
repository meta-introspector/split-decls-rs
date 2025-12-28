macro_rules! ZeroLengthSimdType {
    () => {
        # [derive (Diagnostic)] # [diag (ty_utils_zero_length_simd_type)] pub (crate) struct ZeroLengthSimdType < 'tcx > { pub ty : Ty < 'tcx > , }
    };
}

ZeroLengthSimdType!()