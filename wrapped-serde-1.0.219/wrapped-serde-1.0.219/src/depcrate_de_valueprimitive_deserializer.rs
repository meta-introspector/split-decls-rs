// Generated macro for primitive_deserializer (macro)
macro_rules! Depcrate_de_valueprimitive_deserializer {
() => {
// Module: crate::de::value
// Provides: {"primitive_deserializer"}
// Dependencies: {}
macro_rules ! primitive_deserializer { ($ ty : ty , $ doc : tt , $ name : ident , $ method : ident $ ($ cast : tt) *) => { # [doc = "A deserializer holding"] # [doc = $ doc] pub struct $ name < E > { value : $ ty , marker : PhantomData < E > } impl_copy_clone ! ($ name) ; impl <'de , E > IntoDeserializer <'de , E > for $ ty where E : de :: Error , { type Deserializer = $ name < E >; fn into_deserializer (self) -> $ name < E > { $ name :: new (self) } } impl < E > $ name < E > { # [allow (missing_docs)] pub fn new (value : $ ty) -> Self { $ name { value , marker : PhantomData , } } } impl <'de , E > de :: Deserializer <'de > for $ name < E > where E : de :: Error , { type Error = E ; forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor <'de >, { visitor .$ method (self . value $ ($ cast) *) } } impl <'de , E > IntoDeserializer <'de , E > for $ name < E > where E : de :: Error , { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } } impl < E > Debug for $ name < E > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct (stringify ! ($ name)) . field ("value" , & self . value) . finish () } } } }
};
}
