macro_rules! stmt {
    () => {
        # [cfg (feature = "full")] mod stmt ;
    };
}

stmt!();