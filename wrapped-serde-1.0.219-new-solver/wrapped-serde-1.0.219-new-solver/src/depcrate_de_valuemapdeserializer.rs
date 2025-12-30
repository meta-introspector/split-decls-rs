// Generated macro for MapDeserializer (struct)
macro_rules! Depcrate_de_valueMapDeserializer {
() => {
// Module: crate::de::value
// Provides: {"MapDeserializer"}
// Dependencies: {}
# [doc = " A deserializer that iterates over a map."] pub struct MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , { iter : iter :: Fuse < I > , value : Option < Second < I :: Item > > , count : usize , lifetime : PhantomData < & 'de () > , error : PhantomData < E > , }
};
}
