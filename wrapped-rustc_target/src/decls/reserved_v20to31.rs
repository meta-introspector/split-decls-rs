macro_rules! deps {
    () => {
        InlineAsmArch!();
        Target!();
        ABI!();
    };
}

macro_rules! reserved_v20to31 {
    () => {
        deps!();
        fn reserved_v20to31 (_arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if target . is_like_aix { match & * target . options . abi { "vec-default" => Err ("v20-v31 are reserved on vec-default ABI") , "vec-extabi" => Ok (()) , _ => unreachable ! ("unrecognized AIX ABI") , } } else { Ok (()) } }
    };
}

reserved_v20to31!();