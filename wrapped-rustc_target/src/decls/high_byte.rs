macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! high_byte {
    () => {
        deps!();
        fn high_byte (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { match arch { InlineAsmArch :: X86_64 => Err ("high byte registers cannot be used as an operand on x86_64") , _ => Ok (()) , } }
    };
}

high_byte!();