macro_rules! deps {
    () => {
        Errors!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl Errors { fn error (& mut self , span : Span , message : String) { self . list . push (syn :: Error :: new (span , message)) ; } }
    };
}

impl_133!()