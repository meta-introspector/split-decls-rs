macro_rules! VisitDelimited {
    () => {
        # [doc = " A visitor wrapper that inserts a delimiter after the wrapped visitor formats"] # [doc = " a field value."] # [derive (Debug)] pub struct VisitDelimited < D , V > { delimiter : D , seen : bool , inner : V , err : fmt :: Result , }
    };
}

VisitDelimited!();