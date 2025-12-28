macro_rules! deps {
    () => {
        Ty!();
        FnSig!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl FnSig { pub fn output (& self) -> Ty { self . inputs_and_output [self . inputs_and_output . len () - 1] } pub fn inputs (& self) -> & [Ty] { & self . inputs_and_output [.. self . inputs_and_output . len () - 1] } }
    };
}

impl_389!();