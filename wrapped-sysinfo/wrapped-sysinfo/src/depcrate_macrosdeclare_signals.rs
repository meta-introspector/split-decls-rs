// Generated macro for declare_signals (macro)
macro_rules! Depcrate_macrosdeclare_signals {
() => {
// Module: crate::macros
// Provides: {"declare_signals"}
// Dependencies: {}
# [cfg (feature = "system")] macro_rules ! declare_signals { ($ kind : ty , _ => None ,) => (use crate :: Signal ; pub (crate) const fn supported_signals () -> &'static [Signal] { & [] }) ; ($ kind : ty , $ (Signal ::$ signal : ident => $ map : expr ,) + _ => None ,) => (use crate :: Signal ; pub (crate) const fn supported_signals () -> &'static [Signal] { & [$ (Signal ::$ signal ,) *] } # [inline] pub (crate) fn convert_signal (s : Signal) -> Option <$ kind > { match s { $ (Signal ::$ signal => Some ($ map) ,) * _ => None , } }) ; ($ kind : ty , $ (Signal ::$ signal : ident => $ map : expr ,) +) => (use crate :: Signal ; pub (crate) const fn supported_signals () -> &'static [Signal] { & [$ (Signal ::$ signal ,) *] } # [inline] pub (crate) fn convert_signal (s : Signal) -> Option <$ kind > { match s { $ (Signal ::$ signal => Some ($ map) ,) * } }) }
};
}
