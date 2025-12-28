macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! Svh {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , Debug , Encodable_NoContext , Decodable_NoContext , Hash)] pub struct Svh { hash : Fingerprint , }
    };
}

Svh!()