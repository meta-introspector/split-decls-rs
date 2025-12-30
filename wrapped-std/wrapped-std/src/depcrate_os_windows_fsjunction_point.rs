// Generated macro for junction_point (function)
macro_rules! Depcrate_os_windows_fsjunction_point {
() => {
// Module: crate::os::windows::fs
// Provides: {"junction_point"}
// Dependencies: {}
# [doc = " Creates a junction point."] # [doc = ""] # [doc = " The `link` path will be a directory junction pointing to the original path."] # [doc = " If `link` is a relative path then it will be made absolute prior to creating the junction point."] # [doc = " The `original` path must be a directory or a link to a directory, otherwise the junction point will be broken."] # [doc = ""] # [doc = " If either path is not a local file path then this will fail."] # [unstable (feature = "junction_point" , issue = "121709")] pub fn junction_point < P : AsRef < Path > , Q : AsRef < Path > > (original : P , link : Q) -> io :: Result < () > { sys :: fs :: junction_point (original . as_ref () , link . as_ref ()) }
};
}
