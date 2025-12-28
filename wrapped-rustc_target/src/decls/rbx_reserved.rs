macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
    };
}

macro_rules! rbx_reserved {
    () => {
        deps!();
        fn rbx_reserved (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { match arch { InlineAsmArch :: X86 => Ok (()) , InlineAsmArch :: X86_64 => { Err ("rbx is used internally by LLVM and cannot be used as an operand for inline asm") } _ => unreachable ! () , } }
    };
}

rbx_reserved!();