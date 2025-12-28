macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl std :: default :: Default for Group { fn default () -> Self { Group { span : Span :: call_site () , } } }
    };
}

impl_49!();