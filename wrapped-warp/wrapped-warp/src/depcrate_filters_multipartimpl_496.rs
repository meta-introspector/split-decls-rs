// Generated macro for impl_496 (impl)
macro_rules! Depcrate_filters_multipartimpl_496 {
() => {
// Module: crate::filters::multipart
// Provides: {"impl_496"}
// Dependencies: {}
impl Stream for BodyIoError { type Item = io :: Result < Bytes > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match Pin :: new (& mut self . 0) . poll_next (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (Some (Ok (bytes))) => Poll :: Ready (Some (Ok (bytes))) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Ready (Some (Err (err))) => { Poll :: Ready (Some (Err (io :: Error :: new (io :: ErrorKind :: Other , err)))) } } } }
};
}
