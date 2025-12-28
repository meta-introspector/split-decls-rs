macro_rules! NonPrimitiveSimdType {
    () => {
        # [derive (Diagnostic)] # [diag (ty_utils_non_primitive_simd_type)] pub (crate) struct NonPrimitiveSimdType < 'tcx > { pub ty : Ty < 'tcx > , pub e_ty : Ty < 'tcx > , }
    };
}

NonPrimitiveSimdType!();