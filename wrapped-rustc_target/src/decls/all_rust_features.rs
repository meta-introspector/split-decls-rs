macro_rules! deps {
    () => {
        Stability!();
    };
}

macro_rules! all_rust_features {
    () => {
        deps!();
        # [doc = " When rustdoc is running, provide a list of all known features so that all their respective"] # [doc = " primitives may be documented."] # [doc = ""] # [doc = " IMPORTANT: If you're adding another feature list above, make sure to add it to this iterator!"] pub fn all_rust_features () -> impl Iterator < Item = (& 'static str , Stability) > { std :: iter :: empty () . chain (ARM_FEATURES . iter ()) . chain (AARCH64_FEATURES . iter ()) . chain (X86_FEATURES . iter ()) . chain (HEXAGON_FEATURES . iter ()) . chain (POWERPC_FEATURES . iter ()) . chain (MIPS_FEATURES . iter ()) . chain (NVPTX_FEATURES . iter ()) . chain (RISCV_FEATURES . iter ()) . chain (WASM_FEATURES . iter ()) . chain (BPF_FEATURES . iter ()) . chain (CSKY_FEATURES) . chain (LOONGARCH_FEATURES) . chain (IBMZ_FEATURES) . chain (SPARC_FEATURES) . chain (M68K_FEATURES) . cloned () . map (| (f , s , _) | (f , s)) }
    };
}

all_rust_features!();