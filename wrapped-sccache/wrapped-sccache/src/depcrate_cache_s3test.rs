// Generated macro for test (module)
macro_rules! Depcrate_cache_s3test {
() => {
// Module: crate::cache::s3
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_endpoint_resolver () -> Result < () > { let cases = vec ! [("no scheme without use_ssl" , "s3-us-east-1.amazonaws.com" , None , "http://s3-us-east-1.amazonaws.com/" ,) , ("http without use_ssl" , "http://s3-us-east-1.amazonaws.com" , None , "http://s3-us-east-1.amazonaws.com/" ,) , ("https without use_ssl" , "https://s3-us-east-1.amazonaws.com" , None , "https://s3-us-east-1.amazonaws.com/" ,) , ("no scheme with use_ssl" , "s3-us-east-1.amazonaws.com" , Some (true) , "https://s3-us-east-1.amazonaws.com/" ,) , ("http with use_ssl" , "http://s3-us-east-1.amazonaws.com" , Some (true) , "https://s3-us-east-1.amazonaws.com/" ,) , ("https with use_ssl" , "https://s3-us-east-1.amazonaws.com" , Some (true) , "https://s3-us-east-1.amazonaws.com/" ,) , ("no scheme with not use_ssl" , "s3-us-east-1.amazonaws.com" , Some (false) , "http://s3-us-east-1.amazonaws.com/" ,) , ("http with not use_ssl" , "http://s3-us-east-1.amazonaws.com" , Some (false) , "http://s3-us-east-1.amazonaws.com/" ,) , ("https with not use_ssl" , "https://s3-us-east-1.amazonaws.com" , Some (false) , "http://s3-us-east-1.amazonaws.com/" ,) ,] ; for (name , endpoint , use_ssl , expected) in cases { let actual = endpoint_resolver (endpoint , use_ssl) ? ; assert_eq ! (actual , expected , "{}" , name) ; } Ok (()) } }
};
}
