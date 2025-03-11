use syn::{Attribute, Expr, Lit, Meta, MetaNameValue};

pub trait DocProcessor {
    fn extract_docstring(&mut self, attrs: &[syn::Attribute]) -> Option<String>;
}

pub fn extract_docstring(attrs: &[Attribute]) -> Option<String> {
    let doc_lines: Vec<String> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Ok(Meta::NameValue(MetaNameValue {
                value: Lit::Str(lit_str),
                ..
            })) = attr.parse_meta()
            {
                Some(lit_str.value().trim().to_string())
            } else {
                None
            }
        })
        .collect();

    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join("\n"))
    }
}
