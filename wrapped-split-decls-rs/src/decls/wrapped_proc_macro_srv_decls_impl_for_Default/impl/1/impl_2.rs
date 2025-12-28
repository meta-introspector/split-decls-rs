use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Default for EnvSnapshot { fn default () -> EnvSnapshot { EnvSnapshot { vars : env :: vars_os () . collect () , } } }