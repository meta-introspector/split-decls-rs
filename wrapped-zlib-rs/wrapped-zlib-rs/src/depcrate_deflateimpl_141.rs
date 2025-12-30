// Generated macro for impl_141 (impl)
macro_rules! Depcrate_deflateimpl_141 {
() => {
// Module: crate::deflate
// Provides: {"impl_141"}
// Dependencies: {}
# [cfg (any (test , feature = "__internal-test"))] impl quickcheck :: Arbitrary for DeflateConfig { fn arbitrary (g : & mut quickcheck :: Gen) -> Self { let mem_levels : Vec < _ > = (1 ..= 9) . collect () ; let levels : Vec < _ > = (0 ..= 9) . collect () ; let mut window_bits = Vec :: new () ; window_bits . extend (9 ..= 15) ; window_bits . extend (9 + 16 ..= 15 + 16) ; window_bits . extend (- 15 ..= - 9) ; Self { level : * g . choose (& levels) . unwrap () , method : Method :: Deflated , window_bits : * g . choose (& window_bits) . unwrap () , mem_level : * g . choose (& mem_levels) . unwrap () , strategy : * g . choose (& [Strategy :: Default , Strategy :: Filtered , Strategy :: HuffmanOnly , Strategy :: Rle , Strategy :: Fixed ,]) . unwrap () , } } }
};
}
