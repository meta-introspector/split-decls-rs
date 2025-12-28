macro_rules! VERSION {
    () => {
        # [doc = " `ECPrivateKey` version."] # [doc = ""] # [doc = " From [RFC5913 Section 3]:"] # [doc = " > version specifies the syntax version number of the elliptic curve"] # [doc = " > private key structure.  For this version of the document, it SHALL"] # [doc = " > be set to ecPrivkeyVer1, which is of type INTEGER and whose value"] # [doc = " > is one (1)."] # [doc = ""] # [doc = " [RFC5915 Section 3]: https://datatracker.ietf.org/doc/html/rfc5915#section-3"] const VERSION : u8 = 1 ;
    };
}

VERSION!();