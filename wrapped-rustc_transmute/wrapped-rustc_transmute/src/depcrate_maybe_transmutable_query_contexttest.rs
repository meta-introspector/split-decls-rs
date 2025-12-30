// Generated macro for test (module)
macro_rules! Depcrate_maybe_transmutable_query_contexttest {
() => {
// Module: crate::maybe_transmutable::query_context
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] pub (crate) mod test { use std :: marker :: PhantomData ; use super :: QueryContext ; pub (crate) struct UltraMinimal < R = ! , T = ! > (PhantomData < (R , T) >) ; impl < R , T > Default for UltraMinimal < R , T > { fn default () -> Self { Self (PhantomData) } } # [derive (Debug , Hash , Eq , PartialEq , Clone , Copy)] pub (crate) enum Def { HasSafetyInvariants , NoSafetyInvariants , } impl crate :: layout :: Def for Def { fn has_safety_invariants (& self) -> bool { self == & Self :: HasSafetyInvariants } } impl < R , T > QueryContext for UltraMinimal < R , T > where R : crate :: layout :: Region , T : crate :: layout :: Type , { type Def = Def ; type Region = R ; type Type = T ; } }
};
}
