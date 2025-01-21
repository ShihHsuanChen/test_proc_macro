//! Tutorial: https://www.youtube.com/watch?v=SMCRQj9Hbx8
//! Convert python list comprehension syntax into rust code
//! using procedural macros (functional). For example: 
//! comp![ x/y for x,y in mylist if y != 0 ]
//!
//! ### General macro concept
//! Frontend (source)
//!    |
//!    | syn
//!    |
//!    |___Intermediate Representation____
//!                                       |
//!                                quote! |
//!                                       |
//!                             Backend (target)
//!
//! ### Grammar:
//! - comp: mapping for_if_clause+
//! >>> exmple 1: ... for_if_clause 
//! >>> exmple 2: ... for_if_clause for_if_clause ...
//!
//! - mapping: expression
//!
//! - for_if_clause:
//!  | 'for' pattern 'in' expression ('if' expression)*
//! >>> example 1: for ... in ...
//! >>> example 2: for ... in ... if ...
//! >>> example 3: for ... in ... if ... if ...
//!
//! - pattern: name (,name)*
//! >>> example 1: a
//! >>> example 2: a, b
//!
//! ### Rust syntax
//! In the case of 
//!
//! ```python 
//! x * 2 for x in xs if x > 0
//! ```
//!
//! The rust code should be 
//!
//! ```rust
//! IntoIterator::into_iter(xs)
//!   .flat_map(|x| {
//!     (x > 0).then(|| x * 2)
//!   })
//! ```
//!
//! or in general
//! ```rust
//! IntoIterator::into_iter(<expression>)
//!   .flat_map(|<pattern>| {
//!     (true (&& <expression>)*).then(|| <mapping>)
//!   })
//! ```

use syn::parse::{Parse, ParseStream};
use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

// Implement the simpler comprehension only
// comp: mapping for_if_clause
// for_if_clause:
//  | 'for' pattern 'in' expression ('if' expression)*
// pattern: name (,name)*

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    let c: Comp = syn::parse_macro_input!(input as Comp);
    quote! { #c }.into()
}


/// comp: mapping for_if_clause
struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}
// frontend
impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse::<Mapping>()?,
            for_if_clause: input.parse::<ForIfClause>()?,
        })
    }
}
// backend
impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // core::iter::IntoIterator::into_iter(<sequence>)
        //   .flat_map(|<pattern>| {
        //     (true (&& <expression>)*).then(|| <mapping>)
        //   })
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern, sequence, conditions,
        } = &self.for_if_clause;

        tokens.extend(quote! {
            core::iter::IntoIterator::into_iter(#sequence)
                .flat_map(move |#pattern| {
                    (true #(&& #conditions)* ).then(|| { #mapping })
                })
        });
    }
}

/// mapping: expression
struct Mapping (syn::Expr);
// frontend
impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // store expression into this object and return
        input.parse().map(Self)
    }
}
// backend
impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}


/// for_if_clause:
///  | 'for' pattern 'in' sequence ('if' expression)*
struct ForIfClause {
    pattern: Pattern,
    sequence: syn::Expr,
    conditions: Vec<Condition>, // zero or more
}
// frontend
impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // check if the input stream starts with the `for` token
        let _ = input.parse::<syn::Token![for]>()?;
        // parse pattern following by the `for` token
        let pattern: Pattern = input.parse()?;
        // check if the `in` token following by the pattern
        let _ = input.parse::<syn::Token![in]>()?;
        // parse sequence following by the `in` token
        let sequence: syn::Expr = input.parse()?;
        // parse conditions
        let conditions: Vec<Condition> = parse_zero_or_more(input);
        Ok(Self {
            pattern,
            sequence,
            conditions,
        })

    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    while let Ok(item) = input.parse::<T>() {
        result.push(item);
    }
    result
}

/// pattern: name (, name)*
struct Pattern (syn::Pat);
// frontend
impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // store expression into this object and return
        syn::Pat::parse_single(input).map(Self) 
        // identical to Ok(Self(syn::Pat::parse_single(input)?))
    }
}
// backend
impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}


/// condition (or 'if' expression)
struct Condition (syn::Expr);
// frontend
impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // check if the input stream starts with the `if` token
        let _ = input.parse::<syn::Token![if]>()?;
        // store expression into this object and return
        input.parse().map(Self)
    }
}
// backend
impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}
