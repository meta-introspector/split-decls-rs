macro_rules! deps {
    () => {
        DurabilityVal!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl From < u8 > for DurabilityVal { fn from (value : u8) -> Self { match value { 0 => DurabilityVal :: Low , 1 => DurabilityVal :: Medium , 2 => DurabilityVal :: High , _ => panic ! ("invalid durability") , } } }
    };
}

impl_94!()