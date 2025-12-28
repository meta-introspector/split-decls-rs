macro_rules! deps {
    () => {
        InlineAsmArch!();
        InlineAsmType!();
        ModifierInfo!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl PowerPCInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg | Self :: reg_nonzero => { if arch == InlineAsmArch :: PowerPC { types ! { _ : I8 , I16 , I32 ; } } else { types ! { _ : I8 , I16 , I32 , I64 ; } } } Self :: freg => types ! { _ : F32 , F64 ; } , Self :: vreg => types ! { altivec : VecI8 (16) , VecI16 (8) , VecI32 (4) , VecF32 (4) ; vsx : F32 , F64 , VecI64 (2) , VecF64 (2) ; } , Self :: cr | Self :: xer => & [] , } } }
    };
}

impl_69!();