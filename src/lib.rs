//! This crate defines a macro to imitate the ternary operator
//! found in C-like languages such as C, C++, Java, etc.
//! The macro can be used to make more compact conditional
//! expressions in Rust code.
//!
//! For example, this code in plain Rust:
//! ```
//! let a = 20;
//! let b = 30;
//!
//! // This is the part we will be able to simplify
//! let min = if a < b {
//!     a
//! } else {
//!     b
//! };
//!
//! // Check the result
//! assert_eq!(min, a);
//! ```
//! ... Can be shortened to the following, with this crate:
//!
//! ```
//! let a = 20;
//! let b = 30;
//!
//! // Shortened from the previous example
//! let min = iffy::i!(a < b ? a : b);
//!
//! // Check the result
//! assert_eq!(min, a);
//! ```



extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Token};

type ParseResult<T> = syn::parse::Result<T>;

struct IffyMacro {
    condition: Expr,
    then_branch: Expr,
    else_branch: Expr,
}

impl Parse for IffyMacro {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        //  Expect a condition expression...
        //        Return error on failure,
        //        Otherwise store parsed value
        let condition: Expr = input.parse()?;

        // .. Followed by a `?` token
        //        Return error on failure
        input.parse::<Token![?]>()?;

        // .. Followed by a `then` branch value
        //        Return error on failure,
        //        Otherwise store parsed value
        let then_branch: Expr = input.parse()?;

        // .. Followed by a `:` tokan
        //        Return error on failure
        input.parse::<Token![:]>()?;

        // .. Followed by an `else` branch value
        //        Return error on failure,
        //        Otherwise store parsed value
        let else_branch: Expr = input.parse()?;

        // Finally, return value on success
        Ok(IffyMacro {
            condition,
            then_branch,
            else_branch,
        })
    }
}

/// Ternary operator macro.
///
/// Modeled after the ternary operator in C-like languages.
///
/// # Examples
///
/// **Basic usage**
/// ```
/// // Include the macro
/// use iffy::i;
///
/// // Define some variables to use in the
/// // with the macro, so that we keep things
/// // neat
/// let a = 20;
/// let b = 30;
///
/// // Use the macro in place of Rust's `if..else`
/// // expressions, for more compact code
/// let result = i!(a > b ? "a wins" : "b wins");
///
/// // Make sure `b` is the winner
/// assert_eq!(result, "b wins");
/// ```
///
/// **Nested usage**
///
/// The crate currently does not support nested ternary syntax,
/// so a temporary variable is needed to simulate this.
/// ```
/// // Include the macro
/// use iffy::i;
///
/// // Define some variables to use in the
/// // with the macro, so that we keep things
/// // neat
/// let a = 20;
/// let b = 30;
/// let c = 50;
///
/// // Temporary variables are required for
/// // nesting ternary operators
/// let tmp0 = i!(b > c ? "b wins" : "c wins");
/// let tmp1 = i!(a > c ? "a wins" : "c wins");
///
/// // Use the macro in place of Rust's `if..else`
/// // expressions, for more compact code
/// let result = i!(b > a ? tmp0 : tmp1);
///
/// // Make sure `c` is the winner
/// assert_eq!(result, "c wins");
/// assert_eq!(tmp0, "c wins");
/// assert_eq!(tmp1, "c wins");
///
/// ```
#[proc_macro]
pub fn i(input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let IffyMacro {
        condition,
        then_branch,
        else_branch,
    } = parse_macro_input!(input as IffyMacro);

    // Compose the `if` expression
    let tokens = quote! {
        if #condition {
            #then_branch
        } else {
            #else_branch
        }
    };

    tokens.into()
}
