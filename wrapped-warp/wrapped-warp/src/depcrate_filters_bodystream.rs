// Generated macro for stream (function)
macro_rules! Depcrate_filters_bodystream {
() => {
// Module: crate::filters::body
// Provides: {"stream"}
// Dependencies: {}
# [doc = " Create a `Filter` that extracts the request body as a `futures::Stream`."] # [doc = ""] # [doc = " If other filters have already extracted the body, this filter will reject"] # [doc = " with a `500 Internal Server Error`."] # [doc = ""] # [doc = " For example usage, please take a look at [examples/stream.rs](https://github.com/seanmonstar/warp/blob/master/examples/stream.rs)."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This does not have a default size limit, it would be wise to use one to"] # [doc = " prevent a overly large request from using too much memory."] pub fn stream () -> impl Filter < Extract = (impl Stream < Item = Result < impl Buf , crate :: Error > > ,) , Error = Rejection > + Copy { body () . map (| body | BodyDataStream :: new (body)) }
};
}
