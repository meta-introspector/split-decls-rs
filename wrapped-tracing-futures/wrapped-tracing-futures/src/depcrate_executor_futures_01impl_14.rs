// Generated macro for impl_14 (impl)
macro_rules! Depcrate_executor_futures_01impl_14 {
() => {
// Module: crate::executor::futures_01
// Provides: {"impl_14"}
// Dependencies: {}
impl < T , F > Executor < F > for WithDispatch < T > where T : Executor < WithDispatch < F > > , F : Future < Item = () , Error = () > , { fn execute (& self , future : F) -> Result < () , ExecuteError < F > > { let future = self . with_dispatch (future) ; self . inner . execute (future) . map_err (| e | { let kind = e . kind () ; let future = e . into_future () . inner ; ExecuteError :: new (kind , future) }) } }
};
}
