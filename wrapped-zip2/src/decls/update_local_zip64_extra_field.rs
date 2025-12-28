macro_rules! deps {
    () => {
        ZipResult!();
        ZipLocalEntryBlock!();
        ZipFileData!();
    };
}

macro_rules! update_local_zip64_extra_field {
    () => {
        deps!();
        fn update_local_zip64_extra_field < T : Write + Seek > (writer : & mut T , file : & mut ZipFileData ,) -> ZipResult < () > { let block = file . zip64_extra_field_block () . ok_or (invalid ! ("Attempted to update a nonexistent ZIP64 extra field")) ? ; let zip64_extra_field_start = file . header_start + size_of :: < ZipLocalEntryBlock > () as u64 + file . file_name_raw . len () as u64 ; writer . seek (SeekFrom :: Start (zip64_extra_field_start)) ? ; let block = block . serialize () ; writer . write_all (& block) ? ; let extra_field = Arc :: get_mut (file . extra_field . as_mut () . unwrap ()) . unwrap () ; extra_field [.. block . len ()] . copy_from_slice (& block) ; Ok (()) }
    };
}

update_local_zip64_extra_field!();