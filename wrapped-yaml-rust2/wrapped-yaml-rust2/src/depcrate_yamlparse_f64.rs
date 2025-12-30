// Generated macro for parse_f64 (function)
macro_rules! Depcrate_yamlparse_f64 {
() => {
// Module: crate::yaml
// Provides: {"parse_f64"}
// Dependencies: {}
fn parse_f64 (v : & str) -> Option < f64 > { match v { ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Some (f64 :: INFINITY) , "-.inf" | "-.Inf" | "-.INF" => Some (f64 :: NEG_INFINITY) , ".nan" | ".NaN" | ".NAN" => Some (f64 :: NAN) , _ if v . as_bytes () . iter () . any (u8 :: is_ascii_digit) => v . parse :: < f64 > () . ok () , _ => None , } }
};
}
