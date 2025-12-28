macro_rules! ClosureKind {
    () => {
        # [doc = " Represents the various closure traits in the language. This"] # [doc = " will determine the type of the environment (`self`, in the"] # [doc = " desugaring) argument that the closure expects."] # [doc = ""] # [doc = " You can get the environment type of a closure using"] # [doc = " `tcx.closure_env_ty()`."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (Encodable , Decodable , HashStable_NoContext))] pub enum ClosureKind { Fn , FnMut , FnOnce , }
    };
}

ClosureKind!()