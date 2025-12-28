macro_rules! UncoveredTyParams {
    () => {
        # [derive_where (Debug ; I : Interner , T : Debug)] pub struct UncoveredTyParams < I : Interner , T > { pub uncovered : T , pub local_ty : Option < I :: Ty > , }
    };
}

UncoveredTyParams!()