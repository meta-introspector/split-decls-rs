macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! TempPath {
    () => {
        deps!();
        # [doc = " A path to a named temporary file without an open file handle."] # [doc = ""] # [doc = " This is useful when the temporary file needs to be used by a child process,"] # [doc = " for example."] # [doc = ""] # [doc = " When dropped, the temporary file is deleted unless `disable_cleanup(true)` was called on the"] # [doc = " builder that constructed this temporary file and/or was called on either this `TempPath` or the"] # [doc = " `NamedTempFile` from which this `TempPath` was constructed."] pub struct TempPath { path : Box < Path > , disable_cleanup : bool , }
    };
}

TempPath!()