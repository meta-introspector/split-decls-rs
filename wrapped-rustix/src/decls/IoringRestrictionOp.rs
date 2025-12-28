macro_rules! IoringRestrictionOp {
    () => {
        # [doc = " `IORING_RESTRICTION_*` constants for use with [`io_uring_restriction`]."] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (u16)] # [non_exhaustive] pub enum IoringRestrictionOp { # [doc = " `IORING_RESTRICTION_REGISTER_OP`"] RegisterOp = sys :: io_uring_register_restriction_op :: IORING_RESTRICTION_REGISTER_OP as _ , # [doc = " `IORING_RESTRICTION_SQE_FLAGS_ALLOWED`"] SqeFlagsAllowed = sys :: io_uring_register_restriction_op :: IORING_RESTRICTION_SQE_FLAGS_ALLOWED as _ , # [doc = " `IORING_RESTRICTION_SQE_FLAGS_REQUIRED`"] SqeFlagsRequired = sys :: io_uring_register_restriction_op :: IORING_RESTRICTION_SQE_FLAGS_REQUIRED as _ , # [doc = " `IORING_RESTRICTION_SQE_OP`"] SqeOp = sys :: io_uring_register_restriction_op :: IORING_RESTRICTION_SQE_OP as _ , }
    };
}

IoringRestrictionOp!()