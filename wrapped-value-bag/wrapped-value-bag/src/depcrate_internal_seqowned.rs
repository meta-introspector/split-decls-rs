// Generated macro for owned (module)
macro_rules! Depcrate_internal_seqowned {
() => {
// Module: crate::internal::seq
// Provides: {"owned"}
// Dependencies: {}
# [cfg (feature = "owned")] pub (crate) mod owned { use super :: * ; use crate :: { owned :: OwnedValueBag , std :: boxed :: Box } ; # [derive (Clone)] pub (crate) struct OwnedSeq (Box < [OwnedValueBag] >) ; impl Seq for OwnedSeq { fn visit (& self , visitor : & mut dyn Visitor < '_ >) { for item in self . 0 . iter () { if let ControlFlow :: Break (()) = visitor . element (item . by_ref ()) { return ; } } } } pub (crate) fn buffer (v : & dyn Seq) -> Result < OwnedSeq , Error > { struct BufferVisitor (Vec < OwnedValueBag >) ; impl < 'v > Visitor < 'v > for BufferVisitor { fn element (& mut self , v : ValueBag) -> ControlFlow < () > { self . 0 . push (v . to_owned ()) ; ControlFlow :: Continue (()) } } let mut buf = BufferVisitor (Vec :: new ()) ; v . visit (& mut buf) ; Ok (OwnedSeq (buf . 0 . into_boxed_slice ())) } }
};
}
