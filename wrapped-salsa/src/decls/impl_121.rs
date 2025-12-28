macro_rules! deps {
    () => {
        IngredientImpl!();
        Configuration!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < C > std :: fmt :: Debug for IngredientImpl < C > where C : Configuration , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("index" , & self . index) . finish () } }
    };
}

impl_121!()