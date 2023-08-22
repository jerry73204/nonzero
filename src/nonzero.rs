use crate::SignedInteger;
use num_traits::Zero;
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
        (_, "") => {
            return Err(syn::Error::new(span, "suffix is required"));
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
