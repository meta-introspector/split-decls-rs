macro_rules! inherent {
    () => {
        # [cfg_attr (feature = "nightly" , rustc_diagnostic_item = "type_ir_inherent")] pub mod inherent ;
    };
}

inherent!()