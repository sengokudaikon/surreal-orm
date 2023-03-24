/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use darling::{
    ast::{self, Data},
    util, FromDeriveInput, FromField, FromMeta, ToTokens,
};
use proc_macro2::TokenStream;
use surrealdb_query_builder::statements::FieldType;
use syn::{Ident, Lit, LitStr, Path};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    get_crate_name, parse_lit_to_tokenstream,
    relations::{NodeTableName, NodeTypeName},
    variables::VariablesModelMacro,
};
use quote::{format_ident, quote};

#[derive(Debug, Clone)]
pub struct Rename {
    pub(crate) serialize: String,
}

/// This enables us to handle potentially nested values i.e
///   #[serde(rename = "simple_name")]
///    or
///   #[serde(rename(serialize = "age"))]
///  #[serde(rename(serialize = "ser_name_nested", deserialize = "deser_name_nested"))]
/// However, We dont care about deserialized name from serde, so we just ignore that.
impl FromMeta for Rename {
    fn from_string(value: &str) -> ::darling::Result<Self> {
        Ok(Self {
            serialize: value.into(),
        })
    }

    fn from_list(items: &[syn::NestedMeta]) -> ::darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRename {
            serialize: String,

            #[darling(default)]
            deserialize: util::Ignored, // Ignore deserialize since we only care about the serialized string
        }

        impl From<FullRename> for Rename {
            fn from(v: FullRename) -> Self {
                let FullRename { serialize, .. } = v;
                Self { serialize }
            }
        }
        FullRename::from_list(items).map(Rename::from)
    }
}

#[derive(Debug, Clone)]
pub struct Relate {
    /// e.g ->writes->book
    pub connection_model: String,
    // #[darling(default)]
    /// e.g StudentWritesBook,
    /// derived from: type StudentWritesBook = Writes<Student, Book>;
    pub model: Option<String>,
}
//#[rename(se)]
impl FromMeta for Relate {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self {
            connection_model: value.into(),
            model: None,
        })
    }
    //TODO: Check to maybe remove cos I probably dont need this
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRelate {
            model: String,
            connection: String,
        }

        impl From<FullRelate> for Relate {
            fn from(v: FullRelate) -> Self {
                let FullRelate {
                    connection, model, ..
                } = v;
                Self {
                    connection_model: connection,
                    model: Some(model),
                }
            }
        }
        FullRelate::from_list(items).map(Relate::from)
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
pub struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub(crate) ident: ::std::option::Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    pub(crate) ty: syn::Type,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub(crate) rename: ::std::option::Option<Rename>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) relate: ::std::option::Option<Relate>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) link_one: ::std::option::Option<String>,

    // reference singular: LinkSelf<Account>
    #[darling(default)]
    pub(crate) link_self: ::std::option::Option<String>,

    // reference plural: LinkMany<Account>
    #[darling(default)]
    pub(crate) link_many: ::std::option::Option<String>,

    #[darling(default)]
    pub(crate) skip_serializing: bool,

    // #[darling(default)]
    // default: ::std::option::Option<syn::Expr>,
    #[darling(default, rename = "type")]
    pub(crate) type_: ::std::option::Option<FieldTypeWrapper>,

    #[darling(default)]
    pub(crate) assert: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) assert_fn: ::std::option::Option<syn::Path>,

    #[darling(default)]
    pub(crate) define: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) define_fn: ::std::option::Option<syn::Path>,

    #[darling(default)]
    pub(crate) value: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) value_fn: ::std::option::Option<syn::Path>,

    #[darling(default)]
    pub(crate) permissions: ::std::option::Option<Permissions>,

    #[darling(default)]
    pub(crate) permissions_fn: ::std::option::Option<PermissionsFn>,

    #[darling(default)]
    skip_serializing_if: ::darling::util::Ignored,

    #[darling(default)]
    with: ::darling::util::Ignored,
    // #[darling(default)]
    // default: ::darling::util::Ignored,
}

