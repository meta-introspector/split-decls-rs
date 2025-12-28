macro_rules! Once {
    () => {
        # [cfg (not (feature = "std"))] # [doc (hidden)] pub type Once = self :: spin :: Once < () > ;
    };
}

Once!()