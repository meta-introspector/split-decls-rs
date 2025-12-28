macro_rules! kw {
    () => {
        # [doc = " This module contains all the defined keyword `Symbol`s."] # [doc = ""] # [doc = " Given that `kw` is imported, use them like `kw::keyword_name`."] # [doc = " For example `kw::Loop` or `kw::Break`."] pub mod kw { pub use super :: kw_generated :: * ; }
    };
}

kw!();