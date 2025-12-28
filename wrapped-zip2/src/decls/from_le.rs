macro_rules! from_le {
    () => {
        # [doc = " Convert all the fields of a struct *from* little-endian representations."] macro_rules ! from_le { ($ obj : ident , $ field : ident , $ type : ty) => { $ obj .$ field = <$ type >:: from_le ($ obj .$ field) ; } ; ($ obj : ident , [($ field : ident , $ type : ty) $ (,) ?]) => { from_le ! [$ obj , $ field , $ type] ; } ; ($ obj : ident , [($ field : ident , $ type : ty) , $ ($ rest : tt) ,+ $ (,) ?]) => { from_le ! [$ obj , $ field , $ type] ; from_le ! ($ obj , [$ ($ rest) ,+]) ; } ; }
    };
}

from_le!();