// Generated macro for impl_47 (impl)
macro_rules! Depcrate_algorithms_utilsimpl_47 {
() => {
// Module: crate::algorithms::utils
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , Idx : Index < usize > + 'a > Debug for UniqueItem < 'a , Idx > where Idx :: Output : Debug , { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_struct ("UniqueItem") . field ("value" , & self . value ()) . field ("original_index" , & self . original_index ()) . finish () } }
};
}
