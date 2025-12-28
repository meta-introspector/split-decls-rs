macro_rules! deps {
    () => {
        BecauseExclusive!();
        CastType!();
        TryFromBytes!();
        PointerMetadata!();
        IntoBytes!();
        KnownLayout!();
        TryCastError!();
    };
}

macro_rules! try_mut_from_prefix_suffix {
    () => {
        deps!();
        # [inline (always)] fn try_mut_from_prefix_suffix < T : IntoBytes + TryFromBytes + KnownLayout + ? Sized > (candidate : & mut [u8] , cast_type : CastType , meta : Option < T :: PointerMetadata > ,) -> Result < (& mut T , & mut [u8]) , TryCastError < & mut [u8] , T > > { match Ptr :: from_mut (candidate) . try_cast_into :: < T , BecauseExclusive > (cast_type , meta) { Ok ((candidate , prefix_suffix)) => { match candidate . try_into_valid () { Ok (valid) => Ok ((valid . as_mut () , prefix_suffix . as_mut ())) , Err (e) => Err (e . map_src (| src | src . as_bytes :: < BecauseExclusive > () . as_mut ()) . into ()) , } } Err (e) => Err (e . map_src (Ptr :: as_mut) . into ()) , } }
    };
}

try_mut_from_prefix_suffix!();