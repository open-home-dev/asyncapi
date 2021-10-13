use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    CorrelationId, Example, ExternalDocumentation, MessageBinding, ReferenceOr, Schema, Tag,
};

/// Describes a message received on a given channel and operation.
///
/// # Schema formats table
/// The following table contains a set of values that every implementation MUST support.
///
/// NAME | ALLOWED VALUES | NOTES
/// -----|----------------|--------
/// [AsyncAPI 2.2.0 Schema Object](https://www.asyncapi.com/docs/specifications/v2.2.0#schemaObject) | `application/vnd.aai.asyncapi;version=2.2.0`, `application/vnd.aai.asyncapi+json;version=2.2.0`, `application/vnd.aai.asyncapi+yaml;version=2.2.0` | This is the default when a `schemaFormat` is not provided.
/// [JSON Schema Draft 07](https://json-schema.org/specification-links.html#draft-7) | `application/schema+json;version=draft-07`, `application/schema+yaml;version=draft-07` |
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// Schema definition of the application headers.
    /// Schema MUST be of type "object". It **MUST NOT** define the protocol headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<ReferenceOr<Schema>>,
    /// Definition of the message payload. It can be of any type
    /// but defaults to [Schema object][crate::Schema]. It must match the schema format,
    /// including encoding type - e.g Avro should be inlined as either
    /// a YAML or JSON object NOT a string to be parsed as YAML or JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    /// Definition of the correlation ID used for message tracing or matching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<ReferenceOr<CorrelationId>>,
    /// A string containing the name of the schema
    /// format/language used to define the message payload.
    /// If omitted, implementations should parse the payload as a
    /// [Schema object][crate::Schema].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_format: Option<String>,
    /// The content type to use when encoding/decoding a message's payload.
    /// The value MUST be a specific media type (e.g. application/json).
    /// When omitted, the value MUST be the one specified on the defaultContentType field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// A machine-friendly name for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A human-friendly title for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of what the message is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the message.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A map where the keys describe the name of
    /// the protocol and the values describe protocol-specific definitions for the message.
    pub bindings: Option<ReferenceOr<MessageBinding>>,
    /// An array with examples of valid message objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Example>, // TODO try to parse better
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.1.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Payload {
    Schema(Schema),
    Any(serde_json::Value),
}
