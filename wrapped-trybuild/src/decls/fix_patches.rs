macro_rules! deps {
    () => {
        RegistryPatch!();
        Directory!();
    };
}

macro_rules! fix_patches {
    () => {
        deps!();
        fn fix_patches (patches : & mut Map < String , RegistryPatch > , dir : & Directory) { for registry in patches . values_mut () { registry . crates . remove ("trybuild") ; for patch in registry . crates . values_mut () { patch . path = patch . path . as_ref () . map (| path | dir . join (path)) ; } } }
    };
}

fix_patches!()