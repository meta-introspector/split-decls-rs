macro_rules! deps {
    () => {
        IngredientImpl!();
        Accumulator!();
        IngredientIndex!();
        Ingredient!();
        Jar!();
        JarImpl!();
        Zalsa!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < A : Accumulator > Jar for JarImpl < A > { fn create_ingredients (_zalsa : & mut Zalsa , first_index : IngredientIndex ,) -> Vec < Box < dyn Ingredient > > { vec ! [Box :: new (< IngredientImpl < A >>:: new (first_index))] } fn id_struct_type_id () -> TypeId { TypeId :: of :: < A > () } }
    };
}

impl_5!()