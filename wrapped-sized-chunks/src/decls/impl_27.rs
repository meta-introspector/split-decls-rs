macro_rules! deps {
    () => {
        Chunk!();
        InlineArray!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < A , T > Debug for InlineArray < A , T > where A : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("Chunk") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_27!();