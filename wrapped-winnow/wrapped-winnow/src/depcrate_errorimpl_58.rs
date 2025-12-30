// Generated macro for impl_58 (impl)
macro_rules! Depcrate_errorimpl_58 {
() => {
// Module: crate::error
// Provides: {"impl_58"}
// Dependencies: {}
impl < E > ErrMode < E > { # [doc = " Tests if the result is Incomplete"] # [inline] pub fn is_incomplete (& self) -> bool { matches ! (self , ErrMode :: Incomplete (_)) } # [doc = " Prevent backtracking, bubbling the error up to the top"] pub fn cut (self) -> Self { match self { ErrMode :: Backtrack (e) => ErrMode :: Cut (e) , rest => rest , } } # [doc = " Enable backtracking support"] pub fn backtrack (self) -> Self { match self { ErrMode :: Cut (e) => ErrMode :: Backtrack (e) , rest => rest , } } # [doc = " Applies the given function to the inner error"] pub fn map < E2 , F > (self , f : F) -> ErrMode < E2 > where F : FnOnce (E) -> E2 , { match self { ErrMode :: Incomplete (n) => ErrMode :: Incomplete (n) , ErrMode :: Cut (t) => ErrMode :: Cut (f (t)) , ErrMode :: Backtrack (t) => ErrMode :: Backtrack (f (t)) , } } # [doc = " Automatically converts between errors if the underlying type supports it"] pub fn convert < F > (self) -> ErrMode < F > where E : ErrorConvert < F > , { ErrorConvert :: convert (self) } # [doc = " Unwrap the mode, returning the underlying error"] # [doc = ""] # [doc = " Returns `Err(self)` for [`ErrMode::Incomplete`]"] # [inline (always)] pub fn into_inner (self) -> Result < E , Self > { match self { ErrMode :: Backtrack (e) | ErrMode :: Cut (e) => Ok (e) , err @ ErrMode :: Incomplete (_) => Err (err) , } } }
};
}
