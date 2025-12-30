// Generated macro for MapSerializer (struct)
macro_rules! Depcrate_serMapSerializer {
() => {
// Module: crate::ser
// Provides: {"MapSerializer"}
// Dependencies: {}
# [doc = " Map serializer."] pub struct MapSerializer < 'input , 'output , Target : UrlEncodedTarget > { urlencoder : & 'output mut UrlEncodedSerializer < 'input , Target > , key : Option < Cow < 'static , str > > , }
};
}
