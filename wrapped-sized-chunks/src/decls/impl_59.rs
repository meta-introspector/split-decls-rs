macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < A , const N : usize > Debug for Chunk < A , N > where A : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("Chunk") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_59!();