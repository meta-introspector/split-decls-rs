macro_rules! impl_350 {
    () => {
        impl ClosureDef { # [doc = " Retrieves the body of the closure definition. Returns None if the body"] # [doc = " isn't available."] pub fn body (& self) -> Option < Body > { with (| ctx | ctx . has_body (self . 0) . then (| | ctx . mir_body (self . 0))) } }
    };
}

impl_350!()