macro_rules! IntoPointer {
    () => {
        pub trait IntoPointer { # [doc = " Returns a pointer which outlives `self`."] fn into_pointer (& self) -> * const () ; }
    };
}

IntoPointer!()