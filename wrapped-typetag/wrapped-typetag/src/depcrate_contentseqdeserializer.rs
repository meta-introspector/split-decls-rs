// Generated macro for SeqDeserializer (struct)
macro_rules! Depcrate_contentSeqDeserializer {
() => {
// Module: crate::content
// Provides: {"SeqDeserializer"}
// Dependencies: {}
struct SeqDeserializer < 'de , E > where E : de :: Error , { iter : < Vec < Content < 'de > > as IntoIterator > :: IntoIter , err : PhantomData < E > , }
};
}
