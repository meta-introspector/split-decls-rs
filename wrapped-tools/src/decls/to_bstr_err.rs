macro_rules! to_bstr_err {
    () => {
        # [doc = " Transform a verbose parser errors from raw bytes into a `BStr` to make printing/debugging human-readable."] pub fn to_bstr_err (err : winnow :: error :: ErrMode < winnow :: error :: TreeError < & [u8] , winnow :: error :: StrContext > > ,) -> winnow :: error :: TreeError < & winnow :: stream :: BStr , winnow :: error :: StrContext > { let err = err . into_inner () . expect ("not a streaming parser") ; err . map_input (winnow :: stream :: BStr :: new) }
    };
}

to_bstr_err!()