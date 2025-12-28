macro_rules! deps {
    () => {
        Lint!();
        Level!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl Lint { pub const fn default_fields_for_macro () -> Self { Lint { name : "" , default_level : Level :: Forbid , desc : "" , edition_lint_opts : None , is_externally_loaded : false , report_in_external_macro : false , future_incompatible : None , feature_gate : None , crate_level_only : false , eval_always : false , } } # [doc = " Gets the lint's name, with ASCII letters converted to lowercase."] pub fn name_lower (& self) -> String { self . name . to_ascii_lowercase () } pub fn default_level (& self , edition : Edition) -> Level { self . edition_lint_opts . filter (| (e , _) | * e <= edition) . map (| (_ , l) | l) . unwrap_or (self . default_level) } }
    };
}

impl_152!()