// Generated macro for seq (module)
macro_rules! Depcrate_internal_fmtseq {
() => {
// Module: crate::internal::fmt
// Provides: {"seq"}
// Dependencies: {}
# [cfg (feature = "seq")] mod seq { use super :: * ; use core :: ops :: ControlFlow ; pub (super) struct FmtSeq < 'a , 'b > (pub (super) fmt :: DebugList < 'b , 'a >) ; impl < 'a , 'b , 'c > crate :: internal :: seq :: Visitor < 'c > for FmtSeq < 'a , 'b > { fn element (& mut self , inner : ValueBag) -> ControlFlow < () > { self . 0 . entry (& inner) ; ControlFlow :: Continue (()) } } }
};
}
