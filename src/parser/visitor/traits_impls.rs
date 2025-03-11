impl ImplVisitor for CodeVisitor<'_> {
    fn process_impl(&mut self, item: &ItemImpl) {
        // Move visit_item_impl logic here
    }
    // The below if placeholder, just copied and pasted from old
    // implementation, which started with:
    // impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    fn visit_item_impl(&mut self, item_impl: &'ast ItemImpl) {
        let impl_id = self.state.next_node_id();

        // Process self type
        let self_type_id = self.state.get_or_create_type(&item_impl.self_ty);

        // Process trait type if it's a trait impl
        let trait_type_id = item_impl.trait_.as_ref().map(|(_, path, _)| {
            let ty = Type::Path(TypePath {
                qself: None,
                path: path.clone(),
            });
            let trait_id = self.state.get_or_create_type(&ty);
            trait_id
        });

        // Skip impl blocks for non-public traits
        if let Some(trait_type_id) = trait_type_id {
            if let Some(trait_type) = self
                .state
                .code_graph
                .type_graph
                .iter()
                .find(|t| t.id == trait_type_id)
            {
                if let TypeKind::Named { path, .. } = &trait_type.kind {
                    let trait_name = path.last().unwrap_or(&String::new()).to_string();
                    let trait_def = self
                        .state
                        .code_graph
                        .traits
                        .iter()
                        .find(|t| t.name == trait_name);

                    if let Some(trait_def) = trait_def {
                        if !matches!(trait_def.visibility, VisibilityKind::Public) {
                            // Skip this impl as the trait is not public
                            return;
                        }
                    } else {
                        // Trait definition not found, skip this impl
                        return;
                    }
                }
            }
        }

        // Process methods
        let mut methods = Vec::new();
        for item in &item_impl.items {
            if let syn::ImplItem::Fn(method) = item {
                let method_node_id = self.state.next_node_id();
                let method_name = method.sig.ident.to_string();

                // Process method parameters
                let mut parameters = Vec::new();
                for arg in &method.sig.inputs {
                    if let Some(param) = self.state.process_fn_arg(arg) {
                        // Add relation between method and parameter
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: param.id,
                            kind: RelationKind::FunctionParameter,
                        });
                        parameters.push(param);
                    }
                }

                // Extract return type if it exists
                let return_type = match &method.sig.output {
                    ReturnType::Default => None,
                    ReturnType::Type(_, ty) => {
                        let type_id = self.state.get_or_create_type(ty);
                        // Add relation between method and return type
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: type_id,
                            kind: RelationKind::FunctionReturn,
                        });
                        Some(type_id)
                    }
                };

                // Process generic parameters for methods
                let generic_params = self.state.process_generics(&method.sig.generics);

                // Extract doc comments and other attributes for methods
                let docstring = self.state.extract_docstring(&method.attrs);
                let attributes = self.state.extract_attributes(&method.attrs);

                // Extract method body as a string
                let body = Some(method.block.to_token_stream().to_string());

                // Store method info
                let method_node = FunctionNode {
                    id: method_node_id,
                    name: method_name,
                    visibility: self.state.convert_visibility(&method.vis),
                    parameters,
                    return_type,
                    generic_params,
                    attributes,
                    docstring,
                    body,
                };
                methods.push(method_node);
            }
        }

        // Process generic parameters for impl block
        let generic_params = self.state.process_generics(&item_impl.generics);

        // Store impl info
        let impl_node = ImplNode {
            id: impl_id,
            self_type: self_type_id,
            trait_type: trait_type_id,
            methods,
            generic_params,
        };
        self.state.code_graph.impls.push(impl_node);

        // Add relation: ImplementsFor or ImplementsTrait
        let relation_kind = if trait_type_id.is_some() {
            RelationKind::ImplementsTrait
        } else {
            RelationKind::ImplementsFor
        };
        self.state.code_graph.relations.push(Relation {
            source: impl_id,
            target: self_type_id,
            kind: relation_kind,
        });
        if let Some(trait_type_id) = trait_type_id {
            self.state.code_graph.relations.push(Relation {
                source: impl_id,
                target: trait_type_id,
                kind: RelationKind::ImplementsTrait,
            });

            // Debug: Print trait type information
            if let Some(trait_type) = self
                .state
                .code_graph
                .type_graph
                .iter()
                .find(|t| t.id == trait_type_id)
            {
                if let TypeKind::Named { path, .. } = &trait_type.kind {
                    println!("Found trait implementation: {:?}", path);
                    // Specific check for DefaultTrait implementation
                    if path.last().unwrap_or(&String::new()) == "DefaultTrait" {
                        if let Some(self_type) = self
                            .state
                            .code_graph
                            .type_graph
                            .iter()
                            .find(|t| t.id == self_type_id)
                        {
                            if let TypeKind::Named { path, .. } = &self_type.kind {
                                println!("Self type for DefaultTrait: {:?}", path);
                                if path.last().unwrap_or(&String::new()) == "ModuleStruct" {
                                    println!("Found DefaultTrait implementation for ModuleStruct");
                                }
                            }
                        }
                    }
                }
            }

            // Debug: Print self type information
            if let Some(self_type) = self
                .state
                .code_graph
                .type_graph
                .iter()
                .find(|t| t.id == self_type_id)
            {
                if let TypeKind::Named { path, .. } = &self_type.kind {
                    println!("Self type: {:?}", path);
                }
            }

            // Debug: Print all methods in the impl
            if let Some(debug_impl) = &self.state.code_graph.impls.last() {
                for method in &debug_impl.methods {
                    println!("Found method {} in impl {}", method.name, impl_id);
                }
            }
        }

        visit::visit_item_impl(self, item_impl);
    }
}

