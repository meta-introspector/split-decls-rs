macro_rules! to_and_from_le {
    () => {
        # [doc = " Implement `from_le()` and `to_le()`, providing the field specification to both macros"] # [doc = " and methods."] macro_rules ! to_and_from_le { ($ ($ args : tt) ,+ $ (,) ?) => { # [inline (always)] fn from_le (mut self) -> Self { from_le ! [self , [$ ($ args) ,+]] ; self } # [inline (always)] fn to_le (mut self) -> Self { to_le ! [self , [$ ($ args) ,+]] ; self } } ; }
    };
}

to_and_from_le!();