// #[derive(Debug, Clone)]
// pub struct ValueWrapper(syn::LitStr);
//
// impl FromMeta for ValueWrapper {
//     fn from_value(value: &Lit) -> darling::Result<Self> {
//         match value {
//             Lit::Int(i) => i,
//             Lit::Float(f) => f,
//             Lit::Bool(b) => b,
//             Lit::Str(str_lit) => ,
//             Lit::ByteStr(_) => todo!(),
//             Lit::Byte(_) => todo!(),
//             Lit::Char(_) => todo!(),
//             Lit::Verbatim(_) => todo!(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Permissions {
    Full,
    None,
    FnName(LitStr),
}

impl Permissions {
    pub fn get_token_stream(&self) -> TokenStream {
        match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnName(permissions) => {
                let permissions = parse_lit_to_tokenstream(permissions).unwrap();
                quote!(.permissions_for(#permissions))
            }
        }
    }
}

impl FromMeta for Permissions {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str_lit) => {
                let value_str = str_lit.value();

                if value_str.to_lowercase() == "none" {
                    Ok(Self::None)
                } else if value_str.to_lowercase() == "full" {
                    Ok(Self::Full)
                } else {
                    Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
                    // Ok(Self::FnName(str_lit.to_owned()))
                }
                // Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
            }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldTypeWrapper(pub String);

// impl Deref for FieldTypeWrapper {
//     type Target = FieldType;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl FromMeta for FieldTypeWrapper {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.parse::<FieldType>() {
            Ok(f) => Ok(Self(value.to_string())),
            Err(e) => Err(darling::Error::unknown_value(&e)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PermissionsFn {
    Full,
    None,
    FnPath(Path),
}

impl PermissionsFn {
    pub fn get_token_stream(&self) -> TokenStream {
        match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnPath(permissions_fn) => {
                quote!(.permissions_for(#permissions_fn()))
            }
        }
    }
}

impl FromMeta for PermissionsFn {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "full" => Ok(Self::Full),
            _ => Err(darling::Error::unexpected_type(value)),
        }
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str) => Ok(Self::FnPath(syn::parse_str::<Path>(&str.value())?)),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
pub struct FieldsGetterOpts {
    pub(crate) ident: syn::Ident,
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, self::MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: ::std::option::Option<Rename>,

    #[darling(default)]
    pub(crate) table_name: ::std::option::Option<String>,

    #[darling(default)]
    pub(crate) relax_table_name: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) schemafull: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) drop: ::std::option::Option<bool>,

    #[darling(default, rename = "as")]
    pub(crate) as_: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) as_fn: ::std::option::Option<syn::Path>,

    #[darling(default)]
    pub(crate) permissions: ::std::option::Option<Permissions>,

    #[darling(default)]
    pub(crate) permissions_fn: ::std::option::Option<PermissionsFn>,

    #[darling(default)]
    pub(crate) define: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) define_fn: ::std::option::Option<syn::Path>,
}

