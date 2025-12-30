// Generated macro for dirs (function)
macro_rules! Depcrate_loaderdirs {
() => {
// Module: crate::loader
// Provides: {"dirs"}
// Dependencies: {}
# [doc = " Returns :"] # [doc = " ```text"] # [doc = " Directories {"] # [doc = "     extensions: [\"rs\"],"] # [doc = "     include: [base],"] # [doc = "     exclude: [base/<exclude>],"] # [doc = " }"] # [doc = " ```"] fn dirs (base : AbsPathBuf , exclude : & [& str]) -> Directories { let exclude = exclude . iter () . map (| it | base . join (it)) . collect :: < Vec < _ > > () ; Directories { extensions : vec ! ["rs" . to_owned ()] , include : vec ! [base] , exclude } }
};
}
