macro_rules! deps {
    () => {
        IrPrint!();
        Interner!();
        OutlivesPredicate!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < I : Interner , T > fmt :: Display for OutlivesPredicate < I , T > where I : IrPrint < OutlivesPredicate < I , T > > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < I as IrPrint < OutlivesPredicate < I , T > > > :: print (self , fmt) } }
    };
}

impl_91!()