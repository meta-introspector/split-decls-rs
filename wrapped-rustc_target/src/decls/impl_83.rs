macro_rules! deps {
    () => {
        InlineAsmArch!();
        ModifierInfo!();
        InlineAsmType!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl S390xInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , _arch : InlineAsmArch , allow_experimental_reg : bool ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg | Self :: reg_addr => types ! { _ : I8 , I16 , I32 , I64 ; } , Self :: freg => types ! { _ : F32 , F64 ; } , Self :: vreg => { if allow_experimental_reg { types ! { vector : I32 , F32 , I64 , F64 , I128 , F128 , VecI8 (16) , VecI16 (8) , VecI32 (4) , VecI64 (2) , VecF32 (4) , VecF64 (2) ; } } else { & [] } } Self :: areg => & [] , } } }
    };
}

impl_83!()