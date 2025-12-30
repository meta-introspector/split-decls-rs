// Generated macro for impl_884 (impl)
macro_rules! Depcrate_util_oneshotimpl_884 {
() => {
// Module: crate::util::oneshot
// Provides: {"impl_884"}
// Dependencies: {}
impl < S , Req > fmt :: Debug for State < S , Req > where S : Service < Req > + fmt :: Debug , Req : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { State :: NotReady { svc , req : Some (req) , } => f . debug_tuple ("State::NotReady") . field (svc) . field (req) . finish () , State :: NotReady { req : None , .. } => unreachable ! () , State :: Called { .. } => f . debug_tuple ("State::Called") . field (& "S::Future") . finish () , State :: Done => f . debug_tuple ("State::Done") . finish () , } } }
};
}
