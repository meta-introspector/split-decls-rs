macro_rules! type_will_always_be_passed_directly {
    () => {
        # [doc = " Returns true if values of a given type will never be passed indirectly, regardless of ABI."] fn type_will_always_be_passed_directly (ty : Ty < '_ >) -> bool { matches ! (ty . kind () , ty :: Bool | ty :: Char | ty :: Float (..) | ty :: Int (..) | ty :: RawPtr (..) | ty :: Ref (..) | ty :: Slice (..) | ty :: Uint (..)) }
    };
}

type_will_always_be_passed_directly!()