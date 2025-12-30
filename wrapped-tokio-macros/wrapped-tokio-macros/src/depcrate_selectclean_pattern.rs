// Generated macro for clean_pattern (function)
macro_rules! Depcrate_selectclean_pattern {
() => {
// Module: crate::select
// Provides: {"clean_pattern"}
// Dependencies: {}
fn clean_pattern (pat : & mut syn :: Pat) { match pat { syn :: Pat :: Lit (_literal) => { } syn :: Pat :: Macro (_macro) => { } syn :: Pat :: Path (_path) => { } syn :: Pat :: Range (_range) => { } syn :: Pat :: Rest (_rest) => { } syn :: Pat :: Verbatim (_tokens) => { } syn :: Pat :: Wild (_underscore) => { } syn :: Pat :: Ident (ident) => { ident . by_ref = None ; ident . mutability = None ; if let Some ((_at , pat)) = & mut ident . subpat { clean_pattern (& mut * pat) ; } } syn :: Pat :: Or (or) => { for case in & mut or . cases { clean_pattern (case) ; } } syn :: Pat :: Slice (slice) => { for elem in & mut slice . elems { clean_pattern (elem) ; } } syn :: Pat :: Struct (struct_pat) => { for field in & mut struct_pat . fields { clean_pattern (& mut field . pat) ; } } syn :: Pat :: Tuple (tuple) => { for elem in & mut tuple . elems { clean_pattern (elem) ; } } syn :: Pat :: TupleStruct (tuple) => { for elem in & mut tuple . elems { clean_pattern (elem) ; } } syn :: Pat :: Reference (reference) => { reference . mutability = None ; clean_pattern (& mut reference . pat) ; } syn :: Pat :: Type (type_pat) => { clean_pattern (& mut type_pat . pat) ; } _ => { } } }
};
}
