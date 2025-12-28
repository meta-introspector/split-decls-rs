macro_rules! deps {
    () => {
        FileName!();
        FileNameDisplayPreference!();
    };
}

macro_rules! FileNameDisplay {
    () => {
        deps!();
        pub struct FileNameDisplay < 'a > { inner : & 'a FileName , display_pref : FileNameDisplayPreference , }
    };
}

FileNameDisplay!();