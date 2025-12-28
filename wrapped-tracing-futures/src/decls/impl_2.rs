macro_rules! deps {
    () => {
        Instrumented!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T , F > Executor < F > for Instrumented < T > where T : Executor < Instrumented < F > > , F : Future < Item = () , Error = () > , { fn execute (& self , future : F) -> Result < () , ExecuteError < F > > { let future = future . instrument (self . span . clone ()) ; self . inner . execute (future) . map_err (| e | { let kind = e . kind () ; let future = e . into_future () . into_inner () ; ExecuteError :: new (kind , future) }) } }
    };
}

impl_2!()