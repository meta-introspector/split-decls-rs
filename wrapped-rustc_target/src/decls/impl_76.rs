macro_rules! deps {
    () => {
        ModifierInfo!();
        InlineAsmArch!();
        InlineAsmType!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl RiscVInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => { if arch == InlineAsmArch :: RiscV64 { types ! { _ : I8 , I16 , I32 , I64 , F16 , F32 , F64 ; } } else { types ! { _ : I8 , I16 , I32 , F16 , F32 ; } } } Self :: freg => types ! { f : F16 , F32 ; d : F64 ; } , Self :: vreg => & [] , } } }
    };
}

impl_76!();