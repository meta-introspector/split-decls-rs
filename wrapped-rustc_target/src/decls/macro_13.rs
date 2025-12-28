macro_rules! macro_13 {
    () => {
        def_reg_class ! { Arm ArmInlineAsmRegClass { reg , sreg , sreg_low16 , dreg , dreg_low16 , dreg_low8 , qreg , qreg_low8 , qreg_low4 , } }
    };
}

macro_13!();