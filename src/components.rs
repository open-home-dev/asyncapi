use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    message_binding::MessageBinding, Channel, ChannelBinding, CorrelationId, Message, MessageTrait,
    OperationBinding, OperationTrait, Parameter, RefOr, Schema, SecurityScheme, Server,
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
/// ```yaml
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
    /// [Schema Objects][crate::Schema].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub schemas: BTreeMap<String, RefOr<Schema>>,
    /// An object to hold reusable
    /// [Message Objects][crate::Message].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub messages: BTreeMap<String, RefOr<Message>>,
    /// An object to hold reusable
    /// [Security Scheme Objects][crate::SecurityScheme].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub security_schemes: BTreeMap<String, RefOr<SecurityScheme>>,
    /// An object to hold reusable
    /// [Parameter Objects][crate::Parameter].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, RefOr<Parameter>>,
    /// An object to hold reusable
    /// [Correlation ID Objects][crate::CorrelationId].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub correlation_ids: BTreeMap<String, RefOr<CorrelationId>>,
    /// An object to hold reusable
    /// [Operation Trait Objects][crate::OperationTrait].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub operation_traits: BTreeMap<String, RefOr<OperationTrait>>,
    /// An object to hold reusable
    /// [Message Trait Objects][crate::MessageTrait].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub message_traits: BTreeMap<String, RefOr<MessageTrait>>,
    /// An object to hold reusable [Server Objects][crate::Server].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub servers: BTreeMap<String, RefOr<Server>>,
    /// An object to hold reusable
    /// [Server Bindings Objects][crate::ServerBinding].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub server_bindings: BTreeMap<String, RefOr<ServerBinding>>,
    /// An object to hold reusable [Channel Item Objects][crate::Channel].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub channels: BTreeMap<String, Channel>,
    /// An object to hold reusable
    /// [Channel Bindings Objects][crate::ChannelBinding].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub channel_bindings: BTreeMap<String, RefOr<ChannelBinding>>,
    /// An object to hold reusable
    /// [Operation Bindings Objects][crate::OperationBinding].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub operation_bindings: BTreeMap<String, RefOr<OperationBinding>>,
    /// An object to hold reusable
    /// [Message Bindings Objects][crate::MessageBinding].
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub message_bindings: BTreeMap<String, RefOr<MessageBinding>>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}
