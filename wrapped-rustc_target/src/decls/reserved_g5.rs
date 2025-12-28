macro_rules! deps {
    () => {
        Target!();
        InlineAsmArch!();
    };
}

macro_rules! reserved_g5 {
    () => {
        deps!();
        fn reserved_g5 (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if arch == InlineAsmArch :: Sparc { Err ("g5 is reserved for system on SPARC32") } else { Ok (()) } }
    };
}

reserved_g5!();