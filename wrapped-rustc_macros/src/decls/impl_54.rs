macro_rules! deps {
    () => {
        Applicability!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl FromStr for Applicability { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "machine-applicable" => Ok (Applicability :: MachineApplicable) , "maybe-incorrect" => Ok (Applicability :: MaybeIncorrect) , "has-placeholders" => Ok (Applicability :: HasPlaceholders) , "unspecified" => Ok (Applicability :: Unspecified) , _ => Err (()) , } } }
    };
}

impl_54!();