// Generated macro for SeqDeserializer (struct)
macro_rules! Depcrate_de_valueSeqDeserializer {
() => {
// Module: crate::de::value
// Provides: {"SeqDeserializer"}
// Dependencies: {}
# [doc = " A deserializer that iterates over a sequence."] # [derive (Clone)] pub struct SeqDeserializer < I , E > { iter : iter :: Fuse < I > , count : usize , marker : PhantomData < E > , }
};
}
