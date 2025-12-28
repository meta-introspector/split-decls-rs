macro_rules! deps {
    () => {
        IngredientIndices!();
        IngredientIndex!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl IngredientIndices { # [inline] pub fn empty () -> Self { Self { indices : Box :: default () , } } pub fn merge (iter : impl IntoIterator < Item = Self >) -> Self { let mut indices = Vec :: new () ; for index in iter { indices . extend (index . indices) ; } indices . sort_unstable () ; indices . dedup () ; Self { indices : indices . into_boxed_slice () , } } pub fn iter (& self) -> impl Iterator < Item = IngredientIndex > + '_ { self . indices . iter () . copied () } }
    };
}

impl_230!()