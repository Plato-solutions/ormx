#![cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]

mod attrs;
mod backend;
mod patch;
mod table;
mod utils;



/// Derives [Table](trait.Table.html) and generates a struct for inserting rows and accessors
/// to certain fields.
///
/// # Example
/// ```rust,ignore  
/// #[derive(ormx::Table)]
/// #[ormx(table = "users", id = user_id, insertable)]
/// struct User {
///     #[ormx(column = "id")]
///     user_id: u32,
///     first_name: String,
///     last_name: String,
///     #[ormx(get_optional(&str))]
///     email: String,
///     #[ormx(default, set)]
///     last_login: Option<NaiveDateTime>,
/// }
/// ```
///
/// # The ID
/// It is required that every table contains an ID column, which uniquely
/// identifies a row.  
/// Probably, you would want to use an auto-incrementing integer for this.  
/// This is a central requirement of ormx, and if your table does not fulfill this requirement, ormx
/// is not what you are looking for.
///
/// # CRUD
/// See the documentation of [Table](trait.Table.html)
///
/// # Insertable
/// ormx will generate a helper struct for inserting rows into the database when using
/// `#[ormx(insertable)]`.  
/// This struct will contain all fields of the struct, except
/// - the ID
/// - fields annotated with `#[ormx(default)]`
///
/// since the value of these fields will be generated by the database.
/// By default, this struct will be named `Insert{struct_name}`, though this can be changed by
/// supplying a custom name: `#[ormx(insertable = CreateUser)]`.
/// The generated struct can be used by [Table::insert](trait.Table.html) or
/// [Insert::insert](trait.Insert.html).
///
/// # Deletable
/// ormx will implement [Delete](trait.Delete.html) for your struct when using
/// `#[ormx(deletable)].
///
/// # Accessors: Getters
/// ormx will generate accessor functions for fields annotated with `#[ormx(get_one)]`,
/// `#[ormx(get_optional)]` and `#[ormx(get_many)]`.
/// These functions can be used to query a row by the value of the annotated field.
///
/// The generated function will have these signature:  
/// **`#[ormx(get_one)]`**:  
/// `{pub} async fn get_by_{field_name}(&{field_type}) -> Result<Self>`
///
/// **`#[ormx(get_optional)]`**:  
/// `{pub} async fn get_by_{field_name}(&{field_type}) -> Result<Option<Self>>`
///
/// **`#[ormx(get_many)]`**:  
/// `{pub} async fn get_by_{field_name}(&{field_type}) -> Result<Vec<Self>>`
///
/// By default, the function will be named `get_by_{field_name)`, though this can be changed by
/// supplying a custom name: `#[ormx(get_one = by_id)]`.
/// By default, the function will take a reference to the type of the annotated field as an argument,
/// though this can be changed by supplying a custom type: `#[ormx(get_one(&str)]`.
///
/// # Accessors: Setters
/// ormx will generate accessor functions for fields annotated with `#[ormx(set)]`.
/// These functions can be used to update a single field of an entity.
///
/// The generated function will have these signature:
/// **`#[ormx(set)]`**:
/// `{pub} async fn set_{field_name}(&mut self, {field_type}) -> Result<Self>`
///
/// By default, the function will be named `set_{field_name)`, though this can be changed by
/// supplying a custom name: `#[ormx(set = set_name)]`.
///
/// # Custom types
/// When using custom types (which implement `sqlx::Type`), the field has to annotated with
/// `#[ormx(custom_type)]`.
/// This will use a column type override for querying this field
/// (see [the sqlx docs on this](https://docs.rs/sqlx/0.4.0-beta.1/sqlx/macro.query_as.html#column-type-override-infer-from-struct-field)).
#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Table, attributes(ormx))]
pub fn derive_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match table::derive(input) {
        Ok(ok) => ok,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

/// Derives [Patch](trait.Patch.html).
#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Patch, attributes(ormx))]
pub fn derive_patch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match patch::derive(input) {
        Ok(ok) => ok,
        Err(err) => err.to_compile_error(),
    }
    .into()
}
