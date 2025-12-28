macro_rules! deps {
    () => {
        RangeEnd!();
        IntRange!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [doc = " Note: this will render signed ranges incorrectly. To render properly, convert to a pattern"] # [doc = " first."] impl fmt :: Debug for IntRange { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_singleton () { let Finite (lo) = self . lo else { unreachable ! () } ; write ! (f , "{lo}") ? ; } else { if let Finite (lo) = self . lo { write ! (f , "{lo}") ? ; } write ! (f , "{}" , RangeEnd :: Excluded) ? ; if let Finite (hi) = self . hi { write ! (f , "{hi}") ? ; } } Ok (()) } }
    };
}

impl_10!();