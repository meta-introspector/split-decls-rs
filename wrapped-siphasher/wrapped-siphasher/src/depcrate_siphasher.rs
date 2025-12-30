// Generated macro for Hasher (struct)
macro_rules! Depcrate_sipHasher {
() => {
// Module: crate::sip
// Provides: {"Hasher"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] struct Hasher < S : Sip > { k0 : u64 , k1 : u64 , length : usize , state : State , tail : u64 , ntail : usize , _marker : PhantomData < S > , }
};
}
