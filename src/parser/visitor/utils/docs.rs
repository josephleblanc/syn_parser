use syn::{Attribute, Meta, MetaNameValue, Lit};

pub trait DocProcessor {
    fn extract_docstring(&mut self, attrs: &[Attribute]) -> Option<String>;
}

pub fn extract_docstring(attrs: &[Attribute]) -> Option<String> {
    let doc_lines: Vec<String> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit_str), .. })) = 
                attr.parse_args::<Meta>()
            {
                Some(lit_str.value().trim().to_string())
            } else {
                None
            }
        })
        .collect();

    (!doc_lines.is_empty()).then(|| doc_lines.join("\n"))
}
