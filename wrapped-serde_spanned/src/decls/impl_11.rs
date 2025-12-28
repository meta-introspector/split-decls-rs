macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl core :: borrow :: Borrow < str > for Spanned < alloc :: borrow :: Cow < '_ , str > > { fn borrow (& self) -> & str { self . get_ref () } }
    };
}

impl_11!();