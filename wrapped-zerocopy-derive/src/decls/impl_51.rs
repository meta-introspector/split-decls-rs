macro_rules! deps {
    () => {
        UnsupportedReprError!();
        FromAttrsError!();
        FromRawReprsError!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < Spanned < FromAttrsError > > for Error { fn from (err : Spanned < FromAttrsError >) -> Error { let Spanned { t : err , span } = err ; match err { FromAttrsError :: FromRawReprs (FromRawReprsError :: Single (_err @ UnsupportedReprError ,)) => Error :: new (span , "unsupported representation hint for the decorated type") , FromAttrsError :: FromRawReprs (FromRawReprsError :: Conflict) => { Error :: new (span , "this conflicts with another representation hint") } FromAttrsError :: Unrecognized => Error :: new (span , "unrecognized representation hint") , } } }
    };
}

impl_51!();