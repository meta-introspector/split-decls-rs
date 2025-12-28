macro_rules! Flags {
    () => {
        pub trait Flags { fn flags (& self) -> TypeFlags ; fn outer_exclusive_binder (& self) -> ty :: DebruijnIndex ; }
    };
}

Flags!();