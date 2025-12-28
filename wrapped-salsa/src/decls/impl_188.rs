macro_rules! deps {
    () => {
        Configuration!();
        JarImpl!();
        Zalsa!();
        IngredientIndex!();
        IngredientImpl!();
        Ingredient!();
        Jar!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < C : Configuration > Jar for JarImpl < C > { fn create_ingredients (_zalsa : & mut Zalsa , first_index : IngredientIndex ,) -> Vec < Box < dyn Ingredient > > { vec ! [Box :: new (IngredientImpl ::< C >:: new (first_index)) as _] } fn id_struct_type_id () -> TypeId { TypeId :: of :: < C :: Struct < 'static > > () } }
    };
}

impl_188!()