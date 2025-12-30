// Generated macro for body_methods (macro)
macro_rules! Depcrate_bodybody_methods {
() => {
// Module: crate::body
// Provides: {"body_methods"}
// Dependencies: {}
macro_rules ! body_methods { () => { # [inline] fn poll_frame (self : std :: pin :: Pin <& mut Self >, cx : & mut std :: task :: Context <'_ >,) -> std :: task :: Poll < Option < Result < http_body :: Frame < Self :: Data >, Self :: Error >>> { self . project () . inner . poll_frame (cx) } # [inline] fn is_end_stream (& self) -> bool { Body :: is_end_stream (& self . inner) } # [inline] fn size_hint (& self) -> http_body :: SizeHint { Body :: size_hint (& self . inner) } } ; }
};
}
