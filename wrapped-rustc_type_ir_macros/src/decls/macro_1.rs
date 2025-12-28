macro_rules! macro_1 {
    () => {
        decl_derive ! ([TypeFoldable_Generic , attributes (type_foldable)] => type_foldable_derive) ;
    };
}

macro_1!();