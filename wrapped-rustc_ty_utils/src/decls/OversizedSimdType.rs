macro_rules! OversizedSimdType {
    () => {
        # [derive (Diagnostic)] # [diag (ty_utils_oversized_simd_type)] pub (crate) struct OversizedSimdType < 'tcx > { pub ty : Ty < 'tcx > , pub max_lanes : u64 , }
    };
}

OversizedSimdType!();