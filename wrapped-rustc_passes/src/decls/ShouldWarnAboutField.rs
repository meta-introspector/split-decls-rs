macro_rules! ShouldWarnAboutField {
    () => {
        enum ShouldWarnAboutField { Yes , No , }
    };
}

ShouldWarnAboutField!()