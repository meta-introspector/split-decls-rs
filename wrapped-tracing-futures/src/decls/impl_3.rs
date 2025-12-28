macro_rules! deps {
    () => {
        WithDispatch!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T , F > Executor < F > for WithDispatch < T > where T : Executor < WithDispatch < F > > , F : Future < Item = () , Error = () > , { fn execute (& self , future : F) -> Result < () , ExecuteError < F > > { let future = self . with_dispatch (future) ; self . inner . execute (future) . map_err (| e | { let kind = e . kind () ; let future = e . into_future () . inner ; ExecuteError :: new (kind , future) }) } }
    };
}

impl_3!()