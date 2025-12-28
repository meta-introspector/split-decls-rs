macro_rules! deps {
    () => {
        Primitive!();
        Size!();
        MachineInfo!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Primitive { pub fn size (self , target : & MachineInfo) -> Size { match self { Primitive :: Int { length , .. } => Size :: from_bits (length . bits ()) , Primitive :: Float { length } => Size :: from_bits (length . bits ()) , Primitive :: Pointer (_) => target . pointer_width , } } }
    };
}

impl_32!()