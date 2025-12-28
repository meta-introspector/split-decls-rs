macro_rules! is_str {
    () => {
        fn is_str (ty : & syn :: Type) -> bool { is_primitive_type (ty , "str") }
    };
}

is_str!();