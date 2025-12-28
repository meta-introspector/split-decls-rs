macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Value { pub (crate) const fn new (a : u16 , b : u16) -> Self { Self { a , b } } pub (crate) fn freq_mut (& mut self) -> & mut u16 { & mut self . a } pub (crate) fn code_mut (& mut self) -> & mut u16 { & mut self . a } pub (crate) fn dad_mut (& mut self) -> & mut u16 { & mut self . b } pub (crate) fn len_mut (& mut self) -> & mut u16 { & mut self . b } # [inline (always)] pub (crate) const fn freq (self) -> u16 { self . a } # [inline (always)] pub (crate) const fn code (self) -> u16 { self . a } # [inline (always)] pub (crate) const fn dad (self) -> u16 { self . b } # [inline (always)] pub (crate) const fn len (self) -> u16 { self . b } }
    };
}

impl_144!()