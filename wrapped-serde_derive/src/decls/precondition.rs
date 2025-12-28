macro_rules! deps {
    () => {
        Field!();
        Ctxt!();
        Container!();
        Identifier!();
        Variant!();
    };
}

macro_rules! precondition {
    () => {
        deps!();
        fn precondition (cx : & Ctxt , cont : & Container) { match cont . attrs . identifier () { attr :: Identifier :: No => { } attr :: Identifier :: Field => { cx . error_spanned_by (cont . original , "field identifiers cannot be serialized") ; } attr :: Identifier :: Variant => { cx . error_spanned_by (cont . original , "variant identifiers cannot be serialized") ; } } }
    };
}

precondition!();