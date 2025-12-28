use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn iteratively_generate_macro (intent : & DwimIntent) -> TokenStream { let state_url = share_generation_state (intent) ; quote ! { compile_error ! (concat ! ("No suitable macro found. Generated state at: " , # state_url , ". Please define appropriate macro or refine intent.")) ; } . into () }