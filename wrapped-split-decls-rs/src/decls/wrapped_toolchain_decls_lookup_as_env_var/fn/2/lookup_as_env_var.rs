use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Looks up the binary as its SCREAMING upper case in the env variables."] fn lookup_as_env_var (executable_name : & str) -> Option < Utf8PathBuf > { env :: var_os (executable_name . to_ascii_uppercase ()) . map (PathBuf :: from) . map (Utf8PathBuf :: try_from) . and_then (Result :: ok) }