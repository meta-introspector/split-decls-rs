// Generated macro for alt (module)
macro_rules! Depcrate_maybe_transmutable_testsalt {
() => {
// Module: crate::maybe_transmutable::tests
// Provides: {"alt"}
// Dependencies: {}
mod alt { use super :: * ; use crate :: Answer ; # [test] fn should_permit_identity_transmutation () { type Tree = layout :: Tree < Def , ! , ! > ; let x = Tree :: Seq (vec ! [Tree :: byte (0) , Tree :: byte (0)]) ; let y = Tree :: Seq (vec ! [Tree :: bool () , Tree :: byte (1)]) ; let layout = Tree :: Alt (vec ! [x , y]) ; let answer = crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (layout . clone () , layout . clone () , crate :: Assume :: default () , UltraMinimal :: default () ,) . answer () ; assert_eq ! (answer , Answer :: Yes , "layout:{:#?}" , layout) ; } }
};
}
