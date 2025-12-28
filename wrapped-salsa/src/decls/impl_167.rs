macro_rules! deps {
    () => {
        IngredientImpl!();
        Configuration!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < C : Configuration > std :: fmt :: Debug for IngredientImpl < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("index" , & self . ingredient_index) . finish () } }
    };
}

impl_167!();