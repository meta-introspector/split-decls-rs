macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! UnvalidatedTinyAsciiStr {
    () => {
        deps!();
        # [doc = " A fixed-length bytes array that is expected to be an ASCII string but does not enforce that invariant."] # [doc = ""] # [doc = " Use this type instead of `TinyAsciiStr` if you don't need to enforce ASCII during deserialization. For"] # [doc = " example, strings that are keys of a map don't need to ever be reified as `TinyAsciiStr`s."] # [doc = ""] # [doc = " The main advantage of this type over `[u8; N]` is that it serializes as a string in"] # [doc = " human-readable formats like JSON."] # [derive (PartialEq , PartialOrd , Eq , Ord , Clone , Copy)] pub struct UnvalidatedTinyAsciiStr < const N : usize > (pub (crate) [u8 ; N]) ;
    };
}

UnvalidatedTinyAsciiStr!()