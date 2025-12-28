macro_rules! UnordBag {
    () => {
        # [doc = " This is a collection type that tries very hard to not expose"] # [doc = " any internal iteration. This is a useful property when trying to"] # [doc = " uphold the determinism invariants imposed by the query system."] # [doc = ""] # [doc = " This collection type is a good choice for collections the"] # [doc = " keys of which don't have a semantic ordering and don't implement"] # [doc = " `Hash` or `Eq`."] # [doc = ""] # [doc = " See [MCP 533](https://github.com/rust-lang/compiler-team/issues/533)"] # [doc = " for more information."] # [derive (Default , Debug , Eq , PartialEq , Clone , Encodable_NoContext , Decodable_NoContext)] pub struct UnordBag < V > { inner : Vec < V > , }
    };
}

UnordBag!();