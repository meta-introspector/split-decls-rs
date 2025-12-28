macro_rules! VirtualPath {
    () => {
        # [doc = " `/`-separated virtual path."] # [doc = ""] # [doc = " This is used to describe files that do not reside on the file system."] # [derive (Debug , Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] struct VirtualPath (String) ;
    };
}

VirtualPath!();