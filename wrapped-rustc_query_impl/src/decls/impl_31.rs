macro_rules! deps {
    () => {
        QueryKeyStringCache!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl QueryKeyStringCache { fn new () -> QueryKeyStringCache { QueryKeyStringCache { def_id_cache : Default :: default () } } }
    };
}

impl_31!()