macro_rules! FileType {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum FileType { Dir , File , Symlink , Unknown , Missing , }
    };
}

FileType!()