macro_rules! deps {
    () => {
        GetSpan!();
        Service!();
    };
}

macro_rules! layer {
    () => {
        deps!();
        # [cfg (feature = "tower-layer")] # [cfg_attr (docsrs , doc (cfg (feature = "tower-layer")))] mod layer { use super :: * ; # [derive (Debug)] pub struct Layer < S , R , G = fn (& S) -> tracing :: Span > where G : GetSpan < S > , S : tower_service :: Service < R > , { get_span : G , _p : PhantomData < fn (S , R) > , } pub fn layer < S , R , G > (get_span : G) -> Layer < S , R , G > where G : GetSpan < S > , S : tower_service :: Service < R > , { Layer { get_span , _p : PhantomData , } } impl < S , R , G > tower_layer :: Layer < S > for Layer < S , R , G > where G : GetSpan < S > , S : tower_service :: Service < R > , { type Service = Service < S > ; fn layer (& self , inner : S) -> Self :: Service { let span = self . get_span . span_for (& inner) ; Service { inner , span } } } impl < S , R , G > Clone for Layer < S , R , G > where G : GetSpan < S > + Clone , S : tower_service :: Service < R > , { fn clone (& self) -> Self { Self { get_span : self . get_span . clone () , _p : PhantomData , } } } }
    };
}

layer!()