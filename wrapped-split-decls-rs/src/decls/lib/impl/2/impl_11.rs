use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl syn :: parse :: Parse for TestWrapperInput { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { let wrapper_path = input . parse () ? ; input . parse :: < syn :: Token ! [,] > () ? ; let test_name = input . parse () ? ; Ok (TestWrapperInput { wrapper_path , test_name , }) } }
}