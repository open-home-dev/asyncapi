mod asyncapi;
mod channel;
mod channel_binding;
mod components;
mod correlation_id;
mod discriminator;
mod example;
mod external_documentation;
mod info;
mod message;
mod message_binding;
mod message_trait;
mod operation_binding;
mod operation_trait;
mod parameter;
mod reference;
mod schema;
mod security_scheme;
mod server;
mod server_binding;
mod tag;
mod variant_or;

pub use asyncapi::AsyncAPI;
pub use channel::Channel;
pub use channel_binding::ChannelBinding;
pub use components::Components;
pub use correlation_id::CorrelationId;
pub use discriminator::Discriminator;
pub use example::Example;
pub use external_documentation::ExternalDocumentation;
pub use info::{Contact, Info, License};
pub use message::Message;
pub use message_binding::MessageBinding;
pub use message_trait::MessageTrait;
pub use operation_binding::OperationBinding;
pub use operation_trait::OperationTrait;
pub use parameter::Parameter;
pub use reference::ReferenceOr;
pub use schema::Schema;
pub use security_scheme::SecurityScheme;
pub use server::Server;
pub use server_binding::ServerBinding;
pub use tag::Tag;
pub use variant_or::{VariantOrUnknown, VariantOrUnknownOrEmpty};

#[allow(clippy::trivially_copy_pass_by_ref)] // needs to match signature for use in serde attribute
#[inline]
pub(crate) const fn is_false(v: &bool) -> bool {
    !(*v)
}
