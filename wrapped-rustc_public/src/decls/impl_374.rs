macro_rules! deps {
    () => {
        ImplTrait!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl ImplDef { # [doc = " Retrieve information about this implementation."] pub fn trait_impl (& self) -> ImplTrait { with (| cx | cx . trait_impl (self)) } }
    };
}

impl_374!()