macro_rules! deps {
    () => {
        ScalarKind!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl ScalarKind { pub fn description (& self) -> & 'static str { match self { Self :: String => "string" , Self :: Boolean (_) => "boolean" , Self :: DateTime => "date-time" , Self :: Float => "float" , Self :: Integer (radix) => radix . description () , } } pub fn invalid_description (& self) -> & 'static str { match self { Self :: String => "invalid string" , Self :: Boolean (_) => "invalid boolean" , Self :: DateTime => "invalid date-time" , Self :: Float => "invalid float" , Self :: Integer (radix) => radix . invalid_description () , } } }
    };
}

impl_45!();