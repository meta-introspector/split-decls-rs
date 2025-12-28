macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl fmt :: Debug for DatabaseKeyIndex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { crate :: attach :: with_attached_database (| db | { let ingredient = db . zalsa () . lookup_ingredient (self . ingredient_index ()) ; ingredient . fmt_index (self . key_index () , f) }) . unwrap_or_else (| | { f . debug_tuple ("DatabaseKeyIndex") . field (& self . ingredient_index ()) . field (& self . key_index ()) . finish () }) } }
    };
}

impl_226!()