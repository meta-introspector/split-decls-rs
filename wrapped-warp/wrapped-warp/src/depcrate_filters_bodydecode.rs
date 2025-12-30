// Generated macro for Decode (trait)
macro_rules! Depcrate_filters_bodyDecode {
() => {
// Module: crate::filters::body
// Provides: {"Decode"}
// Dependencies: {}
trait Decode { const MIME : (mime :: Name < 'static > , mime :: Name < 'static >) ; const WITH_NO_CONTENT_TYPE : bool ; fn decode < B : Buf , T : DeserializeOwned > (buf : B) -> Result < T , BoxError > ; }
};
}
