// Generated macro for impl_46 (impl)
macro_rules! Depcrate_layerimpl_46 {
() => {
// Module: crate::layer
// Provides: {"impl_46"}
// Dependencies: {}
impl < S , F > Layer < S > for ErrorLayer < S , F > where S : Subscriber + for < 'span > LookupSpan < 'span > , F : for < 'writer > FormatFields < 'writer > + 'static , { # [doc = " Notifies this layer that a new span was constructed with the given"] # [doc = " `Attributes` and `Id`."] fn on_new_span (& self , attrs : & span :: Attributes < '_ > , id : & span :: Id , ctx : layer :: Context < '_ , S >) { let span = ctx . span (id) . expect ("span must already exist!") ; if span . extensions () . get :: < FormattedFields < F > > () . is_some () { return ; } let mut fields = FormattedFields :: < F > :: new (String :: new ()) ; if self . format . format_fields (fields . as_writer () , attrs) . is_ok () { span . extensions_mut () . insert (fields) ; } } unsafe fn downcast_raw (& self , id : TypeId) -> Option < * const () > { match id { id if id == TypeId :: of :: < Self > () => Some (self as * const _ as * const ()) , id if id == TypeId :: of :: < WithContext > () => { Some (& self . get_context as * const _ as * const ()) } _ => None , } } }
};
}
