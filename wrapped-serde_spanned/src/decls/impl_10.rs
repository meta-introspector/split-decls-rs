macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [allow (unused_qualifications)] impl core :: borrow :: Borrow < str > for Spanned < alloc :: string :: String > { fn borrow (& self) -> & str { self . get_ref () } }
    };
}

impl_10!()