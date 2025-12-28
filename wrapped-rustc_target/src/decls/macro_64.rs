macro_rules! macro_64 {
    () => {
        def_reg_class ! { Nvptx NvptxInlineAsmRegClass { reg16 , reg32 , reg64 , } }
    };
}

macro_64!()