macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl alloc :: fmt :: Debug for Error { fn fmt (& self , f : & mut alloc :: fmt :: Formatter < '_ >) -> Result < () , alloc :: fmt :: Error > { let mut s = f . debug_struct ("Error") ; s . field ("kind" , & self . kind) ; if let Some (err) = self . err . as_ref () { s . field ("err" , & alloc :: format ! ("{err}")) ; } s . finish () } }
    };
}

impl_422!();