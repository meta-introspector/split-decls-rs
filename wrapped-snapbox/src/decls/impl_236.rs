macro_rules! deps {
    () => {
        RedactedValue!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        # [cfg (feature = "regex")] impl From < & '_ regex :: Regex > for RedactedValue { fn from (inner : & '_ regex :: Regex) -> Self { inner . clone () . into () } }
    };
}

impl_236!();