// Generated macro for register_float (function)
macro_rules! Depcrateregister_float {
() => {
// Module: crate
// Provides: {"register_float"}
// Dependencies: {}
# [doc = " Register all generators for a single float."] fn register_float < F : Float > (tests : & mut Vec < TestInfo > , cfg : & Config) where RangeInclusive < F :: Int > : Iterator < Item = F :: Int > , < F :: Int as TryFrom < u128 > > :: Error : std :: fmt :: Debug , StandardUniform : Distribution < < F as traits :: Float > :: Int > , { if F :: BITS <= MAX_BITS_FOR_EXHAUUSTIVE { TestInfo :: register :: < F , gen_ :: exhaustive :: Exhaustive < F > > (tests) ; } gen_ :: fuzz :: Fuzz :: < F > :: set_iterations (cfg . fuzz_count) ; TestInfo :: register :: < F , gen_ :: exponents :: LargeExponents < F > > (tests) ; TestInfo :: register :: < F , gen_ :: exponents :: SmallExponents < F > > (tests) ; TestInfo :: register :: < F , gen_ :: fuzz :: Fuzz < F > > (tests) ; TestInfo :: register :: < F , gen_ :: integers :: LargeInt < F > > (tests) ; TestInfo :: register :: < F , gen_ :: integers :: SmallInt > (tests) ; TestInfo :: register :: < F , gen_ :: long_fractions :: RepeatingDecimal > (tests) ; TestInfo :: register :: < F , gen_ :: many_digits :: RandDigits < F > > (tests) ; TestInfo :: register :: < F , gen_ :: sparse :: FewOnesFloat < F > > (tests) ; TestInfo :: register :: < F , gen_ :: sparse :: FewOnesInt < F > > (tests) ; TestInfo :: register :: < F , gen_ :: spot_checks :: RegressionCheck > (tests) ; TestInfo :: register :: < F , gen_ :: spot_checks :: Special > (tests) ; TestInfo :: register :: < F , gen_ :: subnorm :: SubnormComplete < F > > (tests) ; TestInfo :: register :: < F , gen_ :: subnorm :: SubnormEdgeCases < F > > (tests) ; }
};
}
