macro_rules! deps {
    () => {
        FromRawReprsError!();
        RawRepr!();
        FromRawReprError!();
    };
}

macro_rules! try_from_raw_reprs {
    () => {
        deps!();
        # [doc = " Tries to extract a high-level repr from a list of `RawRepr`s."] fn try_from_raw_reprs < 'a , E , R : TryFrom < RawRepr , Error = FromRawReprError < E > > > (r : impl IntoIterator < Item = & 'a Spanned < RawRepr > > ,) -> Result < Option < Spanned < R > > , Spanned < FromRawReprsError < E > > > { r . into_iter () . try_fold (None , | found : Option < Spanned < R > > , raw | { let new = match Spanned :: < R > :: try_from (* raw) { Ok (r) => r , Err (FromRawReprError :: None) => return Ok (found) , Err (FromRawReprError :: Err (Spanned { t : err , span })) => { return Err (Spanned :: new (FromRawReprsError :: Single (err) , span)) } } ; if let Some (found) = found { let span = found . span . join (new . span) . unwrap_or (new . span) ; Err (Spanned :: new (FromRawReprsError :: Conflict , span)) } else { Ok (Some (new)) } }) }
    };
}

try_from_raw_reprs!();