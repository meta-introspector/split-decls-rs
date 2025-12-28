macro_rules! deps {
    () => {
        Database!();
        RawDatabase!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'db , Db : Database + ? Sized > From < & 'db mut Db > for RawDatabase < 'db > { # [inline] fn from (db : & 'db mut Db) -> Self { RawDatabase { ptr : NonNull :: from (db) . cast () , _marker : std :: marker :: PhantomData , } } }
    };
}

impl_75!();