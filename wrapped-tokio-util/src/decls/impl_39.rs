macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Clone for CancellationToken { # [doc = " Creates a clone of the [`CancellationToken`] which will get cancelled"] # [doc = " whenever the current token gets cancelled, and vice versa."] fn clone (& self) -> Self { tree_node :: increase_handle_refcount (& self . inner) ; CancellationToken { inner : self . inner . clone () , } } }
    };
}

impl_39!()