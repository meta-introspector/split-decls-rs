use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl EnvironmentOps for MockEnvironmentOps { fn get_var (& self , key : & str) -> Option < String > { self . vars . get (key) . cloned () } fn set_var (& self , key : & str , value : & str) { } fn current_dir (& self) -> IoResult < PathBuf > { Ok (PathBuf :: from ("/mock/current/dir")) } fn args (& self) -> Vec < String > { vec ! ["mock_program" . to_string ()] } fn home_dir (& self) -> Option < PathBuf > { Some (PathBuf :: from ("/mock/home")) } }