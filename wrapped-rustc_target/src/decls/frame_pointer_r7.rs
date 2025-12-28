macro_rules! deps {
    () => {
        Target!();
        InlineAsmArch!();
    };
}

macro_rules! frame_pointer_r7 {
    () => {
        deps!();
        fn frame_pointer_r7 (_arch : InlineAsmArch , _reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if frame_pointer_is_r7 (target_features , target) { Err ("the frame pointer (r7) cannot be used as an operand for inline asm") } else { Ok (()) } }
    };
}

frame_pointer_r7!()