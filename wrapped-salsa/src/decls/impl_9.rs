macro_rules! deps {
    () => {
        IngredientImpl!();
        Accumulator!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < A > std :: fmt :: Debug for IngredientImpl < A > where A : Accumulator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("index" , & self . index) . finish () } }
    };
}

impl_9!();