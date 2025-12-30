// Generated macro for try_from_raw_reprs (function)
macro_rules! Depcrate_reprtry_from_raw_reprs {
() => {
// Module: crate::repr
// Provides: {"try_from_raw_reprs"}
// Dependencies: {}
# [doc = " Tries to extract a high-level repr from a list of `RawRepr`s."] fn try_from_raw_reprs < 'a , E , R : TryFrom < RawRepr , Error = FromRawReprError < E > > > (r : impl IntoIterator < Item = & 'a Spanned < RawRepr > > ,) -> Result < Option < Spanned < R > > , Spanned < FromRawReprsError < E > > > { r . into_iter () . try_fold (None , | found : Option < Spanned < R > > , raw | { let new = match Spanned :: < R > :: try_from (* raw) { Ok (r) => r , Err (FromRawReprError :: None) => return Ok (found) , Err (FromRawReprError :: Err (Spanned { t : err , span })) => { return Err (Spanned :: new (FromRawReprsError :: Single (err) , span)) } } ; if let Some (found) = found { let span = found . span . join (new . span) . unwrap_or (new . span) ; Err (Spanned :: new (FromRawReprsError :: Conflict , span)) } else { Ok (Some (new)) } }) }
};
}
