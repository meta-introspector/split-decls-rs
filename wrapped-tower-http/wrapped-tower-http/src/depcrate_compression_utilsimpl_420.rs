// Generated macro for impl_420 (impl)
macro_rules! Depcrate_compression_utilsimpl_420 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_420"}
// Dependencies: {}
impl < B > Stream for BodyIntoStream < B > where B : Body , { type Item = Result < B :: Data , B :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { loop { let this = self . as_mut () . project () ; if * this . yielded_all_data { return Poll :: Ready (None) ; } match std :: task :: ready ! (this . body . poll_frame (cx)) { Some (Ok (frame)) => match frame . into_data () { Ok (data) => return Poll :: Ready (Some (Ok (data))) , Err (frame) => { * this . yielded_all_data = true ; * this . non_data_frame = Some (frame) ; } } , Some (Err (err)) => return Poll :: Ready (Some (Err (err))) , None => { * this . yielded_all_data = true ; } } } } }
};
}
