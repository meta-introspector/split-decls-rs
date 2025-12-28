macro_rules! Turbofish {
    () => {
        # [doc = " Returned by `TypeGenerics::as_turbofish`."] # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (all (any (feature = "full" , feature = "derive") , feature = "printing"))))] pub struct Turbofish < 'a > (& 'a Generics) ;
    };
}

Turbofish!()