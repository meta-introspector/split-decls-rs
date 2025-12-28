macro_rules! deps {
    () => {
        Target!();
        InlineAsmArch!();
    };
}

macro_rules! frame_pointer_r11 {
    () => {
        deps!();
        fn frame_pointer_r11 (arch : InlineAsmArch , reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , target : & Target , is_clobber : bool ,) -> Result < () , & 'static str > { not_thumb1 (arch , reloc_model , target_features , target , is_clobber) ? ; if ! frame_pointer_is_r7 (target_features , target) { Err ("the frame pointer (r11) cannot be used as an operand for inline asm") } else { Ok (()) } }
    };
}

frame_pointer_r11!();