impl FieldsGetterOpts {
    pub fn get_table_definition_token(&self) -> TokenStream {
        let FieldsGetterOpts {
            ident: ref struct_name_ident,
            ref data,
            ref rename_all,
            ref table_name,
            ref relax_table_name,
            ref drop,
            ref schemafull,
            ref as_,
            ref as_fn,
            ref permissions,
            ref permissions_fn,
            ref define,
            ref define_fn,
            ..
        } = *self;

        let crate_name = super::get_crate_name(false);

        if ((define_fn.is_some() || define.is_some())
            && (drop.is_some()
                || as_.is_some()
                || as_fn.is_some()
                || schemafull.is_some()
                || permissions.is_some()
                || permissions_fn.is_some()))
        {
            panic!("Invalid combinationation. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
                            drop,
                            as,
                            as_fn,
                            schemafull,
                            permissions,
                            permissions_fn");
        }

        let mut define_table: Option<TokenStream> = None;
        let mut define_table_methods = vec![];

        match (define, define_fn){
            (Some(define), None) => {
                let define = parse_lit_to_tokenstream(define).unwrap();
                define_table = Some(quote!(#define));
            },
            (None, Some(define_fn)) => {
                define_table = Some(quote!(#define_fn()));
            },
            (Some(_), Some(_)) => panic!("define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        if let Some(drop) = drop {
            define_table_methods.push(quote!(.drop()))
        }

        match (as_, as_fn){
            (Some(as_), None) => {
                let as_ = parse_lit_to_tokenstream(as_).unwrap();
                define_table_methods.push(quote!(.as_(#as_)))
            },
            (None, Some(as_fn)) => {
                    define_table_methods.push(quote!(#as_fn()));
            },
            (Some(_), Some(_)) => panic!("as and as_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        if let Some(schemafull) = schemafull {
            define_table_methods.push(quote!(.schemafull()))
        }

        match (permissions, permissions_fn){
            (None, Some(p_fn)) => {
                    define_table_methods.push(p_fn.get_token_stream());
            },
            (Some(p), None) => {
                    define_table_methods.push(p.get_token_stream());
            },
            (Some(_), Some(_)) => panic!("permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        define_table.unwrap_or_else(|| {
            quote!(
                #crate_name::statements::define_table(Self::table_name())
                #( #define_table_methods) *
            )
        })
    }
}

#[derive(Default, Clone)]
pub(crate) struct ReferencedNodeMeta {
    pub(crate) foreign_node_type: TokenStream,
    pub(crate) foreign_node_schema_import: TokenStream,
    pub(crate) record_link_default_alias_as_method: TokenStream,
    pub(crate) foreign_node_type_validator: TokenStream,
    pub(crate) field_definition: TokenStream,
}

impl ReferencedNodeMeta {
    pub fn with_field_definition(
        mut self,
        field_receiver: &MyFieldReceiver,
        struct_name_ident_str: &String,
        field_name_normalized: &String,
    ) -> Self {
        let crate_name = get_crate_name(false);
        let mut define_field: Option<TokenStream> = None;
        let mut define_field_methods = vec![];

        match field_receiver {
            MyFieldReceiver {
                define,
                define_fn,
                type_,
                assert,
                assert_fn,
                value,
                value_fn,
                permissions,
                permissions_fn,
                ..
            } if (define_fn.is_some() || define.is_some())
                && (type_.is_some()
                    || assert.is_some()
                    || assert_fn.is_some()
                    || value.is_some()
                    || value_fn.is_some()
                    || permissions.is_some()
                    || permissions_fn.is_some()) =>
            {
                panic!("Invalid combinationation. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:                 
                            type_,
                            assert,
                            assert_fn,
                            value,
                            value_fn,
                            permissions,
                            permissions_fn");
            }
            MyFieldReceiver {
                define: Some(_),
                define_fn: Some(_),
                ..
            } => {
                panic!("define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                define: Some(define),
                ..
            } => {
                let define = parse_lit_to_tokenstream(define).unwrap();
                define_field = Some(quote!(#define));
            }
            MyFieldReceiver {
                define_fn: Some(define_fn),
                ..
            } => {
                define_field = Some(quote!(#define_fn()));
            }
            _ => {}
        };

        if let Some(type_) = &field_receiver.type_ {
            let type_ = type_.0.to_string();
            define_field_methods.push(quote!(.type_(#type_.parse::<#crate_name::statements::FieldType>()
                                                        .expect("Must have been checked at compile time. If not, this is a bug. Please report"))
                                             )
                                      );
        };

        match field_receiver {
            MyFieldReceiver {
                value: Some(value),
                value_fn: Some(value_fn),
                ..
            } => {
                panic!("value and value_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                value: Some(value), ..
            } => {
                let value = parse_lit_to_tokenstream(value).unwrap();
                define_field_methods.push(quote!(.value(#crate_name::Value::from(#value))));
            }
            MyFieldReceiver {
                value_fn: Some(value_fn),
                ..
            } => {
                define_field_methods.push(quote!(.value(#crate_name::Value::from(#value_fn()))));
            }
            _ => {}
        };

        match field_receiver {
            MyFieldReceiver {
                assert: Some(_),
                assert_fn: Some(_),
                ..
            } => {
                panic!("assert and assert_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                assert: Some(assert),
                ..
            } => {
                let assert = parse_lit_to_tokenstream(assert).unwrap();
                define_field_methods.push(quote!(.assert(#assert)));
            }
            MyFieldReceiver {
                assert_fn: Some(assert_fn),
                ..
            } => {
                define_field_methods.push(quote!(.assert(#assert_fn())));
            }
            _ => {}
        };
        match field_receiver {
            MyFieldReceiver {
                permissions: Some(_),
                permissions_fn: Some(_),
                ..
            } => {
                panic!("permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                permissions: Some(permissions),
                ..
            } => {
                define_field_methods.push(permissions.get_token_stream());
            }
            MyFieldReceiver {
                permissions_fn: Some(permissions_fn),
                ..
            } => {
                define_field_methods.push(permissions_fn.get_token_stream());
            }
            _ => {}
        };

        self.field_definition = define_field.unwrap_or_else(||quote!(
                    #crate_name::statements::define_field(#crate_name::Field::new(#field_name_normalized))
                                            .on_table(#crate_name::Table::from(#struct_name_ident_str))
                                            #( # define_field_methods) *
            ));
        self
    }

    pub(crate) fn from_record_link(
        node_type_name: &NodeTypeName,
        normalized_field_name: &::syn::Ident,
        struct_name_ident: &::syn::Ident,
    ) -> Self {
        let VariablesModelMacro {
            __________connect_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let schema_type_ident = format_ident!("{node_type_name}");
        let crate_name = get_crate_name(false);

        let foreign_node_schema_import = if node_type_name.to_string()
            == struct_name_ident.to_string()
        {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #schema_type_ident = <super::#schema_type_ident as #crate_name::SurrealdbNode>::Schema;)
        };
        Self {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SurrealdbNode>::Schema;
            foreign_node_schema_import,

            foreign_node_type_validator: quote!(
                ::static_assertions::assert_impl_one!(#schema_type_ident: #crate_name::SurrealdbNode);
            ),

            record_link_default_alias_as_method: quote!(
                pub fn #normalized_field_name(&self, clause: impl Into<#crate_name::Clause>) -> #schema_type_ident {
                    #schema_type_ident::#__________connect_to_graph_traversal_string(
                        // &self.#___________graph_traversal_string,
                        self.get_connection(),
                        clause,
                        self.get_bindings(),
                        self.get_errors()
                    )

                }
            ),
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
        }
    }
}

pub(crate) struct NormalisedField {
    pub(crate) field_ident_normalised: Ident,
    pub(crate) field_ident_normalised_as_str: String,
}

impl NormalisedField {
    pub(crate) fn from_receiever(
        field_receiver: &MyFieldReceiver,
        struct_level_casing: Option<CaseString>,
    ) -> Self {
        let field_ident = field_receiver.ident.as_ref().unwrap();

        let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
            uncased_field_name: field_ident.to_string(),
            casing: struct_level_casing,
        });

        // get the field's proper serialized format. Renaming should take precedence
        let original_field_name_normalised = &field_receiver.rename.as_ref().map_or_else(
            || field_ident_cased.into(),
            |renamed| renamed.clone().serialize,
        );
        let ref field_ident_normalised = format_ident!("{original_field_name_normalised}");

        let (field_ident_normalised, field_ident_normalised_as_str) =
            if original_field_name_normalised.trim_start_matches("r#") == "in".to_string() {
                (format_ident!("in_"), "in".to_string())
            } else {
                (
                    field_ident_normalised.to_owned(),
                    field_ident_normalised.to_string(),
                )
            };
        let serialized_field_name_no_skip = if field_receiver.skip_serializing {
            None
        } else {
            Some(field_ident_normalised_as_str.clone())
        };
        Self {
            field_ident_normalised,
            field_ident_normalised_as_str,
        }
    }
}
