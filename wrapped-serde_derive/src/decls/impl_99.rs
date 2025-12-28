macro_rules! deps {
    () => {
        Ctxt!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Ctxt { # [doc = " Create a new context object."] # [doc = ""] # [doc = " This object contains no errors, but will still trigger a panic if it is not `check`ed."] pub fn new () -> Self { Ctxt { errors : RefCell :: new (Some (Vec :: new ())) , } } # [doc = " Add an error to the context object with a tokenenizable object."] # [doc = ""] # [doc = " The object is used for spanning in error messages."] pub fn error_spanned_by < A : ToTokens , T : Display > (& self , obj : A , msg : T) { self . errors . borrow_mut () . as_mut () . unwrap () . push (syn :: Error :: new_spanned (obj . into_token_stream () , msg)) ; } # [doc = " Add one of Syn's parse errors."] pub fn syn_error (& self , err : syn :: Error) { self . errors . borrow_mut () . as_mut () . unwrap () . push (err) ; } # [doc = " Consume this object, producing a formatted error string if there are errors."] pub fn check (self) -> syn :: Result < () > { let mut errors = self . errors . borrow_mut () . take () . unwrap () . into_iter () ; let Some (mut combined) = errors . next () else { return Ok (()) ; } ; for rest in errors { combined . combine (rest) ; } Err (combined) } }
    };
}

impl_99!()