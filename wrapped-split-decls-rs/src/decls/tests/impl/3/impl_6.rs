use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < O , E > OutcomeTrait for TestOutcome < O , E > where O : Clone , { type Error = Error < O , E > ; type Obligation = O ; fn new () -> Self { Self { errors : vec ! [] , completed : vec ! [] } } fn record_completed (& mut self , outcome : & Self :: Obligation) { self . completed . push (outcome . clone ()) } fn record_error (& mut self , error : Self :: Error) { self . errors . push (error) } }
}