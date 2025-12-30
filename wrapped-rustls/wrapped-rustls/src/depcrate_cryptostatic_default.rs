// Generated macro for static_default (module)
macro_rules! Depcrate_cryptostatic_default {
() => {
// Module: crate::crypto
// Provides: {"static_default"}
// Dependencies: {}
mod static_default { # [cfg (not (feature = "std"))] use alloc :: boxed :: Box ; # [cfg (feature = "std")] use std :: sync :: OnceLock ; # [cfg (not (feature = "std"))] use once_cell :: race :: OnceBox ; use super :: CryptoProvider ; use crate :: sync :: Arc ; # [cfg (feature = "std")] pub (crate) fn install_default (default_provider : CryptoProvider ,) -> Result < () , Arc < CryptoProvider > > { PROCESS_DEFAULT_PROVIDER . set (Arc :: new (default_provider)) } # [cfg (not (feature = "std"))] pub (crate) fn install_default (default_provider : CryptoProvider ,) -> Result < () , Arc < CryptoProvider > > { PROCESS_DEFAULT_PROVIDER . set (Box :: new (Arc :: new (default_provider))) . map_err (| e | * e) } pub (crate) fn get_default () -> Option < & 'static Arc < CryptoProvider > > { PROCESS_DEFAULT_PROVIDER . get () } # [cfg (feature = "std")] static PROCESS_DEFAULT_PROVIDER : OnceLock < Arc < CryptoProvider > > = OnceLock :: new () ; # [cfg (not (feature = "std"))] static PROCESS_DEFAULT_PROVIDER : OnceBox < Arc < CryptoProvider > > = OnceBox :: new () ; }
};
}
