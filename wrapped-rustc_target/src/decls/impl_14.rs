macro_rules! deps {
    () => {
        ModifierInfo!();
        InlineAsmType!();
        InlineAsmArch!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl ArmInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { match self { Self :: qreg | Self :: qreg_low8 | Self :: qreg_low4 => & ['e' , 'f'] , _ => & [] , } } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , _arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => types ! { _ : I8 , I16 , I32 , F16 , F32 ; } , Self :: sreg | Self :: sreg_low16 => types ! { vfp2 : I32 , F16 , F32 ; } , Self :: dreg_low16 | Self :: dreg_low8 => types ! { vfp2 : I64 , F64 ; neon : VecI8 (8) , VecI16 (4) , VecI32 (2) , VecI64 (1) , VecF16 (4) , VecF32 (2) ; } , Self :: dreg => types ! { d32 : I64 , F64 ; neon : VecI8 (8) , VecI16 (4) , VecI32 (2) , VecI64 (1) , VecF16 (4) , VecF32 (2) ; } , Self :: qreg | Self :: qreg_low8 | Self :: qreg_low4 => types ! { neon : VecI8 (16) , VecI16 (8) , VecI32 (4) , VecI64 (2) , VecF16 (8) , VecF32 (4) ; } , } } }
    };
}

impl_14!();