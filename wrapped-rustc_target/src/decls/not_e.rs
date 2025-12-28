macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! not_e {
    () => {
        deps!();
        fn not_e (_arch : InlineAsmArch , _reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if is_e (target_features) { Err ("register can't be used with the `e` target feature") } else { Ok (()) } }
    };
}

not_e!()