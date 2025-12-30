// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: Data ; # [test] fn constraint_len_calculator () { let test_data = vec ! [Data { name : "Emirhan Tala" . to_string () , address : "Cambridgelaan 6XX\n3584 XX Utrecht" . to_string () , email : "tala.emirhan@gmail.com" . to_string () , } , Data { name : "thistextis26characterslong" . to_string () , address : "this line is 31 characters long\nbottom line is 33 characters long" . to_string () , email : "thisemailis40caharacterslong@ratatui.com" . to_string () , } ,] ; let (longest_name_len , longest_address_len , longest_email_len) = crate :: constraint_len_calculator (& test_data) ; assert_eq ! (26 , longest_name_len) ; assert_eq ! (33 , longest_address_len) ; assert_eq ! (40 , longest_email_len) ; } }
};
}
