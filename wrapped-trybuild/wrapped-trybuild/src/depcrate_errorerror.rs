// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum Error { Cargo (io :: Error) , CargoFail , GetManifest (PathBuf , Box < Error >) , Glob (GlobError) , Io (io :: Error) , Metadata (serde_json :: Error) , Mismatch , NoWorkspaceManifest , Open (PathBuf , io :: Error) , Pattern (PatternError) , ProjectDir , ReadStderr (io :: Error) , RunFailed , ShouldNotHaveCompiled , TomlDe (toml :: de :: Error) , TomlSer (toml :: ser :: Error) , UpdateVar (OsString) , WriteStderr (io :: Error) , }
};
}
