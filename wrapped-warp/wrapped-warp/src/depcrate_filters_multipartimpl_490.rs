// Generated macro for impl_490 (impl)
macro_rules! Depcrate_filters_multipartimpl_490 {
() => {
// Module: crate::filters::multipart
// Provides: {"impl_490"}
// Dependencies: {}
impl Stream for FormData { type Item = Result < Part , crate :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . inner . poll_next_field (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (Ok (Some (part))) => { if part . name () . is_some () || part . file_name () . is_some () { Poll :: Ready (Some (Ok (Part { part }))) } else { Poll :: Ready (Some (Err (crate :: Error :: new (MultipartFieldMissingName)))) } } Poll :: Ready (Ok (None)) => Poll :: Ready (None) , Poll :: Ready (Err (err)) => Poll :: Ready (Some (Err (crate :: Error :: new (err)))) , } } }
};
}
