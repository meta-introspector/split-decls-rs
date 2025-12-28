macro_rules! deps {
    () => {
        Alt!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < V > Alt < V > { # [doc = " Wraps the provided visitor so that any `fmt::Debug` fields are formatted"] # [doc = " using the alternative (`:#`) formatter."] pub fn new (inner : V) -> Self { Alt (inner) } }
    };
}

impl_6!()