macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! reserved_r13 {
    () => {
        deps!();
        fn reserved_r13 (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if target . is_like_aix && arch == InlineAsmArch :: PowerPC { Ok (()) } else { Err ("r13 is a reserved register on this target") } }
    };
}

reserved_r13!();