macro_rules! deps {
    () => {
        Ingredient!();
        Zalsa!();
        IngredientIndex!();
    };
}

macro_rules! Jar {
    () => {
        deps!();
        # [doc = " A \"jar\" is a group of ingredients that are added atomically."] # [doc = ""] # [doc = " Each type implementing jar can be added to the database at most once."] pub trait Jar : Any { # [doc = " Create the ingredients given the index of the first one."] # [doc = ""] # [doc = " All subsequent ingredients will be assigned contiguous indices."] fn create_ingredients (zalsa : & mut Zalsa , first_index : IngredientIndex ,) -> Vec < Box < dyn Ingredient > > ; # [doc = " This returns the [`TypeId`] of the ID struct, that is, the struct that wraps `salsa::Id`"] # [doc = " and carry the name of the jar."] fn id_struct_type_id () -> TypeId ; }
    };
}

Jar!()