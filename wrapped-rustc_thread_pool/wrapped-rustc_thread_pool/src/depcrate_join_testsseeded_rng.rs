// Generated macro for seeded_rng (function)
macro_rules! Depcrate_join_testsseeded_rng {
() => {
// Module: crate::join::tests
// Provides: {"seeded_rng"}
// Dependencies: {}
fn seeded_rng () -> XorShiftRng { let mut seed = < XorShiftRng as SeedableRng > :: Seed :: default () ; (0 ..) . zip (seed . as_mut ()) . for_each (| (i , x) | * x = i) ; XorShiftRng :: from_seed (seed) }
};
}
