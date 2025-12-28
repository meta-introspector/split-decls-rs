macro_rules! ExpectedFound {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [derive (TypeFoldable_Generic , TypeVisitable_Generic)] pub struct ExpectedFound < T > { pub expected : T , pub found : T , }
    };
}

ExpectedFound!()