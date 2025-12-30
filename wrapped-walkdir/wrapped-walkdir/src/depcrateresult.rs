// Generated macro for Result (type)
macro_rules! DepcrateResult {
() => {
// Module: crate
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A result type for walkdir operations."] # [doc = ""] # [doc = " Note that this result type embeds the error type in this crate. This"] # [doc = " is only useful if you care about the additional information provided by"] # [doc = " the error (such as the path associated with the error or whether a loop"] # [doc = " was dectected). If you want things to Just Work, then you can use"] # [doc = " [`io::Result`] instead since the error type in this package will"] # [doc = " automatically convert to an [`io::Result`] when using the [`try!`] macro."] # [doc = ""] # [doc = " [`io::Result`]: https://doc.rust-lang.org/stable/std/io/type.Result.html"] # [doc = " [`try!`]: https://doc.rust-lang.org/stable/std/macro.try.html"] pub type Result < T > = :: std :: result :: Result < T , Error > ;
};
}
