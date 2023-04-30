//! This crate aims to provide data structures that represent the [AsyncAPI v 2.3.0 specification](https://www.asyncapi.com/docs/specifications/v2.3.0).
//!
//! This crate is still in an early development stage. The official valid test files are all parsed without errors.
//!
//! This crate builds upon the work for the [openapiv3 crate](https://crates.io/crates/openapiv3) and adapts it for the AsyncAPI specification.
//! ## Example
//! ```rust
//! use asyncapi::AsyncAPI;
//!
//! # #[cfg(not(feature = "utoipa"))]
//! fn main() {
//!     let data = include_str!("../tests/asyncapi.yml");
//!     let asyncapi: AsyncAPI = serde_yaml::from_str(data).expect("Could not deserialize input");
//!     let serialized = serde_yaml::to_string(&asyncapi).expect("Could not serialize");
//!     println!("{}", serialized);
//! #   assert_eq!(data, serialized);
//! }
//! # #[cfg(feature = "utoipa")]
//! # fn main() {}
//! ```
//!
//! ## Features
//!
//! ### Utoipa
//! This uses Utoipa as internal structs.  
//! Especially useful to generate the Schema from Rust structs and then convert it to AsyncAPI.
//!
//! OpenAPI Schemas have a subset of the Functions from AsyncAPI.  
//! Because of this the conversion from OpenAPI -> AsyncAPI works, but AsyncAPI -> OpenAPI may fail.
//! ```rust
//! # #[cfg(feature = "utoipa")]
//! use utoipa::{openapi, OpenApi, ToSchema};
//! use asyncapi::Components;
//!
//! # #[cfg(feature = "utoipa")]
//! #[derive(ToSchema)]
//! struct UserSignedUp {
//!     email: String,
//!     first_name: String,
//!     last_name: String
//! }
//!
//! # #[cfg(feature = "utoipa")]
//! #[derive(OpenApi)]
//! #[openapi(components(schemas(UserSignedUp)))]
//! pub struct ApiDoc;
//!
//! # #[cfg(feature = "utoipa")]
//! fn main() {
//!     let openapi: openapi::OpenApi = ApiDoc::openapi();
//!     let schemas = openapi.components.expect("Could not generate Schemas").schemas;
//!
//!     let asyncapi = asyncapi::AsyncAPI {
//!         components: Some(Components {
//!             schemas,
//!             ..Default::default()
//!         }),
//!         ..Default::default()
//!     };
//!     let serialized = serde_yaml::to_string(&asyncapi).expect("Could not serialize");
//!     println!("{}", serialized);
//!   # assert_eq!(include_str!("../tests/utoipa-asyncapi.yml"), serialized);
//! }
//! # #[cfg(not(feature = "utoipa"))]
//! # fn main() {}
//! ```
mod api;
mod channel;
pub mod channel_binding;
mod components;
mod correlation_id;
mod example;
mod external_documentation;
#[cfg(not(feature = "utoipa"))]
mod info;
mod message;
pub mod message_binding;
mod message_trait;
pub mod operation_binding;
mod operation_trait;
mod parameter;
#[cfg(not(feature = "utoipa"))]
mod reference;
#[cfg(not(feature = "utoipa"))]
pub mod schema;
#[cfg(feature = "utoipa")]
pub mod schema {
    pub use utoipa::openapi::Schema;
}
mod discriminator;
mod security_scheme;
mod server;
pub mod server_binding;
mod tag;
mod variant_or;

pub use api::AsyncAPI;
pub use channel::{Channel, Operation, OperationMessageType};
pub use channel_binding::ChannelBinding;
pub use components::Components;
pub use correlation_id::CorrelationId;
pub use example::Example;
pub use external_documentation::ExternalDocumentation;
#[cfg(not(feature = "utoipa"))]
pub use info::{Contact, Info, License};
pub use message::Message;
pub use message::Payload;
pub use message_binding::MessageBinding;
pub use message_trait::MessageTrait;
pub use operation_binding::OperationBinding;
pub use operation_trait::OperationTrait;
pub use parameter::Parameter;
#[cfg(not(feature = "utoipa"))]
pub use reference::Ref;
#[cfg(not(feature = "utoipa"))]
pub use reference::RefOr;
pub use schema::Schema;
pub use security_scheme::SecurityScheme;
pub use server::{SecurityRequirement, Server, ServerVariable};
pub use server_binding::ServerBinding;
pub use tag::Tag;
#[cfg(feature = "utoipa")]
pub use utoipa::openapi::Ref;
#[cfg(feature = "utoipa")]
pub use utoipa::openapi::RefOr;
#[cfg(feature = "utoipa")]
pub use utoipa::openapi::{Contact, Info, License};
pub use variant_or::{VariantOrUnknown, VariantOrUnknownOrEmpty};
