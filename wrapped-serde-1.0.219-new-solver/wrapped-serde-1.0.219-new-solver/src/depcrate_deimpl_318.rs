// Generated macro for impl_318 (impl)
macro_rules! Depcrate_deimpl_318 {
() => {
// Module: crate::de
// Provides: {"impl_318"}
// Dependencies: {}
impl < 'de , A > MapAccess < 'de > for & mut A where A : ? Sized + MapAccess < 'de > , { type Error = A :: Error ; # [inline] fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Self :: Error > where K : DeserializeSeed < 'de > , { (* * self) . next_key_seed (seed) } # [inline] fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Self :: Error > where V : DeserializeSeed < 'de > , { (* * self) . next_value_seed (seed) } # [inline] fn next_entry_seed < K , V > (& mut self , kseed : K , vseed : V ,) -> Result < Option < (K :: Value , V :: Value) > , Self :: Error > where K : DeserializeSeed < 'de > , V : DeserializeSeed < 'de > , { (* * self) . next_entry_seed (kseed , vseed) } # [inline] fn next_entry < K , V > (& mut self) -> Result < Option < (K , V) > , Self :: Error > where K : Deserialize < 'de > , V : Deserialize < 'de > , { (* * self) . next_entry () } # [inline] fn next_key < K > (& mut self) -> Result < Option < K > , Self :: Error > where K : Deserialize < 'de > , { (* * self) . next_key () } # [inline] fn next_value < V > (& mut self) -> Result < V , Self :: Error > where V : Deserialize < 'de > , { (* * self) . next_value () } # [inline] fn size_hint (& self) -> Option < usize > { (* * self) . size_hint () } }
};
}
