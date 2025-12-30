// Generated macro for rustc (module)
macro_rules! Depcrate_maybe_transmutablerustc {
() => {
// Module: crate::maybe_transmutable
// Provides: {"rustc"}
// Dependencies: {}
# [cfg (feature = "rustc")] mod rustc { use rustc_middle :: ty :: layout :: LayoutCx ; use rustc_middle :: ty :: { Ty , TyCtxt , TypingEnv } ; use super :: * ; use crate :: layout :: tree :: rustc :: Err ; impl < 'tcx > MaybeTransmutableQuery < Ty < 'tcx > , TyCtxt < 'tcx > > { # [doc = " This method begins by converting `src` and `dst` from `Ty`s to `Tree`s,"] # [doc = " then computes an answer using those trees."] # [instrument (level = "debug" , skip (self) , fields (src = ? self . src , dst = ? self . dst))] pub (crate) fn answer (self ,) -> Answer < < TyCtxt < 'tcx > as QueryContext > :: Region , < TyCtxt < 'tcx > as QueryContext > :: Type > { let Self { src , dst , assume , context } = self ; let layout_cx = LayoutCx :: new (context , TypingEnv :: fully_monomorphized ()) ; let src = Tree :: from_ty (src , layout_cx) ; let dst = Tree :: from_ty (dst , layout_cx) ; match (src , dst) { (Err (Err :: TypeError (_)) , _) | (_ , Err (Err :: TypeError (_))) => { Answer :: No (Reason :: TypeError) } (Err (Err :: UnknownLayout) , _) => Answer :: No (Reason :: SrcLayoutUnknown) , (_ , Err (Err :: UnknownLayout)) => Answer :: No (Reason :: DstLayoutUnknown) , (Err (Err :: NotYetSupported) , _) => Answer :: No (Reason :: SrcIsNotYetSupported) , (_ , Err (Err :: NotYetSupported)) => Answer :: No (Reason :: DstIsNotYetSupported) , (Err (Err :: SizeOverflow) , _) => Answer :: No (Reason :: SrcSizeOverflow) , (_ , Err (Err :: SizeOverflow)) => Answer :: No (Reason :: DstSizeOverflow) , (Ok (src) , Ok (dst)) => MaybeTransmutableQuery { src , dst , assume , context } . answer () , } } } }
};
}
