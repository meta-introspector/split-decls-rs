macro_rules! FileExcluded {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum FileExcluded { Yes , No , }
    };
}

FileExcluded!();