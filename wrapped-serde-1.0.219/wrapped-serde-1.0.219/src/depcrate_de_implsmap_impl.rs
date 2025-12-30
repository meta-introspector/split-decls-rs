// Generated macro for map_impl (macro)
macro_rules! Depcrate_de_implsmap_impl {
() => {
// Module: crate::de::impls
// Provides: {"map_impl"}
// Dependencies: {}
macro_rules ! map_impl { ($ (# [$ attr : meta]) * $ ty : ident < K $ (: $ kbound1 : ident $ (+ $ kbound2 : ident) *) *, V $ (, $ typaram : ident : $ bound1 : ident $ (+ $ bound2 : ident) *) *>, $ access : ident , $ with_capacity : expr ,) => { $ (# [$ attr]) * impl <'de , K , V $ (, $ typaram) *> Deserialize <'de > for $ ty < K , V $ (, $ typaram) *> where K : Deserialize <'de > $ (+ $ kbound1 $ (+ $ kbound2) *) *, V : Deserialize <'de >, $ ($ typaram : $ bound1 $ (+ $ bound2) *) ,* { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer <'de >, { struct MapVisitor < K , V $ (, $ typaram) *> { marker : PhantomData <$ ty < K , V $ (, $ typaram) *>>, } impl <'de , K , V $ (, $ typaram) *> Visitor <'de > for MapVisitor < K , V $ (, $ typaram) *> where K : Deserialize <'de > $ (+ $ kbound1 $ (+ $ kbound2) *) *, V : Deserialize <'de >, $ ($ typaram : $ bound1 $ (+ $ bound2) *) ,* { type Value = $ ty < K , V $ (, $ typaram) *>; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a map") } # [inline] fn visit_map < A > (self , mut $ access : A) -> Result < Self :: Value , A :: Error > where A : MapAccess <'de >, { let mut values = $ with_capacity ; while let Some ((key , value)) = tri ! ($ access . next_entry ()) { values . insert (key , value) ; } Ok (values) } } let visitor = MapVisitor { marker : PhantomData } ; deserializer . deserialize_map (visitor) } } } }
};
}
