mod api;
mod channel;
pub mod channel_binding;
mod components;
mod correlation_id;
mod example;
mod external_documentation;
#[cfg(not(feature = "openapi-info"))]
mod info;
mod message;
pub mod message_binding;
mod message_trait;
pub mod operation_binding;
mod operation_trait;
mod parameter;
#[cfg(not(feature = "openapi-schema"))]
mod reference;
#[cfg(not(feature = "openapi-schema"))]
pub mod schema;
#[cfg(feature = "openapi-schema")]
pub mod schema {
    pub use openapiv3::{
        AdditionalProperties, AnySchema, ArrayType, IntegerFormat, IntegerType, NumberFormat,
        NumberType, ObjectType, Schema, SchemaData, SchemaKind, StringFormat, StringType, Type,
    };
}
mod security_scheme;
mod server;
pub mod server_binding;
mod tag;
mod variant_or;
mod discriminator;

pub use api::AsyncAPI;
pub use channel::{Channel, Operation, OperationMessageType};
pub use channel_binding::ChannelBinding;
pub use components::Components;
pub use correlation_id::CorrelationId;
pub use example::Example;
pub use external_documentation::ExternalDocumentation;
#[cfg(not(feature = "openapi-info"))]
pub use info::{Contact, Info, License};
pub use message::Message;
pub use message_binding::MessageBinding;
pub use message_trait::MessageTrait;
#[cfg(feature = "openapi-schema")]
pub use openapiv3::ReferenceOr;
#[cfg(feature = "openapi-info")]
pub use openapiv3::{Contact, Info, License};
pub use operation_binding::OperationBinding;
pub use operation_trait::OperationTrait;
pub use parameter::Parameter;
#[cfg(not(feature = "openapi-schema"))]
pub use reference::ReferenceOr;
pub use schema::Schema;
pub use security_scheme::SecurityScheme;
pub use server::{SecurityRequirement, Server, ServerVariable};
pub use server_binding::ServerBinding;
pub use tag::Tag;
pub use variant_or::{VariantOrUnknown, VariantOrUnknownOrEmpty};
