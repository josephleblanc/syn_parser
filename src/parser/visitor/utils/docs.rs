use syn::parse;
// use syn::{Attribute, Expr, Lit, Meta, MetaNameValue};
use syn::{Attribute, Expr, Lit, Meta, MetaNameValue};

pub trait DocProcessor {
    // fn extract_docstring(&mut self, attrs: &[Attribute]) -> Option<String>;
    fn extract_docstring(&mut self, attrs: &[Attribute]) -> Option<String> {
        let doc_lines: Vec<String> = attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .filter_map(|attr| {
                if let Ok(Meta::NameValue(MetaNameValue {
                    value: meta_value, ..
                })) = attr.parse_args::<Meta>()
                {
                    if let Expr::Lit(expr) = meta_value {
                        // lit_str.lit.to_string()
                        if let Lit::Str(lit_str) = &expr.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
                None
            })
            .collect();

        (!doc_lines.is_empty()).then(|| doc_lines.join("\n"))
    }
}

// impl<'ast> DocProcessor<'ast> for CodeVisitor<'_> {}
pub fn extract_docstring(attrs: &[Attribute]) -> Option<String> {
    let doc_lines: Vec<String> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Ok(Meta::NameValue(MetaNameValue {
                value: meta_value, ..
            })) = attr.parse_args::<Meta>()
            {
                if let Expr::Lit(expr) = meta_value {
                    // lit_str.lit.to_string()
                    if let Lit::Str(lit_str) = &expr.lit {
                        return Some(lit_str.value());
                    }
                }
            }
            None
        })
        .collect();

    (!doc_lines.is_empty()).then(|| doc_lines.join("\n"))
}
