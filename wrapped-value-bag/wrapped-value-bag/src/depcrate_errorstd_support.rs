// Generated macro for std_support (module)
macro_rules! Depcrate_errorstd_support {
() => {
// Module: crate::error
// Provides: {"std_support"}
// Dependencies: {}
# [cfg (feature = "std")] mod std_support { use super :: * ; use crate :: std :: { boxed :: Box , error , io } ; pub (crate) type BoxedError = Box < dyn error :: Error + Send + Sync > ; impl Error { # [doc = " Create an error from a standard error type."] pub fn boxed < E > (err : E) -> Self where E : Into < BoxedError > , { Error { inner : Inner :: Boxed (err . into ()) , } } } impl error :: Error for Error { } impl From < io :: Error > for Error { fn from (err : io :: Error) -> Self { Error :: boxed (err) } } }
};
}
