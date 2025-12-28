macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! x86_64_only {
    () => {
        deps!();
        fn x86_64_only (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { match arch { InlineAsmArch :: X86 => Err ("register is only available on x86_64") , InlineAsmArch :: X86_64 => Ok (()) , _ => unreachable ! () , } }
    };
}

x86_64_only!()