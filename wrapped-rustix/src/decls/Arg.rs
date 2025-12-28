macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! Arg {
    () => {
        deps!();
        # [doc = " A trait for passing path arguments."] # [doc = ""] # [doc = " This is similar to [`AsRef`]`<`[`Path`]`>`, but is implemented for more"] # [doc = " kinds of strings and can convert into more kinds of strings."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(any(feature = \"fs\", feature = \"net\"))]"] # [doc = " use rustix::ffi::CStr;"] # [doc = " use rustix::io;"] # [doc = " # #[cfg(any(feature = \"fs\", feature = \"net\"))]"] # [doc = " use rustix::path::Arg;"] # [doc = ""] # [doc = " # #[cfg(any(feature = \"fs\", feature = \"net\"))]"] # [doc = " pub fn touch<P: Arg>(path: P) -> io::Result<()> {"] # [doc = "     let path = path.into_c_str()?;"] # [doc = "     _touch(&path)"] # [doc = " }"] # [doc = ""] # [doc = " # #[cfg(any(feature = \"fs\", feature = \"net\"))]"] # [doc = " fn _touch(path: &CStr) -> io::Result<()> {"] # [doc = "     // implementation goes here"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Users can then call `touch(\"foo\")`, `touch(cstr!(\"foo\"))`,"] # [doc = " `touch(Path::new(\"foo\"))`, or many other things."] # [doc = ""] # [doc = " [`AsRef`]: std::convert::AsRef"] pub trait Arg { # [doc = " Returns a view of this string as a string slice."] fn as_str (& self) -> io :: Result < & str > ; # [doc = " Returns a potentially-lossy rendering of this string as a"] # [doc = " `Cow<'_, str>`."] # [cfg (feature = "alloc")] fn to_string_lossy (& self) -> Cow < '_ , str > ; # [doc = " Returns a view of this string as a maybe-owned [`CStr`]."] # [cfg (feature = "alloc")] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > ; # [doc = " Consumes `self` and returns a view of this string as a maybe-owned"] # [doc = " [`CStr`]."] # [cfg (feature = "alloc")] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b ; # [doc = " Runs a closure with `self` passed in as a `&CStr`."] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > ; }
    };
}

Arg!();