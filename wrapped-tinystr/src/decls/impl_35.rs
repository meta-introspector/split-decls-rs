macro_rules! deps {
    () => {
        TinyAsciiStr!();
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < const N : usize > From < TinyAsciiStr < N > > for UnvalidatedTinyAsciiStr < N > { fn from (other : TinyAsciiStr < N >) -> Self { other . to_unvalidated () } }
    };
}

impl_35!();