macro_rules! deps {
    () => {
        DeflateConfig!();
        Strategy!();
        Method!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Default for DeflateConfig { fn default () -> Self { Self { level : crate :: c_api :: Z_DEFAULT_COMPRESSION , method : Method :: Deflated , window_bits : MAX_WBITS , mem_level : DEF_MEM_LEVEL , strategy : Strategy :: Default , } } }
    };
}

impl_131!()