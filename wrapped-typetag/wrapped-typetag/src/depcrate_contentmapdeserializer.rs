// Generated macro for MapDeserializer (struct)
macro_rules! Depcrate_contentMapDeserializer {
() => {
// Module: crate::content
// Provides: {"MapDeserializer"}
// Dependencies: {}
struct MapDeserializer < 'de , E > where E : de :: Error , { iter : < Vec < (Content < 'de > , Content < 'de >) > as IntoIterator > :: IntoIter , value : Option < Content < 'de > > , err : PhantomData < E > , }
};
}
