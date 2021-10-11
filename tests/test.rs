use asyncapi::AsyncAPI;

macro_rules! test_deserialize_serialize {
    ($path:stmt, $name:stmt, $fn_name:ident) => {
        paste::item! {
            #[test]
            fn [<test_ $fn_name _deserialize_serialize>]() {
                let asyncapi: AsyncAPI = serde_yaml::from_str(include_str!($path))
                    .expect(&format!("Could not deserialize {}", $name));
                serde_yaml::to_string(&asyncapi)
                    .expect(&format!("Could not serialize {}", $name));
            }
        }
    };
}

test_deserialize_serialize!("../fixtures/anyof.yml", "anyof", anyof);
test_deserialize_serialize!(
    "../fixtures/application-headers.yml",
    "application-headers",
    application_headers
);

test_deserialize_serialize!(
    "../fixtures/correlation-id.yml",
    "correlation-id",
    correlation_id
);

test_deserialize_serialize!(
    "../fixtures/gitter-streaming.yml",
    "gitter-streaming",
    gitter_streaming
);

test_deserialize_serialize!("../fixtures/mercure.yml", "mercure", mercure);

test_deserialize_serialize!("../fixtures/not.yml", "not", not);

test_deserialize_serialize!("../fixtures/oneof.yml", "oneof", oneof);

test_deserialize_serialize!("../fixtures/rpc-client.yml", "rpc-client", rpc_client);

test_deserialize_serialize!("../fixtures/rpc-server.yml", "rpc-server", rpc_server);

test_deserialize_serialize!("../fixtures/simple.yml", "simple", simple);

test_deserialize_serialize!("../fixtures/slack-rtm.yml", "slack-rtm", slack_rtm);

test_deserialize_serialize!(
    "../fixtures/streetlights-kafka.yml",
    "streetlights-kafka",
    streetlights_kafka
);

test_deserialize_serialize!(
    "../fixtures/streetlights-mqtt.yml",
    "streetlights-mqtt",
    streetlights_mqtt
);

test_deserialize_serialize!(
    "../fixtures/websocket-gemini.yml",
    "websocket-gemini",
    websocket_gemini
);
