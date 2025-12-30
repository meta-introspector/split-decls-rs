// Generated macro for impl_385 (impl)
macro_rules! Depcrate_queryimpl_385 {
() => {
// Module: crate::query
// Provides: {"impl_385"}
// Dependencies: {}
impl < I > QueryStackFrame < I > { # [inline] pub fn new (info : I , dep_kind : DepKind , hash : impl FnOnce () -> Hash64 , def_id : Option < DefId > , def_id_for_ty_in_cycle : Option < DefId > ,) -> Self { Self { info , def_id , dep_kind , hash : hash () , def_id_for_ty_in_cycle } } fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx ,) -> QueryStackFrame < QueryStackFrameExtra > { QueryStackFrame { info : qcx . lift_query_info (& self . info) , dep_kind : self . dep_kind , hash : self . hash , def_id : self . def_id , def_id_for_ty_in_cycle : self . def_id_for_ty_in_cycle , } } }
};
}
