macro_rules! deps {
    () => {
        PointerMetadata!();
        TryCastError!();
        KnownLayout!();
        CastType!();
        TryFromBytes!();
        BecauseImmutable!();
        Immutable!();
    };
}

macro_rules! try_ref_from_prefix_suffix {
    () => {
        deps!();
        # [inline (always)] fn try_ref_from_prefix_suffix < T : TryFromBytes + KnownLayout + Immutable + ? Sized > (source : & [u8] , cast_type : CastType , meta : Option < T :: PointerMetadata > ,) -> Result < (& T , & [u8]) , TryCastError < & [u8] , T > > { match Ptr :: from_ref (source) . try_cast_into :: < T , BecauseImmutable > (cast_type , meta) { Ok ((source , prefix_suffix)) => { match source . try_into_valid () { Ok (valid) => Ok ((valid . as_ref () , prefix_suffix . as_ref ())) , Err (e) => Err (e . map_src (| src | src . as_bytes :: < BecauseImmutable > () . as_ref ()) . into ()) , } } Err (e) => Err (e . map_src (Ptr :: as_ref) . into ()) , } }
    };
}

try_ref_from_prefix_suffix!()