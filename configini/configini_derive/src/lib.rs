use quote::{format_ident, quote};


/// Derive macro Configini
///
/// The idea is that you can derive Configini for the struct that will
/// hold your configuration and automatically obtain a wrapper around
/// your struct with getter and setter for the pub field of the struct
/// and load and save function. Obviously then your struct cannot
/// contain any load or save field.
/// The wrapper type generated will be called as your struct with 'Manager'
/// at the end, i.e the struct *Config* will generate a *ConfigManger*
/// struct.
/// It will also add a `new` method to your struct that return a manager
/// for the configuration. So your struct cannot have a new method either.
#[proc_macro_derive(Configini)]
pub fn derive_into_hash_map(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let mut config_implementation = quote! {
    };

    let config_identifier = &input.ident;
    let manager_identifier = format_ident!("{}Manager", config_identifier);

    match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            config_implementation.extend(quote! {

                pub struct #manager_identifier {
                    pub path: String,
                    config: #config_identifier,
                }

                impl #config_identifier {
                    pub fn new(file_path: &str) -> Result<#manager_identifier, configini::ConfiginiError> {
                        Ok(#manager_identifier {
                            path: file_path.to_string(),
                            config: Self::default(),
                        })
                    }
                }
            });

            let mut save_block = quote!{
                let mut ini_file = configini::Ini::new();
            };
            
            let mut load_block = quote!{
                let mut ini_file = configini::Ini::load_from_file(&self.path).expect("Failed to load the config file");
            };

            for field in fields {
                if let syn::Visibility::Public(_) = field.vis {
                    if let Some(ident) = &field.ident
                        && let syn::Type::Path(path) = &field.ty
                        && path.path.segments.len() == 1
                    {
                        let field_name = ident.to_string();
                        let field_ident = format_ident!("{}", field_name);
                        let set_field_ident = format_ident!("set_{}", field_name);
                        let field_ty = format_ident!("{}", path.path.segments[0].ident.to_string());
                        
                        config_implementation.extend(quote!{
                            impl #manager_identifier {
                                pub fn #field_ident(&self) -> #field_ty {
                                    self.config.#field_ident
                                }

                                pub fn #set_field_ident(&mut self, value: #field_ty) {
                                    self.config.#field_ident = value;
                                }
                            }
                        });

                        save_block.extend(quote!{
                            ini_file.with_section::<String>(None).set(
                                #field_name,
                                self.config.#field_ident.to_string(),
                            );
                        });

                        load_block.extend(quote!{
                            if let Some(value) = ini_file.with_section::<String>(None).get(#field_name) {
                                if let Ok(value) = value.parse() {
                                    self.config.#field_ident = value;
                                };
                            }
                        });
                    }
                }
            }

            save_block.extend(quote!{
                ini_file.write_to_file(&self.path).expect("Failed to save the config file");
            });

            let save_block = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, quote!{#save_block});
            config_implementation.extend(quote!{
                impl #manager_identifier {
                    pub fn save(&self) #save_block
                }
            });

            let load_block = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, quote!{#load_block});
            config_implementation.extend(quote!{
                impl #manager_identifier {
                    pub fn load(&mut self) #load_block
                }
            });


        }
        _ => unimplemented!("Configini derive macro is implemented only for structs"),
    }

    let config_implementation_ts = config_implementation.into();
    config_implementation_ts
}
