macro_rules! ExpnHash {
    () => {
        # [doc = " A unique hash value associated to an expansion."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug , Encodable , Decodable , HashStable_Generic)] pub struct ExpnHash (Fingerprint) ;
    };
}

ExpnHash!()