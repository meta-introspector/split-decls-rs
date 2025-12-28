macro_rules! deps {
    () => {
        Sealed!();
        DeValue!();
    };
}

macro_rules! Index {
    () => {
        deps!();
        # [doc = " Types that can be used to index a `toml::Value`"] # [doc = ""] # [doc = " Currently this is implemented for `usize` to index arrays and `str` to index"] # [doc = " tables."] # [doc = ""] # [doc = " This trait is sealed and not intended for implementation outside of the"] # [doc = " `toml` crate."] pub trait Index : Sealed { # [doc (hidden)] fn index < 'r , 'i > (& self , val : & 'r DeValue < 'i >) -> Option < & 'r Spanned < DeValue < 'i > > > ; }
    };
}

Index!()