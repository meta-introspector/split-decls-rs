// Generated macro for impl_34 (impl)
macro_rules! Depcrate_task_join_mapimpl_34 {
() => {
// Module: crate::task::join_map
// Provides: {"impl_34"}
// Dependencies: {}
impl < K : fmt :: Debug , V , S > fmt :: Debug for JoinMap < K , V , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct KeySet < 'a , K : fmt :: Debug > (& 'a HashTable < (K , AbortHandle) >) ; impl < K : fmt :: Debug > fmt :: Debug for KeySet < '_ , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . 0 . iter () . map (| (key , abort) | (key , abort . id ()))) . finish () } } f . debug_struct ("JoinMap") . field ("tasks" , & KeySet (& self . tasks_by_key)) . finish () } }
};
}
