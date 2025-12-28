macro_rules! deps {
    () => {
        Accumulator!();
        IngredientImpl!();
        Zalsa!();
        ZalsaLocal!();
        JarImpl!();
        IngredientIndex!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < A : Accumulator > IngredientImpl < A > { # [doc = " Find the accumulator ingredient for `A` in the database, if any."] pub fn from_zalsa (zalsa : & Zalsa) -> Option < & Self > { let index = zalsa . lookup_jar_by_type :: < JarImpl < A > > () ; let ingredient = zalsa . lookup_ingredient (index) . assert_type :: < Self > () ; Some (ingredient) } pub fn new (index : IngredientIndex) -> Self { Self { index , phantom : PhantomData , } } pub fn push (& self , zalsa_local : & ZalsaLocal , value : A) { if let Err (()) = zalsa_local . accumulate (self . index , value) { panic ! ("cannot accumulate values outside of an active tracked function") ; } } pub fn index (& self) -> IngredientIndex { self . index } }
    };
}

impl_7!()