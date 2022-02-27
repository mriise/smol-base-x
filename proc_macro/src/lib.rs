use proc_macro::{self, TokenStream};
use quote::ToTokens;
use syn::{
    parse::Parse, parse_macro_input, parse_quote, Arm, Expr,
    ExprMatch, LitStr, Token,
};

struct CharMatch {
    expr: Box<Expr>,
    alphabet: LitStr,
}

impl Parse for CharMatch {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr: Box<Expr> = input.parse()?;
        input.parse::<Token![,]>()?;
        let alphabet: LitStr = input.parse()?;

        Ok(CharMatch {
            expr,
            alphabet,
        })
    }
}

#[proc_macro]
pub fn gen_char_match(input: TokenStream) -> TokenStream {
    let CharMatch {
        expr,
        alphabet,
    } = parse_macro_input!(input as CharMatch);

    let cases = alphabet.value();

    let mut arms: Vec<Arm> = cases
        .char_indices()
        .map(|(i, ch)| -> Arm { parse_quote!(#ch => Some(#i),) })
        .collect();

    arms.push(parse_quote!(_ => None,));

let match_exp = ExprMatch {
        attrs: Default::default(),
        match_token: Default::default(),
        brace_token: Default::default(),
        expr,
        arms,
    };

    match_exp.into_token_stream().into()
}
