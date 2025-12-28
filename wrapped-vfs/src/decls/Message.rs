macro_rules! deps {
    () => {
        LoadingProgress!();
        Handle!();
        Config!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [doc = " Message about an action taken by a [`Handle`]."] pub enum Message { # [doc = " Indicate a gradual progress."] # [doc = ""] # [doc = " This is supposed to be the number of loaded files."] Progress { # [doc = " The total files to be loaded."] n_total : usize , # [doc = " The files that have been loaded successfully."] n_done : LoadingProgress , # [doc = " The dir being loaded, `None` if its for a file."] dir : Option < AbsPathBuf > , # [doc = " The [`Config`] version."] config_version : u32 , } , # [doc = " The handle loaded the following files' content for the first time."] Loaded { files : Vec < (AbsPathBuf , Option < Vec < u8 > >) > } , # [doc = " The handle loaded the following files' content."] Changed { files : Vec < (AbsPathBuf , Option < Vec < u8 > >) > } , }
    };
}

Message!()