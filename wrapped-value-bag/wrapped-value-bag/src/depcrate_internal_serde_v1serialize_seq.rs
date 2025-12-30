// Generated macro for serialize_seq (function)
macro_rules! Depcrate_internal_serde_v1serialize_seq {
() => {
// Module: crate::internal::serde::v1
// Provides: {"serialize_seq"}
// Dependencies: {}
# [cfg (feature = "seq")] fn serialize_seq < S : value_bag_serde1 :: lib :: Serializer > (s : S , seq : & dyn crate :: internal :: seq :: Seq ,) -> Result < S :: Ok , S :: Error > { use crate :: std :: ops :: ControlFlow ; use value_bag_serde1 :: lib :: ser :: SerializeSeq ; struct SerializeVisitor < S : SerializeSeq > { serializer : S , err : Option < S :: Error > , } impl < 'v , S : SerializeSeq > crate :: internal :: seq :: Visitor < 'v > for SerializeVisitor < S > { fn element (& mut self , v : ValueBag) -> ControlFlow < () > { match self . serializer . serialize_element (& v) { Ok (()) => ControlFlow :: Continue (()) , Err (e) => { self . err = Some (e) ; ControlFlow :: Break (()) } } } } let mut s = SerializeVisitor { serializer : s . serialize_seq (None) ? , err : None , } ; seq . visit (& mut s) ; if let Some (e) = s . err { return Err (e) ; } s . serializer . end () }
};
}
