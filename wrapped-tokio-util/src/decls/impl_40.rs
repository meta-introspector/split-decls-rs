macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Drop for CancellationToken { fn drop (& mut self) { tree_node :: decrease_handle_refcount (& self . inner) ; } }
    };
}

impl_40!();