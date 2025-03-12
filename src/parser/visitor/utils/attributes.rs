use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::parse::Parser;

use crate::parser::visitor::{processor::AttributeOperations, CodeProcessor};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParsedAttribute {
    pub name: String,
    pub args: Vec<String>,
    pub value: Option<String>,
}

impl<T> AttributeOperations for T
where
    T: CodeProcessor,
{
    fn extract_attributes(&mut self, attrs: &[syn::Attribute]) -> Vec<ParsedAttribute> {
        extract_attributes(attrs)
    }
}

pub fn extract_attributes(attrs: &[syn::Attribute]) -> Vec<ParsedAttribute> {
    attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("doc"))
        .map(parse_attribute)
        .collect()
}

pub fn parse_attribute(attr: &syn::Attribute) -> ParsedAttribute {
    let name = attr.path().to_token_stream().to_string();
    let mut args = Vec::new();

    match &attr.meta {
        syn::Meta::List(list) => {
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;
            if let Ok(nested_metas) = parser.parse2(list.tokens.clone()) {
                for meta in nested_metas {
                    args.push(meta.to_token_stream().to_string());
                }
            }
        }
        syn::Meta::NameValue(name_value) => {
            args.push(name_value.value.to_token_stream().to_string());
        }
        syn::Meta::Path(path) => {
            args.push(path.to_token_stream().to_string());
        }
    }

    ParsedAttribute {
        name,
        args,
        value: Some(attr.to_token_stream().to_string()),
    }
}
