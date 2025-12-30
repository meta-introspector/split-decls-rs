// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let svg_parsed_map = resvg :: usvg :: Tree :: from_data (& benchlib :: decompress_file (SVG_DATA) , & Options :: default ()) . unwrap () ; let resvg_tree = resvg :: Tree :: from_usvg (& svg_parsed_map) ; run_benchmark_group (| group | { group . register_benchmark ("svg-parse-1" , | | { | | resvg :: usvg :: Tree :: from_data (SVG_DATA , & Options :: default ()) . unwrap () }) ; group . register_benchmark ("svg-render-1" , | | { let width = 1024u32 ; let height = 1024u32 ; let mut buffer = vec ! [0u8 ; (width * height * tiny_skia :: BYTES_PER_PIXEL as u32) as usize] ; let tree = & resvg_tree ; move | | { let mut pixmap = tiny_skia :: PixmapMut :: from_bytes (buffer . as_mut_slice () , width , height) . unwrap () ; tree . render (Transform :: default () , & mut pixmap) ; buffer } }) ; }) ; }
};
}
