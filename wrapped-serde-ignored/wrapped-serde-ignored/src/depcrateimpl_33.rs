// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , 'b , 'de , X , F > de :: MapAccess < 'de > for MapAccess < 'a , 'b , X , F > where X : de :: MapAccess < 'de > , F : FnMut (Path) , { type Error = X :: Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , X :: Error > where K : DeserializeSeed < 'de > , { self . delegate . next_key_seed (CaptureKey :: new (seed , & mut self . key)) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , X :: Error > where V : DeserializeSeed < 'de > , { let path = Path :: Map { parent : self . path , key : self . key () ? , } ; self . delegate . next_value_seed (TrackedSeed :: new (seed , self . callback , path)) } fn size_hint (& self) -> Option < usize > { self . delegate . size_hint () } }
};
}
