// Generated macro for layer (module)
macro_rules! Depcrate_request_spanlayer {
() => {
// Module: crate::request_span
// Provides: {"layer"}
// Dependencies: {}
# [cfg (feature = "tower-layer")] # [cfg_attr (docsrs , doc (cfg (feature = "tower-layer")))] mod layer { use super :: * ; # [derive (Debug)] pub struct Layer < R , G = fn (& R) -> tracing :: Span > where G : GetSpan < R > + Clone , { get_span : G , _p : PhantomData < fn (R) > , } pub fn layer < R , G > (get_span : G) -> Layer < R , G > where G : GetSpan < R > + Clone , { Layer { get_span , _p : PhantomData , } } impl < S , R , G > tower_layer :: Layer < S > for Layer < R , G > where S : tower_service :: Service < R > , G : GetSpan < R > + Clone , { type Service = Service < S , R , G > ; fn layer (& self , service : S) -> Self :: Service { Service :: new (service , self . get_span . clone ()) } } impl < R , G > Clone for Layer < R , G > where G : GetSpan < R > + Clone , { fn clone (& self) -> Self { Self { get_span : self . get_span . clone () , _p : PhantomData , } } } }
};
}
