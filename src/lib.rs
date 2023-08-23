mod nonzero;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};

pub(crate) struct SignedInteger {
    is_negative: bool,
    literal: syn::LitInt,
    span: Span,
}

impl Parse for SignedInteger {
    fn parse(input: ParseStream) -> Result<Self> {
        use syn::Token;

        let span = input.span();
        let is_negative = if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            true
        } else {
            false
        };
        let literal = input.parse::<syn::LitInt>()?;

        let output = SignedInteger {
            is_negative,
            literal,
            span,
        };

        Ok(output)
    }
}

#[proc_macro]
pub fn nonzero(input: TokenStream) -> TokenStream {
    let integer = syn::parse_macro_input!(input as SignedInteger);
    let result = nonzero::nonzero(integer);
    match result {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
