// Generated macro for FilterBase (trait)
macro_rules! Depcrate_filterFilterBase {
() => {
// Module: crate::filter
// Provides: {"FilterBase"}
// Dependencies: {}
pub trait FilterBase { type Extract : Tuple ; type Error : IsReject ; type Future : Future < Output = Result < Self :: Extract , Self :: Error > > + Send ; fn filter (& self , internal : Internal) -> Self :: Future ; fn map_err < F , E > (self , _internal : Internal , fun : F) -> MapErr < Self , F > where Self : Sized , F : Fn (Self :: Error) -> E + Clone , E : :: std :: fmt :: Debug + Send , { MapErr { filter : self , callback : fun , } } }
};
}
