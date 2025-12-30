// Generated macro for set_panic_hook (function)
macro_rules! Depcrate_uiset_panic_hook {
() => {
// Module: crate::ui
// Provides: {"set_panic_hook"}
// Dependencies: {}
# [doc = " indicatif likes to eat panic messages. This workaround isn't ideal, but it improves things."] # [doc = " <https://github.com/console-rs/indicatif/issues/121>."] pub fn set_panic_hook (drop_bars : & [ProgressBar]) { let hook = std :: panic :: take_hook () ; let drop_bars = drop_bars . to_owned () ; std :: panic :: set_hook (Box :: new (move | info | { for bar in & drop_bars { bar . abandon () ; println ! () ; io :: stdout () . flush () . unwrap () ; io :: stderr () . flush () . unwrap () ; } hook (info) ; })) ; }
};
}
