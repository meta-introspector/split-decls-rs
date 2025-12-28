macro_rules! ImplGenerics {
    () => {
        # [doc = " Returned by `Generics::split_for_impl`."] # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (all (any (feature = "full" , feature = "derive") , feature = "printing"))))] pub struct ImplGenerics < 'a > (& 'a Generics) ;
    };
}

ImplGenerics!()