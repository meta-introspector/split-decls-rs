macro_rules! deps {
    () => {
        FileSetConfig!();
        VfsPath!();
    };
}

macro_rules! FileSetConfigBuilder {
    () => {
        deps!();
        # [doc = " Builder for [`FileSetConfig`]."] # [derive (Default)] pub struct FileSetConfigBuilder { roots : Vec < Vec < VfsPath > > , }
    };
}

FileSetConfigBuilder!()