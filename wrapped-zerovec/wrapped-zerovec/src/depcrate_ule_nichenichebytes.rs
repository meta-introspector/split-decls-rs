// Generated macro for NicheBytes (trait)
macro_rules! Depcrate_ule_nicheNicheBytes {
() => {
// Module: crate::ule::niche
// Provides: {"NicheBytes"}
// Dependencies: {}
# [doc = " The [`ULE`] types implementing this trait guarantee that [`NicheBytes::NICHE_BIT_PATTERN`]"] # [doc = " can never occur as a valid byte representation of the type."] # [doc = ""] # [doc = " Guarantees for a valid implementation."] # [doc = " 1. N must be equal to `core::mem::sizeo_of::<Self>()` or else it will"] # [doc = "    cause panics."] # [doc = " 2. The bit pattern [`NicheBytes::NICHE_BIT_PATTERN`] must not be incorrect as it would lead to"] # [doc = "    weird behaviour."] # [doc = " 3. The abstractions built on top of this trait must panic on an invalid N."] # [doc = " 4. The abstractions built on this trait that use type punning must ensure that type being"] # [doc = "    punned is [`ULE`]."] pub trait NicheBytes < const N : usize > { const NICHE_BIT_PATTERN : [u8 ; N] ; }
};
}
