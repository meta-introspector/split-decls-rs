macro_rules! deps {
    () => {
        VisitOutput!();
    };
}

macro_rules! VisitFmt {
    () => {
        deps!();
        # [doc = " Extension trait implemented by visitors to indicate that they write to a"] # [doc = " `fmt::Write` instance, and allow access to that writer."] pub trait VisitFmt : VisitOutput < fmt :: Result > { # [doc = " Returns the formatter that this visitor writes to."] fn writer (& mut self) -> & mut dyn fmt :: Write ; }
    };
}

VisitFmt!();