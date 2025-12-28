macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! should_allow_deprecated {
    () => {
        deps!();
        # [doc = " Determine if an `#[allow(deprecated)]` should be added to the derived impl."] # [doc = ""] # [doc = " This should happen if the derive input or an enum variant it contains has"] # [doc = " one of:"] # [doc = "   - `#[deprecated]`"] # [doc = "   - `#[allow(deprecated)]`"] fn should_allow_deprecated (input : & syn :: DeriveInput) -> bool { if contains_deprecated (& input . attrs) { return true ; } if let syn :: Data :: Enum (data_enum) = & input . data { for variant in & data_enum . variants { if contains_deprecated (& variant . attrs) { return true ; } } } false }
    };
}

should_allow_deprecated!();