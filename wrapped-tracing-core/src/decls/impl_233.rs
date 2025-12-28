macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl Id { # [doc = " Constructs a new span ID from the given `u64`."] # [doc = ""] # [doc = " <pre class=\"ignore\" style=\"white-space:normal;font:inherit;\">"] # [doc = "     <strong>Note</strong>: Span IDs must be greater than zero."] # [doc = " </pre>"] # [doc = ""] # [doc = " # Panics"] # [doc = " - If the provided `u64` is 0."] pub fn from_u64 (u : u64) -> Self { Id (NonZeroU64 :: new (u) . expect ("span IDs must be > 0")) } # [doc = " Constructs a new span ID from the given `NonZeroU64`."] # [doc = ""] # [doc = " Unlike [`Id::from_u64`](Id::from_u64()), this will never panic."] # [inline] pub const fn from_non_zero_u64 (id : NonZeroU64) -> Self { Id (id) } # [allow (clippy :: wrong_self_convention)] # [doc = " Returns the span's ID as a `u64`."] pub fn into_u64 (& self) -> u64 { self . 0 . get () } # [allow (clippy :: wrong_self_convention)] # [doc = " Returns the span's ID as a `NonZeroU64`."] # [inline] pub const fn into_non_zero_u64 (& self) -> NonZeroU64 { self . 0 } }
    };
}

impl_233!()