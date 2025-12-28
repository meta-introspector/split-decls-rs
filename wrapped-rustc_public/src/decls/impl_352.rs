macro_rules! deps {
    () => {
        GenericArgs!();
        VariantIdx!();
        Discr!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl CoroutineDef { # [doc = " Retrieves the body of the coroutine definition. Returns None if the body"] # [doc = " isn't available."] pub fn body (& self) -> Option < Body > { with (| cx | cx . has_body (self . 0) . then (| | cx . mir_body (self . 0))) } pub fn discriminant_for_variant (& self , args : & GenericArgs , idx : VariantIdx) -> Discr { with (| cx | cx . coroutine_discr_for_variant (* self , args , idx)) } }
    };
}

impl_352!();