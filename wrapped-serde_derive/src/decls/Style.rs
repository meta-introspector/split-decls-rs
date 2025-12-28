macro_rules! Style {
    () => {
        # [derive (Copy , Clone)] pub enum Style { # [doc = " Named fields."] Struct , # [doc = " Many unnamed fields."] Tuple , # [doc = " One unnamed field."] Newtype , # [doc = " No fields."] Unit , }
    };
}

Style!();