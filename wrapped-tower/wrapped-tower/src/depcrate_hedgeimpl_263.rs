// Generated macro for impl_263 (impl)
macro_rules! Depcrate_hedgeimpl_263 {
() => {
// Module: crate::hedge
// Provides: {"impl_263"}
// Dependencies: {}
impl < P , Request > crate :: filter :: AsyncPredicate < Request > for PolicyPredicate < P > where P : Policy < Request > , { type Future = Either < future :: Ready < Result < Request , crate :: BoxError > > , future :: Pending < Result < Request , crate :: BoxError > > , > ; type Request = Request ; fn check (& mut self , request : Request) -> Self :: Future { if self . 0 . can_retry (& request) { Either :: Left (future :: ready (Ok (request))) } else { Either :: Right (future :: pending ()) } } }
};
}
