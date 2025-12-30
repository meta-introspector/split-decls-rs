// Generated macro for Predicate (trait)
macro_rules! Depcrate_compression_predicatePredicate {
() => {
// Module: crate::compression::predicate
// Provides: {"Predicate"}
// Dependencies: {}
# [doc = " Predicate used to determine if a response should be compressed or not."] pub trait Predicate : Clone { # [doc = " Should this response be compressed or not?"] fn should_compress < B > (& self , response : & http :: Response < B >) -> bool where B : Body ; # [doc = " Combine two predicates into one."] # [doc = ""] # [doc = " The resulting predicate enables compression if both inner predicates do."] fn and < Other > (self , other : Other) -> And < Self , Other > where Self : Sized , Other : Predicate , { And { lhs : self , rhs : other , } } }
};
}
