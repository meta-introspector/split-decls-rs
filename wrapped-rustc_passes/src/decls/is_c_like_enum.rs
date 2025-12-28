macro_rules! is_c_like_enum {
    () => {
        fn is_c_like_enum (item : & Item < '_ >) -> bool { if let ItemKind :: Enum (_ , _ , ref def) = item . kind { for variant in def . variants { match variant . data { hir :: VariantData :: Unit (..) => { } _ => return false , } } true } else { false } }
    };
}

is_c_like_enum!()