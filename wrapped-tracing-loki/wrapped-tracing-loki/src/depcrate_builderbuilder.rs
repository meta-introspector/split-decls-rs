// Generated macro for builder (function)
macro_rules! Depcrate_builderbuilder {
() => {
// Module: crate::builder
// Provides: {"builder"}
// Dependencies: {}
# [doc = " Create a [`Builder`] for constructing a [`Layer`] and its corresponding"] # [doc = " [`BackgroundTask`]."] # [doc = ""] # [doc = " See the crate's root documentation for an example."] pub fn builder () -> Builder { let mut http_headers = reqwest :: header :: HeaderMap :: new () ; http_headers . insert (reqwest :: header :: CONTENT_TYPE , reqwest :: header :: HeaderValue :: from_static ("application/x-snappy") ,) ; Builder { labels : FormattedLabels :: new () , extra_fields : HashMap :: new () , http_headers , } }
};
}
