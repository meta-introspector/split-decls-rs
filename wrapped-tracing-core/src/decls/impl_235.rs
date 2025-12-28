macro_rules! deps {
    () => {
        Metadata!();
        Parent!();
        ValueSet!();
        Current!();
        Id!();
        FieldSet!();
        Attributes!();
        Visit!();
        Field!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'a > Attributes < 'a > { # [doc = " Returns `Attributes` describing a new child span of the current span,"] # [doc = " with the provided metadata and values."] pub fn new (metadata : & 'static Metadata < 'static > , values : & 'a field :: ValueSet < 'a >) -> Self { Attributes { metadata , values , parent : Parent :: Current , } } # [doc = " Returns `Attributes` describing a new span at the root of its own trace"] # [doc = " tree, with the provided metadata and values."] pub fn new_root (metadata : & 'static Metadata < 'static > , values : & 'a field :: ValueSet < 'a >) -> Self { Attributes { metadata , values , parent : Parent :: Root , } } # [doc = " Returns `Attributes` describing a new child span of the specified"] # [doc = " parent span, with the provided metadata and values."] pub fn child_of (parent : Id , metadata : & 'static Metadata < 'static > , values : & 'a field :: ValueSet < 'a > ,) -> Self { Attributes { metadata , values , parent : Parent :: Explicit (parent) , } } # [doc = " Returns a reference to the new span's metadata."] pub fn metadata (& self) -> & 'static Metadata < 'static > { self . metadata } # [doc = " Returns a reference to a `ValueSet` containing any values the new span"] # [doc = " was created with."] pub fn values (& self) -> & field :: ValueSet < 'a > { self . values } # [doc = " Returns true if the new span should be a root."] pub fn is_root (& self) -> bool { matches ! (self . parent , Parent :: Root) } # [doc = " Returns true if the new span's parent should be determined based on the"] # [doc = " current context."] # [doc = ""] # [doc = " If this is true and the current thread is currently inside a span, then"] # [doc = " that span should be the new span's parent. Otherwise, if the current"] # [doc = " thread is _not_ inside a span, then the new span will be the root of its"] # [doc = " own trace tree."] pub fn is_contextual (& self) -> bool { matches ! (self . parent , Parent :: Current) } # [doc = " Returns the new span's explicitly-specified parent, if there is one."] # [doc = ""] # [doc = " Otherwise (if the new span is a root or is a child of the current span),"] # [doc = " returns `None`."] pub fn parent (& self) -> Option < & Id > { match self . parent { Parent :: Explicit (ref p) => Some (p) , _ => None , } } # [doc = " Records all the fields in this set of `Attributes` with the provided"] # [doc = " [Visitor]."] # [doc = ""] # [doc = " [visitor]: super::field::Visit"] pub fn record (& self , visitor : & mut dyn field :: Visit) { self . values . record (visitor) } # [doc = " Returns `true` if this set of `Attributes` contains a value for the"] # [doc = " given `Field`."] pub fn contains (& self , field : & field :: Field) -> bool { self . values . contains (field) } # [doc = " Returns true if this set of `Attributes` contains _no_ values."] pub fn is_empty (& self) -> bool { self . values . is_empty () } # [doc = " Returns the set of all [fields] defined by this span's [`Metadata`]."] # [doc = ""] # [doc = " Note that the [`FieldSet`] returned by this method includes *all* the"] # [doc = " fields declared by this span, not just those with values that are recorded"] # [doc = " as part of this set of `Attributes`. Other fields with values not present in"] # [doc = " this `Attributes`' value set may [record] values later."] # [doc = ""] # [doc = " [fields]: crate::field"] # [doc = " [record]: Attributes::record()"] # [doc = " [`Metadata`]: crate::metadata::Metadata"] # [doc = " [`FieldSet`]: crate::field::FieldSet"] pub fn fields (& self) -> & FieldSet { self . values . field_set () } }
    };
}

impl_235!()