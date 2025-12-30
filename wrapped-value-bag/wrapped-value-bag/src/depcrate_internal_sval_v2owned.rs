// Generated macro for owned (module)
macro_rules! Depcrate_internal_sval_v2owned {
() => {
// Module: crate::internal::sval::v2
// Provides: {"owned"}
// Dependencies: {}
# [cfg (feature = "owned")] pub (crate) mod owned { impl value_bag_sval2 :: lib :: Value for crate :: OwnedValueBag { fn stream < 'sval , S : value_bag_sval2 :: lib :: Stream < 'sval > + ? Sized > (& 'sval self , s : & mut S ,) -> value_bag_sval2 :: lib :: Result { value_bag_sval2 :: lib_ref :: ValueRef :: stream_ref (& self . by_ref () , s) } } pub (crate) type OwnedValue = value_bag_sval2 :: buffer :: Value < 'static > ; pub (crate) fn buffer (v : impl value_bag_sval2 :: lib :: Value ,) -> Result < OwnedValue , value_bag_sval2 :: buffer :: Error > { OwnedValue :: collect_owned (v) } }
};
}
