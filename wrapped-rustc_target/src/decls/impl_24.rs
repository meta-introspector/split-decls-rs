macro_rules! deps {
    () => {
        InlineAsmType!();
        ModifierInfo!();
        InlineAsmArch!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl AvrInlineAsmRegClass { pub fn valid_modifiers (self , _arch : InlineAsmArch) -> & 'static [char] { match self { Self :: reg_pair | Self :: reg_iw | Self :: reg_ptr => & ['h' , 'l'] , _ => & [] , } } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , _arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => types ! { _ : I8 ; } , Self :: reg_upper => types ! { _ : I8 ; } , Self :: reg_pair => types ! { _ : I16 ; } , Self :: reg_iw => types ! { _ : I16 ; } , Self :: reg_ptr => types ! { _ : I16 ; } , } } }
    };
}

impl_24!();