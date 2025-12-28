macro_rules! deps {
    () => {
        IrPrint!();
        Binder!();
        Interner!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < I : Interner , T > fmt :: Display for Binder < I , T > where I : IrPrint < Binder < I , T > > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < I as IrPrint < Binder < I , T > > > :: print (self , fmt) } }
    };
}

impl_87!();