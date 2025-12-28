macro_rules! deps {
    () => {
        Ctxt!();
        Container!();
        Data!();
    };
}

macro_rules! precondition_sized {
    () => {
        deps!();
        fn precondition_sized (cx : & Ctxt , cont : & Container) { if let Data :: Struct (_ , fields) = & cont . data { if let Some (last) = fields . last () { if let syn :: Type :: Slice (_) = ungroup (last . ty) { cx . error_spanned_by (cont . original , "cannot deserialize a dynamically sized struct" ,) ; } } } }
    };
}

precondition_sized!();