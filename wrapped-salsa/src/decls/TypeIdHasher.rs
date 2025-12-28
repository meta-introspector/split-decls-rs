macro_rules! TypeIdHasher {
    () => {
        # [derive (Default)] pub (crate) struct TypeIdHasher (u64) ;
    };
}

TypeIdHasher!()