macro_rules! deps {
    () => {
        Visit!();
        Field!();
        Empty!();
        Value!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl Value for Empty { # [inline] fn record (& self , _ : & Field , _ : & mut dyn Visit) { } }
    };
}

impl_164!()