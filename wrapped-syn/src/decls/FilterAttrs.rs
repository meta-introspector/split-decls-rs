macro_rules! FilterAttrs {
    () => {
        # [cfg (feature = "printing")] pub (crate) trait FilterAttrs < 'a > { type Ret : Iterator < Item = & 'a Attribute > ; fn outer (self) -> Self :: Ret ; # [cfg (feature = "full")] fn inner (self) -> Self :: Ret ; }
    };
}

FilterAttrs!();