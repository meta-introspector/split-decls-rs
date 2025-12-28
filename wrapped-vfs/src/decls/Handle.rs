macro_rules! deps {
    () => {
        Sender!();
        Config!();
    };
}

macro_rules! Handle {
    () => {
        deps!();
        # [doc = " Interface for reading and watching files."] pub trait Handle : fmt :: Debug { # [doc = " Spawn a new handle with the given `sender`."] fn spawn (sender : Sender) -> Self where Self : Sized ; # [doc = " Set this handle's configuration."] fn set_config (& mut self , config : Config) ; # [doc = " The file's content at `path` has been modified, and should be reloaded."] fn invalidate (& mut self , path : AbsPathBuf) ; # [doc = " Load the content of the given file, returning [`None`] if it does not"] # [doc = " exists."] fn load_sync (& mut self , path : & AbsPath) -> Option < Vec < u8 > > ; }
    };
}

Handle!();