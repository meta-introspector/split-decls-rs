macro_rules! cfg_block_on {
    () => {
        # [doc = " Enables `enter::block_on`."] macro_rules ! cfg_block_on { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "fs" , feature = "net" , feature = "io-std" , feature = "rt" ,))] $ item) * } }
    };
}

cfg_block_on!()