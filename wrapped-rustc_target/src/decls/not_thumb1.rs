macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! not_thumb1 {
    () => {
        deps!();
        fn not_thumb1 (_arch : InlineAsmArch , _reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , _target : & Target , is_clobber : bool ,) -> Result < () , & 'static str > { if ! is_clobber && target_features . contains (& sym :: thumb_mode) && ! target_features . contains (& sym :: thumb2) { Err ("high registers (r8+) can only be used as clobbers in Thumb-1 code") } else { Ok (()) } }
    };
}

not_thumb1!();