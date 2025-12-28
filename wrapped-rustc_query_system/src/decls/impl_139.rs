macro_rules! deps {
    () => {
        QueryState!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < K , I > Default for QueryState < K , I > { fn default () -> QueryState < K , I > { QueryState { active : Default :: default () } } }
    };
}

impl_139!();