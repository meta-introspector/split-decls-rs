macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! restricted_for_arm64ec {
    () => {
        deps!();
        fn restricted_for_arm64ec (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if arch == InlineAsmArch :: Arm64EC { Err ("x13, x14, x23, x24, x28, v16-v31, p*, ffr cannot be used for Arm64EC") } else { Ok (()) } }
    };
}

restricted_for_arm64ec!()