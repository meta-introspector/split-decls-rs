macro_rules! arbitrary_support {
    () => {
        # [cfg (feature = "arbitrary")] pub (crate) mod arbitrary_support ;
    };
}

arbitrary_support!()