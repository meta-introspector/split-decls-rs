use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl syn :: parse :: Parse for UseWrapperInput { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { let original_path = input . parse () ? ; input . parse :: < syn :: Token ! [=>] > () ? ; let wrapper_path = input . parse () ? ; Ok (UseWrapperInput { original_path , wrapper_path , }) } }
}