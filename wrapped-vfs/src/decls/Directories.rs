macro_rules! Directories {
    () => {
        # [doc = " Specifies a set of files on the file system."] # [doc = ""] # [doc = " A file is included if:"] # [doc = "   * it has included extension"] # [doc = "   * it is under an `include` path"] # [doc = "   * it is not under `exclude` path"] # [doc = ""] # [doc = " If many include/exclude paths match, the longest one wins."] # [doc = ""] # [doc = " If a path is in both `include` and `exclude`, the `exclude` one wins."] # [derive (Debug , Clone , Default)] pub struct Directories { pub extensions : Vec < String > , pub include : Vec < AbsPathBuf > , pub exclude : Vec < AbsPathBuf > , }
    };
}

Directories!();