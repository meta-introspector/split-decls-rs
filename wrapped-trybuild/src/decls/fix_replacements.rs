macro_rules! deps {
    () => {
        Directory!();
        Patch!();
    };
}

macro_rules! fix_replacements {
    () => {
        deps!();
        fn fix_replacements (replacements : & mut Map < String , Patch > , dir : & Directory) { replacements . remove ("trybuild") ; for replacement in replacements . values_mut () { replacement . path = replacement . path . as_ref () . map (| path | dir . join (path)) ; } }
    };
}

fix_replacements!()