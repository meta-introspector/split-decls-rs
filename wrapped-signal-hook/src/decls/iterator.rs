macro_rules! iterator {
    () => {
        # [cfg (all (not (windows) , feature = "iterator"))] # [cfg_attr (docsrs , doc (cfg (all (not (windows) , feature = "iterator"))))] pub mod iterator ;
    };
}

iterator!()