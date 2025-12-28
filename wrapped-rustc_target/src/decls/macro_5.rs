macro_rules! macro_5 {
    () => {
        def_reg_class ! { AArch64 AArch64InlineAsmRegClass { reg , vreg , vreg_low16 , preg , } }
    };
}

macro_5!();