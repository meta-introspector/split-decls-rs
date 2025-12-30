// Generated macro for owned (module)
macro_rules! Depcrate_internal_serde_v1owned {
() => {
// Module: crate::internal::serde::v1
// Provides: {"owned"}
// Dependencies: {}
# [cfg (feature = "owned")] pub (crate) mod owned { use crate :: std :: boxed :: Box ; impl value_bag_serde1 :: lib :: Serialize for crate :: OwnedValueBag { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : value_bag_serde1 :: lib :: Serializer , { value_bag_serde1 :: lib :: Serialize :: serialize (& self . by_ref () , s) } } pub (crate) type OwnedSerialize = Box < value_bag_serde1 :: buf :: Owned > ; pub (crate) fn buffer (v : impl value_bag_serde1 :: lib :: Serialize ,) -> Result < OwnedSerialize , value_bag_serde1 :: buf :: Error > { value_bag_serde1 :: buf :: Owned :: buffer (v) . map (Box :: new) } }
};
}
