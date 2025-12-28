macro_rules! to_le {
    () => {
        # [doc = " Convert all the fields of a struct *into* little-endian representations."] macro_rules ! to_le { ($ obj : ident , $ field : ident , $ type : ty) => { $ obj .$ field = <$ type >:: to_le ($ obj .$ field) ; } ; ($ obj : ident , [($ field : ident , $ type : ty) $ (,) ?]) => { to_le ! [$ obj , $ field , $ type] ; } ; ($ obj : ident , [($ field : ident , $ type : ty) , $ ($ rest : tt) ,+ $ (,) ?]) => { to_le ! [$ obj , $ field , $ type] ; to_le ! ($ obj , [$ ($ rest) ,+]) ; } ; }
    };
}

to_le!();