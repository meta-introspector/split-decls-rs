// Generated macro for Sanitize (trait)
macro_rules! DepcrateSanitize {
() => {
// Module: crate
// Provides: {"Sanitize"}
// Dependencies: {}
# [doc = " A trait for sanitizing values and members of over-the-wire messages."] # [doc = ""] # [doc = " Implementation should recursively descend through the data structure and"] # [doc = " sanitize all struct members and enum clauses. Sanitize excludes signature-"] # [doc = " verification checks, those are handled by another pass. Sanitize checks"] # [doc = " should include but are not limited to:"] # [doc = ""] # [doc = " - All index values are in range."] # [doc = " - All values are within their static max/min bounds."] pub trait Sanitize { fn sanitize (& self) -> Result < () , SanitizeError > { Ok (()) } }
};
}
