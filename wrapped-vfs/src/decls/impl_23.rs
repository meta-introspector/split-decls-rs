macro_rules! deps {
    () => {
        Directories!();
        Entry!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Entry { # [doc = " Returns:"] # [doc = " ```text"] # [doc = " Entry::Directories(Directories {"] # [doc = "     extensions: [\"rs\"],"] # [doc = "     include: [base],"] # [doc = "     exclude: [base/.git],"] # [doc = " })"] # [doc = " ```"] pub fn rs_files_recursively (base : AbsPathBuf) -> Entry { Entry :: Directories (dirs (base , & [".git"])) } # [doc = " Returns:"] # [doc = " ```text"] # [doc = " Entry::Directories(Directories {"] # [doc = "     extensions: [\"rs\"],"] # [doc = "     include: [base],"] # [doc = "     exclude: [base/.git, base/target],"] # [doc = " })"] # [doc = " ```"] pub fn local_cargo_package (base : AbsPathBuf) -> Entry { Entry :: Directories (dirs (base , & [".git" , "target"])) } # [doc = " Returns:"] # [doc = " ```text"] # [doc = " Entry::Directories(Directories {"] # [doc = "     extensions: [\"rs\"],"] # [doc = "     include: [base],"] # [doc = "     exclude: [base/.git, /tests, /examples, /benches],"] # [doc = " })"] # [doc = " ```"] pub fn cargo_package_dependency (base : AbsPathBuf) -> Entry { Entry :: Directories (dirs (base , & [".git" , "/tests" , "/examples" , "/benches"])) } # [doc = " Returns `true` if `path` is included in `self`."] # [doc = ""] # [doc = " See [`Directories::contains_file`]."] pub fn contains_file (& self , path : & AbsPath) -> bool { match self { Entry :: Files (files) => files . iter () . any (| it | it == path) , Entry :: Directories (dirs) => dirs . contains_file (path) , } } # [doc = " Returns `true` if `path` is included in `self`."] # [doc = ""] # [doc = " - If `self` is `Entry::Files`, returns `false`"] # [doc = " - Else, see [`Directories::contains_dir`]."] pub fn contains_dir (& self , path : & AbsPath) -> bool { match self { Entry :: Files (_) => false , Entry :: Directories (dirs) => dirs . contains_dir (path) , } } }
    };
}

impl_23!();