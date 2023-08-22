use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};
use nonzero::nonzero as nz;

#[test]
fn test() {
    // unsigned integers
    assert_eq!(nz!(1usize), NonZeroUsize::new(1).unwrap());
    assert_eq!(nz!(1u8), NonZeroU8::new(1).unwrap());
    assert_eq!(nz!(1u16), NonZeroU16::new(1).unwrap());
    assert_eq!(nz!(1u32), NonZeroU32::new(1).unwrap());
    assert_eq!(nz!(1u64), NonZeroU64::new(1).unwrap());

    // signed integers
    assert_eq!(nz!(1isize), NonZeroIsize::new(1).unwrap());
    assert_eq!(nz!(1i8), NonZeroI8::new(1).unwrap());
    assert_eq!(nz!(1i16), NonZeroI16::new(1).unwrap());
    assert_eq!(nz!(1i32), NonZeroI32::new(1).unwrap());
    assert_eq!(nz!(1i64), NonZeroI64::new(1).unwrap());

    // negative signed integers
    assert_eq!(nz!(-1isize), NonZeroIsize::new(-1).unwrap());
    assert_eq!(nz!(-1i8), NonZeroI8::new(-1).unwrap());
    assert_eq!(nz!(-1i16), NonZeroI16::new(-1).unwrap());
    assert_eq!(nz!(-1i32), NonZeroI32::new(-1).unwrap());
    assert_eq!(nz!(-1i64), NonZeroI64::new(-1).unwrap());
}

#[test]
fn type_inference() {
    #[derive(Debug, PartialEq)]
    struct MyNonZeroIsize(NonZeroIsize);
    assert_eq!(
        MyNonZeroIsize(nz!(-1)),
        MyNonZeroIsize(NonZeroIsize::new(-1).unwrap()),
    );

    #[derive(Debug, PartialEq)]
    struct MyNonZeroU16(NonZeroU16);
    assert_eq!(
        MyNonZeroU16(nz!(1234)),
        MyNonZeroU16(NonZeroU16::new(1234).unwrap()),
    );

    fn non_zero() -> NonZeroI64 {
        nz!(9)
    }
    assert_eq!(non_zero(), NonZeroI64::new(9).unwrap());

    fn double(val: NonZeroUsize) -> NonZeroUsize {
        val.checked_mul(NonZeroUsize::new(2).unwrap()).unwrap()
    }
    assert_eq!(double(nz!(2500)), NonZeroUsize::new(5000).unwrap());
}
