macro_rules! deps {
    () => {
        Configuration!();
        IngredientImpl!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < C > std :: fmt :: Debug for IngredientImpl < C > where C : Configuration , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("index" , & self . ingredient_index) . finish () } }
    };
}

impl_193!()