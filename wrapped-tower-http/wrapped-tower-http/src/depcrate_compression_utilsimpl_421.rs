// Generated macro for impl_421 (impl)
macro_rules! Depcrate_compression_utilsimpl_421 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_421"}
// Dependencies: {}
impl < B > Body for BodyIntoStream < B > where B : Body , { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { if let Some (frame) = std :: task :: ready ! (self . as_mut () . poll_next (cx)) { return Poll :: Ready (Some (frame . map (Frame :: data))) ; } let this = self . project () ; if let Some (frame) = this . non_data_frame . take () { return Poll :: Ready (Some (Ok (frame))) ; } this . body . poll_frame (cx) } # [inline] fn size_hint (& self) -> http_body :: SizeHint { self . body . size_hint () } }
};
}
