macro_rules! OnBrokenPipe {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Hash , Encodable , Decodable , HashStable_Generic)] pub enum OnBrokenPipe { Default , Kill , Error , Inherit , }
    };
}

OnBrokenPipe!();