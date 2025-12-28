macro_rules! deps {
    () => {
        Callsite!();
        Field!();
        FieldSet!();
        Identifier!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl Field { # [doc = " Returns an [`Identifier`] that uniquely identifies the [`Callsite`]"] # [doc = " which defines this field."] # [doc = ""] # [doc = " [`Identifier`]: super::callsite::Identifier"] # [doc = " [`Callsite`]: super::callsite::Callsite"] # [inline] pub fn callsite (& self) -> callsite :: Identifier { self . fields . callsite () } # [doc = " Returns a string representing the name of the field."] pub fn name (& self) -> & 'static str { self . fields . names [self . i] } # [doc = " Returns the index of this field in its [`FieldSet`]."] pub fn index (& self) -> usize { self . i } }
    };
}

impl_167!()