macro_rules! deps {
    () => {
        DurabilityVal!();
        Durability!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Durability { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if f . alternate () { match self . 0 { DurabilityVal :: Low => f . write_str ("Durability::LOW") , DurabilityVal :: Medium => f . write_str ("Durability::MEDIUM") , DurabilityVal :: High => f . write_str ("Durability::HIGH") , } } else { f . debug_tuple ("Durability") . field (& (self . 0 as usize)) . finish () } } }
    };
}

impl_92!();