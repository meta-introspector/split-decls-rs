macro_rules! deps {
    () => {
        ZipResult!();
        Zip32CDEBlock!();
        Zip32CentralDirectoryEnd!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Zip32CentralDirectoryEnd { fn into_block_and_comment (self) -> (Zip32CDEBlock , Box < [u8] >) { let Self { disk_number , disk_with_central_directory , number_of_files_on_this_disk , number_of_files , central_directory_size , central_directory_offset , zip_file_comment , } = self ; let block = Zip32CDEBlock { magic : Zip32CDEBlock :: MAGIC , disk_number , disk_with_central_directory , number_of_files_on_this_disk , number_of_files , central_directory_size , central_directory_offset , zip_file_comment_length : zip_file_comment . len () as u16 , } ; (block , zip_file_comment) } pub fn parse < T : Read > (reader : & mut T) -> ZipResult < Zip32CentralDirectoryEnd > { let Zip32CDEBlock { disk_number , disk_with_central_directory , number_of_files_on_this_disk , number_of_files , central_directory_size , central_directory_offset , zip_file_comment_length , .. } = Zip32CDEBlock :: parse (reader) ? ; let mut zip_file_comment = vec ! [0u8 ; zip_file_comment_length as usize] . into_boxed_slice () ; if let Err (e) = reader . read_exact (& mut zip_file_comment) { if e . kind () == io :: ErrorKind :: UnexpectedEof { return Err (invalid ! ("EOCD comment exceeds file boundary")) ; } return Err (e . into ()) ; } Ok (Zip32CentralDirectoryEnd { disk_number , disk_with_central_directory , number_of_files_on_this_disk , number_of_files , central_directory_size , central_directory_offset , zip_file_comment , }) } pub fn write < T : Write > (self , writer : & mut T) -> ZipResult < () > { let (block , comment) = self . into_block_and_comment () ; if comment . len () > u16 :: MAX as usize { return Err (invalid ! ("EOCD comment length exceeds u16::MAX")) ; } block . write (writer) ? ; writer . write_all (& comment) ? ; Ok (()) } pub fn may_be_zip64 (& self) -> bool { self . number_of_files == u16 :: MAX || self . central_directory_offset == u32 :: MAX } }
    };
}

impl_163!();