macro_rules! Variant {
    () => {
        # [doc = " The reserved variants of UUIDs."] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [Variant Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.1)"] # [derive (Clone , Copy , Debug , PartialEq)] # [non_exhaustive] # [repr (u8)] pub enum Variant { # [doc = " Reserved by the NCS for backward compatibility."] NCS = 0u8 , # [doc = " As described in the RFC 9562 Specification (default)."] # [doc = " (for backward compatibility it is not yet renamed)"] RFC4122 , # [doc = " Reserved by Microsoft for backward compatibility."] Microsoft , # [doc = " Reserved for future expansion."] Future , }
    };
}

Variant!();