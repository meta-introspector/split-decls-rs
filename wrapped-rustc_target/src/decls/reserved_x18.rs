macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! reserved_x18 {
    () => {
        deps!();
        fn reserved_x18 (_arch : InlineAsmArch , _reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if target_reserves_x18 (target , target_features) { Err ("x18 is a reserved register on this target") } else { Ok (()) } }
    };
}

reserved_x18!()