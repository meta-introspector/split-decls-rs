macro_rules! deps {
    () => {
        Zip64CentralDirectoryEnd!();
        DataAndPosition!();
        Zip32CentralDirectoryEnd!();
    };
}

macro_rules! CentralDirectoryEndInfo {
    () => {
        deps!();
        pub (crate) struct CentralDirectoryEndInfo { pub eocd : DataAndPosition < Zip32CentralDirectoryEnd > , pub eocd64 : Option < DataAndPosition < Zip64CentralDirectoryEnd > > , pub archive_offset : u64 , }
    };
}

CentralDirectoryEndInfo!();