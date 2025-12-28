macro_rules! impl_478 {
    () => {
        impl LinkerFeatures { # [doc = " Parses a single `-C linker-features` well-known feature, not a set of flags."] pub fn from_str (s : & str) -> Option < LinkerFeatures > { Some (match s { "cc" => LinkerFeatures :: CC , "lld" => LinkerFeatures :: LLD , _ => return None , }) } # [doc = " Return the linker feature name, as would be passed on the CLI."] # [doc = ""] # [doc = " Returns `None` if the bitflags aren't a singular component (but a mix of multiple flags)."] pub fn as_str (self) -> Option < & 'static str > { Some (match self { LinkerFeatures :: CC => "cc" , LinkerFeatures :: LLD => "lld" , _ => return None , }) } # [doc = " Returns whether the `lld` linker feature is enabled."] pub fn is_lld_enabled (self) -> bool { self . contains (LinkerFeatures :: LLD) } # [doc = " Returns whether the `cc` linker feature is enabled."] pub fn is_cc_enabled (self) -> bool { self . contains (LinkerFeatures :: CC) } }
    };
}

impl_478!()