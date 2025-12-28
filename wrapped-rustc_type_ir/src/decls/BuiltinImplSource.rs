macro_rules! deps {
    () => {
        Upcast!();
    };
}

macro_rules! BuiltinImplSource {
    () => {
        deps!();
        # [derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext , Encodable_NoContext , Decodable_NoContext))] pub enum BuiltinImplSource { # [doc = " A built-in impl that is considered trivial, without any nested requirements. They"] # [doc = " are preferred over where-clauses, and we want to track them explicitly."] Trivial , # [doc = " Some built-in impl we don't need to differentiate. This should be used"] # [doc = " unless more specific information is necessary."] Misc , # [doc = " A built-in impl for trait objects. The index is only used in winnowing."] Object (usize) , # [doc = " A built-in implementation of `Upcast` for trait objects to other trait objects."] # [doc = ""] # [doc = " The index is only used for winnowing."] TraitUpcasting (usize) , }
    };
}

BuiltinImplSource!();