macro_rules! deps {
    () => {
        RustcInfo!();
    };
}

macro_rules! ProcessExpandedManifestInputs {
    () => {
        deps!();
        pub struct ProcessExpandedManifestInputs < 'a > { pub expanded_manifest_path : & 'a Path , pub project_root : & 'a Path , pub rustc_info : & 'a RustcInfo , pub verbosity : u8 , pub layer : Option < u32 > , pub canonical_output_root : & 'a Path , pub package_filter : Option < String > , pub log_output_dir : Option < & 'a Path > , pub json_summary_path : Option < & 'a Path > , }
    };
}

ProcessExpandedManifestInputs!();