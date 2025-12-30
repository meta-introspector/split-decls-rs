// Generated macro for impl_634 (impl)
macro_rules! Depcrate_ule_nicheimpl_634 {
() => {
// Module: crate::ule::niche
// Provides: {"impl_634"}
// Dependencies: {}
# [doc = " Safety for ULE trait"] # [doc = " 1. NichedOptionULE does not have any padding bytes due to `#[repr(C)]` on a struct"] # [doc = "    containing only ULE fields."] # [doc = "    NichedOptionULE either contains NICHE_BIT_PATTERN or valid U byte sequences."] # [doc = "    In both cases the data is initialized."] # [doc = " 2. NichedOptionULE is aligned to 1 byte due to `#[repr(C, packed)]` on a struct containing only"] # [doc = "    ULE fields."] # [doc = " 3. validate_bytes impl returns an error if invalid bytes are encountered."] # [doc = " 4. validate_bytes impl returns an error there are extra bytes."] # [doc = " 5. The other ULE methods are left to their default impl."] # [doc = " 6. NichedOptionULE equality is based on ULE equality of the subfield, assuming that NicheBytes"] # [doc = "    has been implemented correctly (this is a correctness but not a safety guarantee)."] unsafe impl < U : NicheBytes < N > + ULE , const N : usize > ULE for NichedOptionULE < U , N > { fn validate_bytes (bytes : & [u8]) -> Result < () , crate :: ule :: UleError > { let size = size_of :: < Self > () ; debug_assert ! (N == core :: mem :: size_of ::< U > ()) ; if bytes . len () % size != 0 { return Err (crate :: ule :: UleError :: length :: < Self > (bytes . len ())) ; } bytes . chunks (size) . try_for_each (| chunk | { if chunk == < U as NicheBytes < N > > :: NICHE_BIT_PATTERN { Ok (()) } else { U :: validate_bytes (chunk) } }) } }
};
}
