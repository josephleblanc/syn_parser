// structures.rs
impl StructVisitor for CodeVisitor<'_> {
    fn process_struct(&mut self, item: &ItemStruct) {
        // Move visit_item_struct logic here
    }

    // The below if placeholder, just copied and pasted from old
    // implementation, which started with:
    // impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    fn visit_item_struct(&mut self, item_struct: &'ast ItemStruct) {
        let struct_id = self.state.next_node_id();
        let struct_name = item_struct.ident.to_string();

        // Process fields
        let mut fields = Vec::new();
        for field in &item_struct.fields {
            let field_id = self.state.next_node_id();
            let field_name = field.ident.as_ref().map(|ident| ident.to_string());
            let type_id = self.state.get_or_create_type(&field.ty);

            let field_node = FieldNode {
                id: field_id,
                name: field_name,
                type_id,
                visibility: self.state.convert_visibility(&field.vis),
                attributes: self.state.extract_attributes(&field.attrs),
            };

            // Add relation between struct and field
            self.state.code_graph.relations.push(Relation {
                source: struct_id,
                target: field_id,
                kind: RelationKind::StructField,
            });

            fields.push(field_node);
        }

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_struct.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_struct.attrs);
        let attributes = self.state.extract_attributes(&item_struct.attrs);

        // Store struct info only if public
        if matches!(item_struct.vis, Visibility::Public(_)) {
            self.state
                .code_graph
                .defined_types
                .push(TypeDefNode::Struct(StructNode {
                    id: struct_id,
                    name: struct_name,
                    visibility: self.state.convert_visibility(&item_struct.vis),
                    fields,
                    generic_params,
                    attributes,
                    docstring,
                }));

            visit::visit_item_struct(self, item_struct);
        }
    }

    fn process_enum(&mut self, item: &ItemStruct) {
        // Move visit_item_enum logic here
    }
    // The below if placeholder, just copied and pasted from old
    // implementation, which started with:
    // impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    fn visit_item_enum(&mut self, item_enum: &'ast ItemEnum) {
        let enum_id = self.state.next_node_id();
        let enum_name = item_enum.ident.to_string();

        // Process variants
        let mut variants = Vec::new();
        for variant in &item_enum.variants {
            let variant_id = self.state.next_node_id();
            let variant_name = variant.ident.to_string();

            // Process fields of the variant
            let mut fields = Vec::new();
            match &variant.fields {
                syn::Fields::Named(fields_named) => {
                    for field in &fields_named.named {
                        let field_id = self.state.next_node_id();
                        let field_name = field.ident.as_ref().map(|ident| ident.to_string());
                        let type_id = self.state.get_or_create_type(&field.ty);

                        let field_node = FieldNode {
                            id: field_id,
                            name: field_name,
                            type_id,
                            visibility: self.state.convert_visibility(&field.vis),
                            attributes: self.state.extract_attributes(&field.attrs),
                        };

                        fields.push(field_node);
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    for (_, field) in fields_unnamed.unnamed.iter().enumerate() {
                        let field_id = self.state.next_node_id();
                        let type_id = self.state.get_or_create_type(&field.ty);

                        let field_node = FieldNode {
                            id: field_id,
                            name: None, // Tuple fields don't have names
                            type_id,
                            visibility: self.state.convert_visibility(&field.vis),
                            attributes: self.state.extract_attributes(&field.attrs),
                        };

                        fields.push(field_node);
                    }
                }
                syn::Fields::Unit => {
                    // Unit variants don't have fields
                }
            }

            // Extract discriminant if any
            let discriminant = variant
                .discriminant
                .as_ref()
                .map(|(_, expr)| expr.to_token_stream().to_string());

            let variant_node = VariantNode {
                id: variant_id,
                name: variant_name,
                fields,
                discriminant,
                attributes: self.state.extract_attributes(&variant.attrs),
            };

            // Add relation between enum and variant
            self.state.code_graph.relations.push(Relation {
                source: enum_id,
                target: variant_id,
                kind: RelationKind::EnumVariant,
            });

            variants.push(variant_node);
        }

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_enum.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_enum.attrs);
        let attributes = self.state.extract_attributes(&item_enum.attrs);

        // Store enum info only if public
        if matches!(item_enum.vis, Visibility::Public(_)) {
            self.state
                .code_graph
                .defined_types
                .push(TypeDefNode::Enum(EnumNode {
                    id: enum_id,
                    name: enum_name,
                    visibility: self.state.convert_visibility(&item_enum.vis),
                    variants,
                    generic_params,
                    attributes,
                    docstring,
                }));

            visit::visit_item_enum(self, item_enum);
        }
    }
    fn visit_item_union(&mut self, item_union: &'ast syn::ItemUnion) {
        // Move visit_item_union logic here
    }

    // The below if placeholder, just copied and pasted from old
    // implementation, which started with:
    // impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    fn visit_item_union(&mut self, item_union: &'ast syn::ItemUnion) {
        let union_id = self.state.next_node_id();
        let union_name = item_union.ident.to_string();

        // Process fields
        let mut fields = Vec::new();
        for field in &item_union.fields.named {
            let field_id = self.state.next_node_id();
            let field_name = field.ident.as_ref().map(|ident| ident.to_string());
            let type_id = self.state.get_or_create_type(&field.ty);

            let field_node = FieldNode {
                id: field_id,
                name: field_name,
                type_id,
                visibility: self.state.convert_visibility(&field.vis),
                attributes: self.state.extract_attributes(&field.attrs),
            };

            // Add relation between union and field
            self.state.code_graph.relations.push(Relation {
                source: union_id,
                target: field_id,
                kind: RelationKind::StructField, // Reuse StructField relation for union fields
            });

            fields.push(field_node);
        }

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_union.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_union.attrs);
        let attributes = self.state.extract_attributes(&item_union.attrs);

        // Store union info only if public
        if matches!(item_union.vis, Visibility::Public(_)) {
            self.state
                .code_graph
                .defined_types
                .push(TypeDefNode::Union(UnionNode {
                    id: union_id,
                    name: union_name,
                    visibility: self.state.convert_visibility(&item_union.vis),
                    fields,
                    generic_params,
                    attributes,
                    docstring,
                }));

            visit::visit_item_union(self, item_union);
        }
    }
}
