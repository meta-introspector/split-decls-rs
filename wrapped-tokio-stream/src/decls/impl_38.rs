macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < St : ? Sized > StreamExt for St where St : Stream { }
    };
}

impl_38!()