// Generated macro for impl_164 (impl)
macro_rules! Depcrate_zone_windowsimpl_164 {
() => {
// Module: crate::zone::windows
// Provides: {"impl_164"}
// Dependencies: {}
impl WindowsParser { # [doc = " Creates a new static [`WindowsParserBorrowed`]."] # [expect (clippy :: new_ret_no_self)] # [cfg (feature = "compiled_data")] pub fn new () -> WindowsParserBorrowed < 'static > { WindowsParserBorrowed :: new () } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < P > (provider : & P) -> Result < Self , DataError > where P : DataProvider < TimezoneIdentifiersWindowsV1 > + ? Sized , { let data = provider . load (Default :: default ()) ? . payload ; Ok (Self { data }) } # [doc = " Returns the borrowed version of the mapper that can be queried from"] # [doc = " the owned mapper."] # [doc = ""] # [doc = " Using the borrowed version allows one to avoid a small potential"] # [doc = " indirection cost when querying the mapper from the owned version."] pub fn as_borrowed (& self) -> WindowsParserBorrowed < '_ > { WindowsParserBorrowed { data : self . data . get () , } } }
};
}
