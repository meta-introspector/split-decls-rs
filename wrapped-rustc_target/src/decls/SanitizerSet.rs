macro_rules! SanitizerSet {
    () => {
        # [derive (Default , Clone , Copy , PartialEq , Eq , Hash , Encodable , Decodable , HashStable_Generic)] pub struct SanitizerSet (u16) ;
    };
}

SanitizerSet!()