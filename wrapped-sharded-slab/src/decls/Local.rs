macro_rules! Local {
    () => {
        pub (crate) struct Local { # [doc = " Index of the first slot on the local free list"] head : UnsafeCell < usize > , }
    };
}

Local!()