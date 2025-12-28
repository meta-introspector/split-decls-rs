macro_rules! raw {
    () => {
        # [doc = " Unsafe functions that mirror the API of the C implementation of Ryū."] pub mod raw { pub use crate :: pretty :: { format32 , format64 } ; }
    };
}

raw!()