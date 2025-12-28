macro_rules! deps {
    () => {
        Value!();
        Field!();
        Visit!();
        Empty!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl Value for Empty { # [inline] fn record (& self , _ : & Field , _ : & mut dyn Visit) { } }
    };
}

impl_164!();