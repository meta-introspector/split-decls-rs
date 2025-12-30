// Generated macro for nsthread_ivar_encoding (function)
macro_rules! Depcrate_array_ivarsnsthread_ivar_encoding {
() => {
// Module: crate::array_ivars
// Provides: {"nsthread_ivar_encoding"}
// Dependencies: {}
# [doc = " Defined in the header as:"] # [doc = " ```objc"] # [doc = " @interface NSThread : NSObject  {"] # [doc = " @private"] # [doc = "     id _private;"] # [doc = "     uint8_t _bytes[44];"] # [doc = " }"] # [doc = " ```"] # [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "GNUStep has a different set of ivars")] fn nsthread_ivar_encoding () { let cls = NSThread :: class () ; let expected = [("_private" , EncodingBox :: Object) , ("_bytes" , EncodingBox :: Array (44 , Box :: new (EncodingBox :: UChar)) ,) ,] ; let actual : Vec < _ > = (* cls . instance_variables ()) . iter () . map (| ivar | { (ivar . name () . to_str () . unwrap () , EncodingBox :: from_str (ivar . type_encoding () . to_str () . unwrap ()) . unwrap () ,) }) . collect () ; assert_eq ! (expected , * actual) ; }
};
}