impl ImplVisitor for CodeVisitor<'_> {
    fn process_trait(&mut self, item: &ItemImpl) {
        // Move visit_item_trait logic here
    }
    // The below if placeholder, just copied and pasted from old
    // implementation, which started with:
    // impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    fn visit_item_trait(&mut self, item_trait: &'ast ItemTrait) {
        let trait_id = self.state.next_node_id();
        let trait_name = item_trait.ident.to_string();

        // Process methods
        let mut methods = Vec::new();
        for item in &item_trait.items {
            if let syn::TraitItem::Fn(method) = item {
                let method_node_id = self.state.next_node_id();
                let method_name = method.sig.ident.to_string();

                // Process method parameters
                let mut parameters = Vec::new();
                for arg in &method.sig.inputs {
                    if let Some(param) = self.state.process_fn_arg(arg) {
                        // Add relation between method and parameter
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: param.id,
                            kind: RelationKind::FunctionParameter,
                        });
                        parameters.push(param);
                    }
                }

                // Extract return type if it exists
                let return_type = match &method.sig.output {
                    ReturnType::Default => None,
                    ReturnType::Type(_, ty) => {
                        let type_id = self.state.get_or_create_type(ty);
                        // Add relation between method and return type
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: type_id,
                            kind: RelationKind::FunctionReturn,
                        });
                        Some(type_id)
                    }
                };

                // Process generic parameters for methods
                let generic_params = self.state.process_generics(&method.sig.generics);

                // Extract doc comments and other attributes for methods
                let docstring = self.state.extract_docstring(&method.attrs);
                let attributes = self.state.extract_attributes(&method.attrs);

                // Extract method body if available (trait methods may have default implementations)
                let body = method
                    .default
                    .as_ref()
                    .map(|block| block.to_token_stream().to_string());

                // Store method info
                let method_node = FunctionNode {
                    id: method_node_id,
                    name: method_name,
                    visibility: VisibilityKind::Public, // Trait methods are always public
                    parameters,
                    return_type,
                    generic_params,
                    attributes,
                    docstring,
                    body,
                };
                methods.push(method_node);
            }
        }

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_trait.generics);

        // Process super traits
        let super_traits: Vec<TypeId> = item_trait
            .supertraits
            .iter()
            .map(|bound| {
                let ty = Type::TraitObject(syn::TypeTraitObject {
                    dyn_token: None,
                    bounds: syn::punctuated::Punctuated::from_iter(vec![bound.clone()]),
                });
                self.state.get_or_create_type(&ty)
            })
            .collect();

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_trait.attrs);
        let attributes = self.state.extract_attributes(&item_trait.attrs);

        // Store trait info
        //  Commenting out below because we should be able to see all traits regardless of
        //  visibility
        // if matches!(item_trait.vis, Visibility::Public(_)) {
        let trait_node = TraitNode {
            id: trait_id,
            name: trait_name.clone(),
            visibility: self.state.convert_visibility(&item_trait.vis),
            methods,
            generic_params,
            super_traits: super_traits.clone(),
            attributes,
            docstring,
        };
        self.state.code_graph.traits.push(trait_node);
        // }

        // Add relation for super traits
        for super_trait_id in &super_traits {
            self.state.code_graph.relations.push(Relation {
                source: trait_id,
                target: *super_trait_id,
                kind: RelationKind::Inherits,
            });
        }

        visit::visit_item_trait(self, item_trait);
    }
}
