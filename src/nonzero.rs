use crate::SignedInteger;
use num_traits::{Bounded, Signed, Unsigned, Zero};
use proc_macro2::TokenStream;
use quote::quote;

/// # Compilation failure tests
///
/// ## `unsigned integer cannot be negative`
///
/// ```compile_fail
/// use nonzero::nonzero as nz;
///
/// let _ = nz!(-1usize);
/// ```
///
/// ## `zero is not allowed`
///
/// ```compile_fail
/// use nonzero::nonzero as nz;
///
/// let _ = nz!(0u8);
/// ```
///
/// ## `cannot infer type`
///
/// ```compile_fail
/// use std::num::NonZeroU32;
/// use nonzero::nonzero as nz;
///
/// // NonZeroU32 implements PartialEq to many types.
/// assert_eq!(nz!(123), NonZeroU32::new(123).unwrap());
/// ```
///
/// ## `the trait `From<NonZeroU32>` is not implemented for `NonZeroI8``
///
/// ```compile_fail
/// use std::num::NonZeroI8;
/// use nonzero::nonzero as nz;
///
/// let _val: NonZeroI8 = nz!(123456789);
/// ```
///
/// ## `suffix is not supported`
///
/// ```compile_fail
/// use nonzero::nonzero as nz;
///
/// let _ = nz!(1_is_not_supported);
/// ```
pub(crate) fn nonzero(integer: SignedInteger) -> syn::Result<TokenStream> {
    let SignedInteger {
        is_negative,
        literal: lit,
        span,
    } = integer;

    let neg = if is_negative {
        quote! {-}
    } else {
        quote! {}
    };

    let tokens = match (is_negative, lit.suffix()) {
        (false, "usize") => {
            let val: usize = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroUsize::new_unchecked(#lit)
                }
            }
        }
        (false, "u8") => {
            let val: u8 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroU8::new_unchecked(#lit)
                }
            }
        }
        (false, "u16") => {
            let val: u16 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroU16::new_unchecked(#lit)
                }
            }
        }
        (false, "u32") => {
            let val: u32 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroU32::new_unchecked(#lit)
                }
            }
        }
        (false, "u64") => {
            let val: u64 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroU64::new_unchecked(#lit)
                }
            }
        }
        (true, "usize" | "u8" | "u16" | "u32" | "u64") => {
            return Err(syn::Error::new(span, "unsigned integer cannot be negative"))
        }
        (_, "isize") => {
            let val: isize = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroIsize::new_unchecked(#neg #lit)
                }
            }
        }
        (_, "i8") => {
            let val: i8 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroI8::new_unchecked(#neg #lit)
                }
            }
        }
        (_, "i16") => {
            let val: i16 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroI16::new_unchecked(#neg #lit)
                }
            }
        }
        (_, "i32") => {
            let val: i32 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroI32::new_unchecked(#neg #lit)
                }
            }
        }
        (_, "i64") => {
            let val: i64 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            quote! {
                unsafe {
                    core::num::NonZeroI64::new_unchecked(#neg #lit)
                }
            }
        }
        (false, "") => {
            let val: u128 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            if in_unsigned_range::<u8>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroU8::new_unchecked(#lit).into()
                    }
                }
            } else if in_unsigned_range::<u16>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroU16::new_unchecked(#lit).into()
                    }
                }
            } else if in_unsigned_range::<u32>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroU32::new_unchecked(#lit).into()
                    }
                }
            } else if in_unsigned_range::<u64>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroU64::new_unchecked(#lit).into()
                    }
                }
            } else {
                quote! {
                    unsafe {
                        core::num::NonZeroU128::new_unchecked(#lit).into()
                    }
                }
            }
        }
        (true, "") => {
            let val: i128 = lit.base10_parse()?;
            check_zero(&lit, val)?;
            if in_signed_range::<i8>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroI8::new_unchecked(#neg #lit).into()
                    }
                }
            } else if in_signed_range::<i16>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroI16::new_unchecked(#neg #lit).into()
                    }
                }
            } else if in_signed_range::<i32>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroI32::new_unchecked(#neg #lit).into()
                    }
                }
            } else if in_signed_range::<i64>(val) {
                quote! {
                    unsafe {
                        core::num::NonZeroI64::new_unchecked(#neg #lit).into()
                    }
                }
            } else {
                quote! {
                    unsafe {
                        core::num::NonZeroI128::new_unchecked(#neg #lit).into()
                    }
                }
            }
        }
        (_, suffix) => {
            return Err(syn::Error::new(
                span,
                format!("the suffix '{}' is not supported", suffix),
            ));
        }
    };

    Ok(tokens)
}

fn check_zero(lit: impl quote::ToTokens, val: impl Zero) -> syn::Result<()> {
    if val.is_zero() {
        Err(syn::Error::new_spanned(lit, "zero is not allowed"))
    } else {
        Ok(())
    }
}

fn in_unsigned_range<T>(val: u128) -> bool
where
    T: Bounded + Unsigned,
    u128: From<T>,
{
    let min = u128::from(T::min_value());
    let max = u128::from(T::max_value());
    (min..=max).contains(&val)
}

fn in_signed_range<T>(val: i128) -> bool
where
    T: Bounded + Signed,
    i128: From<T>,
{
    let min = i128::from(T::min_value());
    let max = i128::from(T::max_value());
    (min..=max).contains(&val)
}
