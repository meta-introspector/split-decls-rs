macro_rules! deps {
    () => {
        Dependency!();
        Directory!();
    };
}

macro_rules! fix_dependencies {
    () => {
        deps!();
        fn fix_dependencies (dependencies : & mut Map < String , Dependency > , dir : & Directory) { dependencies . remove ("trybuild") ; for dep in dependencies . values_mut () { dep . path = dep . path . as_ref () . map (| path | Directory :: new (dir . join (path))) ; } }
    };
}

fix_dependencies!();