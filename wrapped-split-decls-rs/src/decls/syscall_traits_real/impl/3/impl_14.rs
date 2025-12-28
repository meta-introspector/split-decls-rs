use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl EnvironmentOps for ProductionEnvironmentOps { fn get_var (& self , key : & str) -> Option < String > { std :: env :: var (key) . ok () } fn set_var (& self , key : & str , value : & str) { std :: env :: set_var (key , value) } fn current_dir (& self) -> IoResult < PathBuf > { std :: env :: current_dir () } fn args (& self) -> Vec < String > { std :: env :: args () . collect () } fn home_dir (& self) -> Option < PathBuf > { std :: env :: var ("HOME") . ok () . map (PathBuf :: from) } }
}