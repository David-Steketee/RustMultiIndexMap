use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(CreateMultiIndexMap, attributes(MultiIndexRef))]
pub fn create_multi_index_map_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_create_multi_index_map(&ast)
}

fn impl_create_multi_index_map(ast: &syn::DeriveInput) -> TokenStream {
    
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let index_variables = fields.iter().filter_map(|f| {
        for attr in &f.attrs {
            if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "MultiIndexRef" {
                let variable_name = f.ident.as_ref().unwrap();
                let variable_type = &f.ty;
                return Some((variable_name, variable_type))
            }
        }
        None
    });

    let indexed_maps = index_variables.clone().map(|(var_name, var_type)| {
        let hashmap_ident = var_to_map(var_name);
        quote! {
            #hashmap_ident: std::collections::HashMap<#var_type, usize>
        }
    });

    let object_name = &ast.ident;
    let map_name = format!("MultiIndex{}Map", object_name);
    let map_ident = syn::Ident::new(&map_name, object_name.span());
    
    let remove_index_from_maps = index_variables.clone().map(|(var_name, _var_type)| {
        let hashmap_ident = var_to_map(var_name);
        
        quote! {
            self.#hashmap_ident.remove(&obj.#var_name);
        }
    });

    let methods = index_variables.clone().map(|(var_name, var_type)| {
        let hashmap_ident = var_to_map(var_name);
        
        let getter_func_name = format!("get_by_{}", var_name);
        let getter_func_ident = syn::Ident::new(&getter_func_name, var_name.span());

        let remove_func_name = format!("remove_by_{}", var_name);
        let remove_func_ident = syn::Ident::new(&remove_func_name, var_name.span());

        let remove_index_from_maps_local = remove_index_from_maps.clone();

        quote! {
            //get
            pub fn #getter_func_ident(&self, key: #var_type) -> std::option::Option<&#object_name> {
                let idx_opt = self.#hashmap_ident.get(&key);
                if let Some(idx) = idx_opt {
                    return Some(&self.slab[idx.clone()]);
                } else {
                    return None;
                }
            }

            //remove
            pub fn #remove_func_ident(&mut self, key: #var_type) -> Result<#object_name, #var_type> {
               let idx_opt = self.#hashmap_ident.get(&key);
               if let Some(idx) = idx_opt {
                   let obj = self.slab.remove(idx.clone());
            
                   #(#remove_index_from_maps_local);*
            
                   return Result::Ok(obj);
               } else {
                   return Result::Err(key);
               }
            }
        }
    });

    let check_indexes_unused = index_variables.clone().map(|(var_name, _var_type)| {
        let hashmap_ident = var_to_map(var_name);

        quote! {
            if let std::collections::hash_map::Entry::Occupied(_existing) = self.#hashmap_ident.entry(obj.#var_name.clone()) {
                return Result::Err(obj);
            }
        }
    });

    let add_slab_index_to_maps = index_variables.clone().map(|(var_name, _var_type)| {
        let hashmap_ident = var_to_map(var_name);
        quote! {
            self.#hashmap_ident.insert(obj.#var_name.clone(), new_entry.key());
        }
    });

    let insert_func = quote! {
        pub fn try_insert(&mut self, obj: #object_name) -> Result<(), #object_name> {
            #(#check_indexes_unused)*
            let new_entry = self.slab.vacant_entry();
            #(#add_slab_index_to_maps)*
            new_entry.insert(obj);
            return Result::Ok(());
        }
    };

    let new_each_map = index_variables.clone().map(|(var_name, _var_type)| {
        let hashmap_ident = var_to_map(var_name);
        quote! {
            #hashmap_ident: std::collections::HashMap::new()
        }
    });

    let new_func = quote! {
        fn new() -> #map_ident {
            return #map_ident {slab: slab::Slab::new(), #(#new_each_map),*};
        }
    };

    let expanded = quote! {
        
        pub struct #map_ident {
            slab: slab::Slab<#object_name>,
            #(#indexed_maps),*
        }

        impl #map_ident {
            #new_func
            
            #(#methods)*

            #insert_func
        }
    };
    expanded.into()
}

fn var_to_map(var: &syn::Ident) -> syn::Ident {
    let hashmap_name = format!("{}_map", var);
    syn::Ident::new(&hashmap_name, var.span())
}