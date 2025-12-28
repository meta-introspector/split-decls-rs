macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        pub (crate) struct Builder < T > { slab : Slab < T > , vacant_list_broken : bool , first_vacant_index : Option < usize > , }
    };
}

Builder!();