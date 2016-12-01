use bindings;

// TYPES

/// Native rdkafka client
pub type RDKafka = bindings::rd_kafka_t;

/// Native rdkafka configuration
pub type RDKafkaConf = bindings::rd_kafka_conf_t;

/// Native rdkafka message
pub type RDKafkaMessage = bindings::rd_kafka_message_t;

/// Native rdkafka topic
pub type RDKafkaTopic = bindings::rd_kafka_topic_t;

/// Native rdkafka topic configuration
pub type RDKafkaTopicConf = bindings::rd_kafka_topic_conf_t;

/// Native rdkafka topic partition list
pub type RDKafkaTopicPartitionList = bindings::rd_kafka_topic_partition_list_t;

// ENUMS

/// Client types
pub use bindings::rd_kafka_type_t as RDKafkaType;

/// Configuration result
pub use bindings::rd_kafka_conf_res_t as RDKafkaConfRes;

/// Response error
pub use bindings::rd_kafka_resp_err_t as RDKafkaRespErr;