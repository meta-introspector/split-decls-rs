macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! generate_chrono_datetime {
    () => {
        deps!();
        # [cfg (feature = "chrono")] # [doc = " Generate a `NaiveDateTime` from a `DateTime`."] fn generate_chrono_datetime (datetime : & DateTime) -> Option < chrono :: NaiveDateTime > { if let Some (d) = chrono :: NaiveDate :: from_ymd_opt (datetime . year () . into () , datetime . month () . into () , datetime . day () . into () ,) { if let Some (d) = d . and_hms_opt (datetime . hour () . into () , datetime . minute () . into () , datetime . second () . into () ,) { return Some (d) ; } } None }
    };
}

generate_chrono_datetime!();