use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    message_binding::MessageBinding, ChannelBinding, CorrelationId, Message, MessageTrait,
    OperationBinding, OperationTrait, Parameter, ReferenceOr, Schema, SecurityScheme,
    ServerBinding,
};

/// Holds a set of reusable objects for different aspects of the AsyncAPI specification.
/// All objects defined within the components object will have no effect on the API
/// unless they are explicitly referenced from properties outside the components object.
///
/// All the fixed fields declared above are objects that MUST use keys that match the
/// regular expression: `^[a-zA-Z0-9\.\-_]+$`.
///
/// Field Name Examples:
///
/// ```
/// User
/// User_1
/// User_Name
/// user-name
/// my.org.User
/// ```
///
/// # Examples
///
/// ```json
/// {
///     "components": {
///         "schemas": {
///         "Category": {
///             "type": "object",
///             "properties": {
///             "id": {
///                 "type": "integer",
///                 "format": "int64"
///             },
///             "name": {
///                 "type": "string"
///             }
///             }
///         },
///         "Tag": {
///             "type": "object",
///             "properties": {
///             "id": {
///                 "type": "integer",
///                 "format": "int64"
///             },
///             "name": {
///                 "type": "string"
///             }
///             }
///         }
///         },
///         "messages": {
///         "userSignUp": {
///             "summary": "Action to sign a user up.",
///             "description": "Multiline description of what this action does.\nHere you have /// another line.\n",
///             "tags": [
///             {
///                 "name": "user"
///             },
///             {
///                 "name": "signup"
///             }
///             ],
///             "headers": {
///             "type": "object",
///             "properties": {
///                 "applicationInstanceId": {
///                 "description": "Unique identifier for a given instance of the publishing /// application",
///                 "type": "string"
///                 }
///             }
///             },
///             "payload": {
///             "type": "object",
///             "properties": {
///                 "user": {
///                 "$ref": "#/components/schemas/userCreate"
///                 },
///                 "signup": {
///                 "$ref": "#/components/schemas/signup"
///                 }
///             }
///             }
///         }
///         },
///         "parameters": {
///         "userId": {
///             "description": "Id of the user.",
///             "schema": {
///             "type": "string"
///             }
///         }
///         },
///         "correlationIds": {
///         "default": {
///             "description": "Default Correlation ID",
///             "location": "$message.header#/correlationId"
///         }
///         },
///         "messageTraits": {
///         "commonHeaders": {
///             "headers": {
///             "type": "object",
///             "properties": {
///                 "my-app-header": {
///                 "type": "integer",
///                 "minimum": 0,
///                 "maximum": 100
///                 }
///             }
///             }
///         }
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// components:
///   schemas:
///     Category:
///       type: object
///       properties:
///         id:
///           type: integer
///           format: int64
///         name:
///           type: string
///     Tag:
///       type: object
///       properties:
///         id:
///           type: integer
///           format: int64
///         name:
///           type: string
///   messages:
///     userSignUp:
///       summary: Action to sign a user up.
///       description: |
///         Multiline description of what this action does.
///         Here you have another line.
///       tags:
///         - name: user
///         - name: signup
///       headers:
///         type: object
///         properties:
///           applicationInstanceId:
///             description: Unique identifier for a given instance of the publishing application
///             type: string
///       payload:
///         type: object
///         properties:
///           user:
///             $ref: "#/components/schemas/userCreate"
///           signup:
///             $ref: "#/components/schemas/signup"
///   parameters:
///     userId:
///       description: Id of the user.
///       schema:
///         type: string
///   correlationIds:
///     default:
///       description: Default Correlation ID
///       location: $message.header#/correlationId
///   messageTraits:
///     commonHeaders:
///       headers:
///         type: object
///         properties:
///           my-app-header:
///             type: integer
///             minimum: 0
///             maximum: 100
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    /// An object to hold reusable
    /// [Schema Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#schemaObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    schemas: IndexMap<String, ReferenceOr<Schema>>,
    /// An object to hold reusable
    /// [Message Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#messageObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    messages: IndexMap<String, ReferenceOr<Message>>,
    /// An object to hold reusable
    /// [Security Scheme Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#securitySchemeObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    security_schemes: IndexMap<String, ReferenceOr<SecurityScheme>>,
    /// An object to hold reusable
    /// [Parameter Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#parameterObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    parameters: IndexMap<String, ReferenceOr<Parameter>>,
    /// An object to hold reusable
    /// [Correlation ID Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#correlationIdObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    correlation_ids: IndexMap<String, ReferenceOr<CorrelationId>>,
    /// An object to hold reusable
    /// [Operation Trait Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#operationTraitObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    operation_traits: IndexMap<String, ReferenceOr<OperationTrait>>,
    /// An object to hold reusable
    /// [Message Trait Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#messageTraitObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    message_traits: IndexMap<String, ReferenceOr<MessageTrait>>,
    /// An object to hold reusable
    /// [Server Bindings Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#serverBindingsObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    server_bindings: IndexMap<String, ReferenceOr<ServerBinding>>,
    /// An object to hold reusable
    /// [Channel Bindings Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#channelBindingsObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    channel_bindings: IndexMap<String, ReferenceOr<ChannelBinding>>,
    /// An object to hold reusable
    /// [Operation Bindings Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#operationBindingsObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    operation_bindings: IndexMap<String, ReferenceOr<OperationBinding>>,
    /// An object to hold reusable
    /// [Message Bindings Objects](https://www.asyncapi.com/docs/specifications/v2.1.0#messageBindingsObject).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    message_bindings: IndexMap<String, ReferenceOr<MessageBinding>>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.1.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
