macro_rules! deps {
    () => {
        Configuration!();
        Zalsa!();
        IngredientIndex!();
        JarImpl!();
        Ingredient!();
        IngredientImpl!();
        Jar!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < C : Configuration > Jar for JarImpl < C > { fn create_ingredients (_zalsa : & mut Zalsa , struct_index : crate :: zalsa :: IngredientIndex ,) -> Vec < Box < dyn Ingredient > > { let struct_ingredient = < IngredientImpl < C > > :: new (struct_index) ; let tracked_field_ingredients = C :: TRACKED_FIELD_INDICES . iter () . copied () . map (| tracked_index | { Box :: new (< FieldIngredientImpl < C > > :: new (tracked_index , struct_index . successor (tracked_index) ,)) as _ }) ; std :: iter :: once (Box :: new (struct_ingredient) as _) . chain (tracked_field_ingredients) . collect () } fn id_struct_type_id () -> TypeId { TypeId :: of :: < C :: Struct < 'static > > () } }
    };
}

impl_354!()