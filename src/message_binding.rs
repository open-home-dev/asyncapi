use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::Schema;

/// Map describing protocol-specific definitions for a message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MessageBinding {
    /// Protocol-specific information for an HTTP message, i.e., a request or a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<HTTPMessageBinding>,
    /// Protocol-specific information for a WebSockets message.
    #[serde(skip_serializing_if = "Option::is_none")]
    ws: Option<WebSocketMessageBinding>,
    /// Protocol-specific information for a Kafka message.
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka: Option<KafkaMessageBinding>,
    /// Protocol-specific information for an AMQP 0-9-1 message.
    #[serde(skip_serializing_if = "Option::is_none")]
    amqp: Option<AMQPMessageBinding>,
    /// Protocol-specific information for an AMQP 1.0 message.
    #[serde(skip_serializing_if = "Option::is_none")]
    qmqp1: Option<AMQP1MessageBinding>,
    /// Protocol-specific information for an MQTT message.
    #[serde(skip_serializing_if = "Option::is_none")]
    mqtt: Option<MQTTMessageBinding>,
    /// Protocol-specific information for an MQTT 5 message.
    #[serde(skip_serializing_if = "Option::is_none")]
    mqtt5: Option<MQTT5MessageBinding>,
    /// Protocol-specific information for a NATS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    nats: Option<NATSMessageBinding>,
    /// Protocol-specific information for a JMS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    jms: Option<JMSMessageBinding>,
    /// Protocol-specific information for an SNS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    sns: Option<SNSMessageBinding>,
    /// Protocol-specific information for an SQS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs: Option<SQSMessageBinding>,
    /// Protocol-specific information for a STOMP message.
    #[serde(skip_serializing_if = "Option::is_none")]
    stomp: Option<STOMPMessageBinding>,
    /// Protocol-specific information for a Redis message.
    #[serde(skip_serializing_if = "Option::is_none")]
    redis: Option<RedisMessageBinding>,
    /// Protocol-specific information for a Mercure message.
    #[serde(skip_serializing_if = "Option::is_none")]
    mercure: Option<MercureMessageBinding>,
    /// Protocol-specific information for an IBM MQ message.
    #[serde(skip_serializing_if = "Option::is_none")]
    ibmmq: Option<IBMMQMessageBinding>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.1.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// This object contains information about the message representation in HTTP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HTTPMessageBinding {
    /// A Schema object containing the definitions for HTTP-specific headers.
    /// This schema MUST be of type object and have a properties key.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    binding_version: Option<String>,
}

/// When using WebSockets, the channel represents the connection.
/// Unlike other protocols that support multiple virtual channels
/// (topics, routing keys, etc.) per connection, WebSockets doesn't
/// support virtual channels or, put it another way, there's only one channel
/// and its characteristics are strongly related to the protocol used for the handshake, i.e., HTTP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessageBinding {
    /// The HTTP method to use when establishing the connection. Its value MUST be either GET or POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,
    /// A Schema object containing the definitions for each query parameter.
    /// This schema MUST be of type object and have a properties key.
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<Schema>,
    /// A Schema object containing the definitions of the HTTP headers to use when establishing the connection.
    /// This schema MUST be of type object and have a properties key.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    binding_version: Option<String>,
}

/// This object contains information about the message representation in Kafka.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KafkaMessageBinding {
    /// The message key.
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    binding_version: Option<String>,
}

/// This object contains information about the message representation in AMQP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQPMessageBinding {
    /// A MIME encoding for the message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    content_encoding: Option<String>,
    /// Application-specific message type.
    #[serde(skip_serializing_if = "Option::is_none")]
    message_type: Option<String>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQP1MessageBinding {}

/// This object contains information about the message representation in MQTT.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTTMessageBinding {
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTT5MessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NATSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JMSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SNSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SQSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct STOMPMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedisMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MercureMessageBinding {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IBMMQMessageBinding {
    #[serde(rename = "type")]
    typ: Option<String>,
}
