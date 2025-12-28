macro_rules! deps {
    () => {
        Identifier!();
        ValueSet!();
        Visit!();
        FieldSet!();
        Field!();
        Callsite!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl ValueSet < '_ > { # [doc = " Returns an [`Identifier`] that uniquely identifies the [`Callsite`]"] # [doc = " defining the fields this `ValueSet` refers to."] # [doc = ""] # [doc = " [`Identifier`]: super::callsite::Identifier"] # [doc = " [`Callsite`]: super::callsite::Callsite"] # [inline] pub fn callsite (& self) -> callsite :: Identifier { self . fields . callsite () } # [doc = " Visits all the fields in this `ValueSet` with the provided [visitor]."] # [doc = ""] # [doc = " [visitor]: Visit"] pub fn record (& self , visitor : & mut dyn Visit) { let my_callsite = self . callsite () ; for (field , value) in self . values { if field . callsite () != my_callsite { continue ; } if let Some (value) = value { value . record (field , visitor) ; } } } # [doc = " Returns the number of fields in this `ValueSet` that would be visited"] # [doc = " by a given [visitor] to the [`ValueSet::record()`] method."] # [doc = ""] # [doc = " [visitor]: Visit"] # [doc = " [`ValueSet::record()`]: ValueSet::record()"] pub fn len (& self) -> usize { let my_callsite = self . callsite () ; self . values . iter () . filter (| (field , _) | field . callsite () == my_callsite) . count () } # [doc = " Returns `true` if this `ValueSet` contains a value for the given `Field`."] pub (crate) fn contains (& self , field : & Field) -> bool { field . callsite () == self . callsite () && self . values . iter () . any (| (key , val) | * key == field && val . is_some ()) } # [doc = " Returns true if this `ValueSet` contains _no_ values."] pub fn is_empty (& self) -> bool { let my_callsite = self . callsite () ; self . values . iter () . all (| (key , val) | val . is_none () || key . callsite () != my_callsite) } pub (crate) fn field_set (& self) -> & FieldSet { self . fields } }
    };
}

impl_181!();