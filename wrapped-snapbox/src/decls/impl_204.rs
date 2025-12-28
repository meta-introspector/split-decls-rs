macro_rules! deps {
    () => {
        Redactions!();
        Data!();
        NormalizeToExpected!();
        NormalizeRedactions!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < 'a > NormalizeToExpected < 'a > { pub fn new () -> Self { Self { substitutions : None , unordered : false , } } # [doc = " Make unordered content comparable"] # [doc = ""] # [doc = " This is done by re-ordering `actual` according to `expected`."] pub fn unordered (mut self) -> Self { self . unordered = true ; self } # [doc = " Apply built-in redactions."] # [doc = ""] # [doc = " Built-in redactions:"] # [doc = " - `...` on a line of its own: match multiple complete lines"] # [doc = " - `[..]`: match multiple characters within a line"] # [doc = ""] # [doc = " Built-ins cannot automatically be applied to `actual` but are inferred from `expected`"] pub fn redact (mut self) -> Self { static REDACTIONS : Redactions = Redactions :: new () ; self . substitutions = Some (& REDACTIONS) ; self } # [doc = " Apply built-in and user [`Redactions`]"] # [doc = ""] # [doc = " Built-in redactions:"] # [doc = " - `...` on a line of its own: match multiple complete lines"] # [doc = " - `[..]`: match multiple characters within a line"] # [doc = ""] # [doc = " Built-ins cannot automatically be applied to `actual` but are inferred from `expected`"] pub fn redact_with (mut self , redactions : & 'a Redactions) -> Self { self . substitutions = Some (redactions) ; self } pub fn normalize (& self , actual : Data , expected : & Data) -> Data { let actual = if let Some (substitutions) = self . substitutions { NormalizeRedactions { redactions : substitutions , } . filter (actual) } else { actual } ; match (self . substitutions , self . unordered) { (None , false) => actual , (Some (substitutions) , false) => { normalize_data_to_redactions (actual , expected , substitutions) } (None , true) => normalize_data_to_unordered (actual , expected) , (Some (substitutions) , true) => { normalize_data_to_unordered_redactions (actual , expected , substitutions) } } } }
    };
}

impl_204!();