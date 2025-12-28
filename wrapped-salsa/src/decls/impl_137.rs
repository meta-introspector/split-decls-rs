macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl Debug for Id { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if self . generation () == 0 { write ! (f , "Id({:x})" , self . index ()) } else { write ! (f , "Id({:x}g{:x})" , self . index () , self . generation ()) } } }
    };
}

impl_137!();