// Generated macro for impl_408 (impl)
macro_rules! Depcrate_value_analysisimpl_408 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_408"}
// Dependencies: {}
impl < 'tcx > PlaceCollector < '_ , 'tcx > { # [tracing :: instrument (level = "trace" , skip (self))] fn register_place (& mut self , place : Place < 'tcx >) -> Option < PlaceIndex > { let mut place_index = self . map . locals [place . local] ? ; let mut ty = PlaceTy :: from_ty (self . body . local_decls [place . local] . ty) ; tracing :: trace ! (? place_index , ? ty) ; if let ty :: Ref (_ , ref_ty , _) | ty :: RawPtr (ref_ty , _) = ty . ty . kind () && let ty :: Slice (..) = ref_ty . kind () { self . map . register_place (self . tcx . types . usize , place_index , TrackElem :: DerefLen) ; } else if ty . ty . is_enum () { let discriminant_ty = ty . ty . discriminant_ty (self . tcx) ; self . map . register_place (discriminant_ty , place_index , TrackElem :: Discriminant) ; } for proj in place . projection { let track_elem = proj . try_into () . ok () ? ; ty = ty . projection_ty (self . tcx , proj) ; place_index = self . map . register_place (ty . ty , place_index , track_elem) ; tracing :: trace ! (? proj , ? place_index , ? ty) ; if let ty :: Ref (_ , ref_ty , _) | ty :: RawPtr (ref_ty , _) = ty . ty . kind () && let ty :: Slice (..) = ref_ty . kind () { self . map . register_place (self . tcx . types . usize , place_index , TrackElem :: DerefLen) ; } else if ty . ty . is_enum () { let discriminant_ty = ty . ty . discriminant_ty (self . tcx) ; self . map . register_place (discriminant_ty , place_index , TrackElem :: Discriminant) ; } } Some (place_index) } }
};
}
