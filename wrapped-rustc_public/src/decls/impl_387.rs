macro_rules! deps {
    () => {
        FnAbi!();
        PolyFnSig!();
        Error!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl PolyFnSig { # [doc = " Compute a `FnAbi` suitable for indirect calls, i.e. to `fn` pointers."] # [doc = ""] # [doc = " NB: this doesn't handle virtual calls - those should use `Instance::fn_abi`"] # [doc = " instead, where the instance is an `InstanceKind::Virtual`."] pub fn fn_ptr_abi (self) -> Result < FnAbi , Error > { with (| cx | cx . fn_ptr_abi (self)) } }
    };
}

impl_387!()