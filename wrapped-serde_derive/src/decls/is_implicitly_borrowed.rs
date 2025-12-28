macro_rules! is_implicitly_borrowed {
    () => {
        fn is_implicitly_borrowed (ty : & syn :: Type) -> bool { is_implicitly_borrowed_reference (ty) || is_option (ty , is_implicitly_borrowed_reference) }
    };
}

is_implicitly_borrowed!();