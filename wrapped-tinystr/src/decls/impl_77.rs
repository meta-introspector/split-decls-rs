macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < const N : usize > Bake for TinyAsciiStr < N > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("tinystr") ; let string = self . as_str () ; quote ! { tinystr :: tinystr ! (# N , # string) } } }
    };
}

impl_77!();