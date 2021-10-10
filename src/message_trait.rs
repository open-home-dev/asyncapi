use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    message_binding::MessageBinding, CorrelationId, ExternalDocumentation, ReferenceOr, Schema, Tag,
};

/// Describes a trait that MAY be applied to a
/// [Message Object](https://www.asyncapi.com/docs/specifications/v2.1.0#messageObject).
/// This object MAY contain any property from the
/// [Message Object](https://www.asyncapi.com/docs/specifications/v2.1.0#messageObject),
/// except `payload` and `traits`.
///
/// If you're looking to apply traits to an operation, see the
/// [Operation Trait Object}(https://www.asyncapi.com/docs/specifications/v2.1.0#operationTraitObject).
///
/// # Examples
///
/// ```json
/// {
///     "schemaFormat": "application/vnd.apache.avro+json;version=1.9.0",
///     "contentType": "application/json"
/// }
/// ```
///
/// ```yaml
/// schemaFormat: 'application/vnd.apache.avro+yaml;version=1.9.0'
/// contentType: application/json
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageTrait {
    /// Schema definition of the application headers.
    /// Schema MUST be of type "object".
    /// It **MUST NOT** define the protocol headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<ReferenceOr<Schema>>,
    /// Definition of the correlation ID used for message tracing or matching.
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<ReferenceOr<CorrelationId>>,
    /// A string containing the name of the schema format/language used to define
    /// the message payload. If omitted, implementations should parse the payload as a
    /// [Schema object](https://www.asyncapi.com/docs/specifications/v2.1.0#schemaObject).
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_format: Option<String>,
    /// The content type to use when encoding/decoding a message's payload.
    /// The value MUST be a specific media type (e.g. `application/json`).
    /// When omitted, the value MUST be the one specified on the
    /// [defaultContentType](https://www.asyncapi.com/docs/specifications/v2.1.0#defaultContentTypeString) field.
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
    /// A machine-friendly name for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// A human-friendly title for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// A short summary of what the message is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    /// A verbose explanation of the message.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<Tag>,
    /// Additional external documentation for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    external_docs: Option<ExternalDocumentation>,
    /// A map where the keys describe the name of the protocol
    /// and the values describe protocol-specific definitions for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    bindings: Option<ReferenceOr<MessageBinding>>,
    /// An array with examples of valid message objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    examples: IndexMap<String, serde_json::Value>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.1.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
