macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! StaticTreeDesc {
    () => {
        deps!();
        pub (crate) struct StaticTreeDesc { # [doc = " static tree or NULL"] pub (crate) static_tree : & 'static [Value] , # [doc = " extra bits for each code or NULL"] extra_bits : & 'static [u8] , # [doc = " base index for extra_bits"] extra_base : usize , # [doc = " max number of elements in the tree"] elems : usize , # [doc = " max bit length for the codes"] max_length : u16 , }
    };
}

StaticTreeDesc!();