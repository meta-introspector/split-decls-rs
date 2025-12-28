macro_rules! deps {
    () => {
        CentralDirectoryInfo!();
        DataAndPosition!();
        ZipError!();
        CentralDirectoryEndInfo!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a CentralDirectoryEndInfo > for CentralDirectoryInfo { type Error = ZipError ; fn try_from (value : & 'a CentralDirectoryEndInfo) -> Result < Self , Self :: Error > { let (relative_cd_offset , number_of_files , disk_number , disk_with_central_directory) = match & value . eocd64 { Some (DataAndPosition { data : eocd64 , .. }) => { if eocd64 . number_of_files_on_this_disk > eocd64 . number_of_files { return Err (invalid ! ("ZIP64 footer indicates more files on this disk than in the whole archive")) ; } (eocd64 . central_directory_offset , eocd64 . number_of_files as usize , eocd64 . disk_number , eocd64 . disk_with_central_directory ,) } _ => (value . eocd . data . central_directory_offset as u64 , value . eocd . data . number_of_files_on_this_disk as usize , value . eocd . data . disk_number as u32 , value . eocd . data . disk_with_central_directory as u32 ,) , } ; let directory_start = relative_cd_offset . checked_add (value . archive_offset) . ok_or (invalid ! ("Invalid central directory size or offset")) ? ; Ok (Self { archive_offset : value . archive_offset , directory_start , number_of_files , disk_number , disk_with_central_directory , }) } }
    };
}

impl_102!()