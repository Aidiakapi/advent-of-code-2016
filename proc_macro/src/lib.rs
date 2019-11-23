#![feature(
    proc_macro_diagnostic,
    proc_macro_quote,
    proc_macro_span,
    proc_macro_hygiene
)]

extern crate proc_macro;
use proc_macro::{
    quote, Delimiter, Diagnostic, Ident, Level, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};
use std::convert::Into;
use std::iter::FromIterator;

fn generate_module_list_impl(token_stream: TokenStream) -> Result<TokenStream, Diagnostic> {
    // parse input
    struct Module {
        name: Ident,
        parts: Vec<Ident>,
        parser: Option<Ident>,
    }
    let mut modules: Vec<Module> = Vec::new();
    let mut tokens = token_stream.into_iter().peekable();

    let const_ident = match tokens.next() {
        Some(TokenTree::Ident(const_ident)) => const_ident,
        Some(token) => return Err(token.span().error("expected constant name")),
        None => return Err(Diagnostic::new(Level::Error, "expected arguments")),
    };
    match tokens.next() {
        Some(TokenTree::Punct(punct))
            if punct.as_char() == ';' && punct.spacing() == Spacing::Alone => {}
        Some(token) => return Err(token.span().error("expected ;")),
        None => {}
    }

    while let Some(module_ident) = tokens.next() {
        if let TokenTree::Ident(module_ident) = module_ident {
            module_ident.span().warning("found name");
            let group = match tokens.next() {
                Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => group,
                Some(ident) => return Err(ident.span().error("expected brackets [ ]")),
                None => return Err(Diagnostic::new(Level::Error, "unexpected end of input")),
            };

            let mut group_tokens = group.stream().into_iter();
            let mut parts = Vec::new();
            while let Some(part_ident) = group_tokens.next() {
                if let TokenTree::Ident(part_ident) = part_ident {
                    parts.push(part_ident);
                } else {
                    return Err(part_ident.span().error("expected part name"));
                }
                match group_tokens.next() {
                    None => {}
                    Some(TokenTree::Punct(punct))
                        if punct.as_char() == ',' && punct.spacing() == Spacing::Alone => {}
                    Some(token) => return Err(token.span().error("expected ,")),
                }
            }

            let parser = match tokens.peek().clone() {
                Some(TokenTree::Punct(punct))
                    if punct.as_char() == ':' && punct.spacing() == Spacing::Alone =>
                {
                    let punct = tokens.next().unwrap();
                    if let Some(TokenTree::Ident(input_ident)) = tokens.next() {
                        Some(input_ident)
                    } else {
                        return Err(punct.span().error("expected parser name after"));
                    }
                }
                _ => None,
            };

            modules.push(Module {
                name: module_ident,
                parts,
                parser,
            });

            match tokens.next() {
                None => {}
                Some(TokenTree::Punct(punct))
                    if punct.as_char() == ',' && punct.spacing() == Spacing::Alone => {}
                Some(token) => {
                    return Err(token.span().error("expected ,"));
                }
            }
        } else {
            return Err(module_ident.span().error("expected module name"));
        }
    }

    let mut tokens: Vec<TokenTree> = Vec::new();
    // emit module imports
    for module in &modules {
        tokens.push(Ident::new("mod", Span::call_site()).into());
        tokens.push(module.name.clone().into());
        tokens.push(Punct::new(';', Spacing::Alone).into());
    }

    // emit table
    let mut table_tokens = TokenStream::new();
    for module in modules {
        let module_name: TokenTree = Literal::string(&module.name.to_string()).into();
        // emit parts
        let mut part_tokens = TokenStream::new();
        for part in &module.parts {
            let part_name: TokenTree = Literal::string(&part.to_string()).into();
            let call_part = TokenStream::from_iter(vec![
                TokenTree::Ident(module.name.clone()),
                Punct::new(':', Spacing::Joint).into(),
                Punct::new(':', Spacing::Joint).into(),
                part.clone().into(),
            ]);
            match module.parser.clone() {
                Some(parser) => {
                    let call_parser = TokenStream::from_iter(vec![
                        TokenTree::Ident(module.name.clone()),
                        Punct::new(':', Spacing::Joint).into(),
                        Punct::new(':', Spacing::Joint).into(),
                        parser.clone().into(),
                    ]);
                    part_tokens.extend(quote!(
                        ($part_name, |input: &str| -> Result<String> {
                            use ::anyhow::anyhow;
                            let input = input.trim();
                            let input = $call_parser(input)
                                .map_err(|err| anyhow!("parse error {:?}", err))
                                .and_then(|(remainder, v)| {
                                    if remainder == "" {
                                        Ok(v)
                                    } else {
                                        Err(anyhow!("input partially parsed, remainder: {:#?}", remainder))
                                    }
                                })?;
                            Ok(format!("{}", $call_part(input)?))
                        }),
                    ))
                }
                None => {
                    part_tokens.extend(quote!(
                        ($part_name, |input: &str| -> Result<String> {
                            let input = input.trim();
                            Ok(format!("{}", $call_part(input)?))
                        }),
                    ));
                }
            }
        }

        table_tokens.extend(quote!(
            ($module_name, &[
                $part_tokens
            ]),
        ));
    }

    // emit DAY_LIST
    tokens.push(Ident::new("const", Span::call_site()).into());
    tokens.push(const_ident.into());
    tokens.push(Punct::new(':', Spacing::Alone).into());
    tokens.extend(
        quote!(&[(&'static str, &[(&'static str, fn(&str) -> anyhow::Result<String>)])] = {
        use std::fmt::Write;
        use anyhow::Result;

        &[
            $table_tokens
        ]
    };),
    );

    let tokens = TokenStream::from_iter(tokens);
    // println!("{}", tokens);
    Ok(tokens)
}

#[proc_macro]
pub fn generate_module_list(token_stream: TokenStream) -> TokenStream {
    generate_module_list_impl(token_stream).unwrap_or_else(|diag| {
        diag.emit();
        TokenStream::new()
    })
}
