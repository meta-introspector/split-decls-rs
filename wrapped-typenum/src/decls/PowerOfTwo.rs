macro_rules! PowerOfTwo {
    () => {
        # [doc = " The **marker trait** for type-level numbers which are a power of two."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Here's a working example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use typenum::{PowerOfTwo, P4, P8};"] # [doc = ""] # [doc = " fn only_p2<P: PowerOfTwo>() {}"] # [doc = ""] # [doc = " only_p2::<P4>();"] # [doc = " only_p2::<P8>();"] # [doc = " ```"] # [doc = ""] # [doc = " Numbers which are not a power of two will fail to compile in this example:"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " use typenum::{P9, P511, P1023, PowerOfTwo};"] # [doc = ""] # [doc = " fn only_p2<P: PowerOfTwo>() { }"] # [doc = ""] # [doc = " only_p2::<P9>();"] # [doc = " only_p2::<P511>();"] # [doc = " only_p2::<P1023>();"] # [doc = " ```"] pub trait PowerOfTwo : Sealed { }
    };
}

PowerOfTwo!();