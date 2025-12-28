macro_rules! macro_101 {
    () => {
        def_reg_class ! { X86 X86InlineAsmRegClass { reg , reg_abcd , reg_byte , xmm_reg , ymm_reg , zmm_reg , kreg , kreg0 , mmx_reg , x87_reg , tmm_reg , } }
    };
}

macro_101!();