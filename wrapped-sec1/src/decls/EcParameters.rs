macro_rules! EcParameters {
    () => {
        # [doc = " Elliptic curve parameters as described in"] # [doc = " [RFC5480 Section 2.1.1](https://datatracker.ietf.org/doc/html/rfc5480#section-2.1.1):"] # [doc = ""] # [doc = " ```text"] # [doc = " ECParameters ::= CHOICE {"] # [doc = "   namedCurve         OBJECT IDENTIFIER"] # [doc = "   -- implicitCurve   NULL"] # [doc = "   -- specifiedCurve  SpecifiedECDomain"] # [doc = " }"] # [doc = "   -- implicitCurve and specifiedCurve MUST NOT be used in PKIX."] # [doc = "   -- Details for SpecifiedECDomain can be found in [X9.62]."] # [doc = "   -- Any future additions to this CHOICE should be coordinated"] # [doc = "   -- with ANSI X9."] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum EcParameters { # [doc = " Elliptic curve named by a particular OID."] # [doc = ""] # [doc = " > namedCurve identifies all the required values for a particular"] # [doc = " > set of elliptic curve domain parameters to be represented by an"] # [doc = " > object identifier."] NamedCurve (ObjectIdentifier) , }
    };
}

EcParameters!()