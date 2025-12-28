macro_rules! deps {
    () => {
        RecordFields!();
    };
}

macro_rules! VisitOutput {
    () => {
        deps!();
        # [doc = " A [visitor] that produces output once it has visited a set of fields."] # [doc = ""] # [doc = " [visitor]: tracing_core::field::Visit"] pub trait VisitOutput < Out > : Visit { # [doc = " Completes the visitor, returning any output."] # [doc = ""] # [doc = " This is called once a full set of fields has been visited."] fn finish (self) -> Out ; # [doc = " Visit a set of fields, and return the output of finishing the visitor"] # [doc = " once the fields have been visited."] fn visit < R > (mut self , fields : & R) -> Out where R : RecordFields , Self : Sized , { fields . record (& mut self) ; self . finish () } }
    };
}

VisitOutput!()