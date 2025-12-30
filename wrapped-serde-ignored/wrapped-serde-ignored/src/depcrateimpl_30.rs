// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
# [doc = " Forwarding impl to preserve context."] impl < 'a , 'b , 'de , X , F > de :: SeqAccess < 'de > for SeqAccess < 'a , 'b , X , F > where X : de :: SeqAccess < 'de > , F : FnMut (Path) , { type Error = X :: Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , X :: Error > where T : DeserializeSeed < 'de > , { let path = Path :: Seq { parent : self . path , index : self . index , } ; self . index += 1 ; self . delegate . next_element_seed (TrackedSeed :: new (seed , self . callback , path)) } fn size_hint (& self) -> Option < usize > { self . delegate . size_hint () } }
};
}
