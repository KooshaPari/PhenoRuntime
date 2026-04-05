//! NATS client with JetStream support
//!
//! NATS is a lightweight, high-performance messaging system
//! with JetStream for persistence.

use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use natsio::{
    Client,
    jetstream::{Context, StreamConfig, ConsumerConfig},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// NATS errors
#[derive(Error, Debug)]
pub enum NatsError {
    #[error("connection error: {0}")]
    Connection(String),

    #[error("publish error: {0}")]
    Publish(String),

    #[error("subscribe error: {0}")]
    Subscribe(String),

    #[error("stream error: {0}")]
    Stream(String),

    #[error("not found: {0}")]
    NotFound(String),
}

/// Message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub subject: String,
    pub data: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

/// Stream info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub name: String,
    pub subjects: Vec<String>,
    pub retention: String,
    pub max_messages: i64,
    pub max_bytes: i64,
}

/// NATS client with JetStream
pub struct NatsClient {
    client: Client,
    jetstream: Context,
}

impl NatsClient {
    /// Connect to NATS server
    pub async fn new(url: &str) -> Result<Self, NatsError> {
        info!("Connecting to NATS: {}", url);

        let client = Client::new(url)
            .await
            .map_err(|e| NatsError::Connection(e.to_string()))?;

        let jetstream = client.jetstream();

        info!("NATS connection established");
        Ok(Self { client, jetstream })
    }

    /// Publish message (fire-and-forget)
    pub async fn publish(&self, subject: &str, data: &[u8]) -> Result<(), NatsError> {
        self.client.publish(subject, data)
            .await
            .map_err(|e| NatsError::Publish(e.to_string()))?;

        debug!("Published to {}: {} bytes", subject, data.len());
        Ok(())
    }

    /// Publish with reply-to (request/response)
    pub async fn request(&self, subject: &str, data: &[u8], timeout: std::time::Duration) -> Result<Message, NatsError> {
        let response = self.client.request(subject, data, timeout)
            .await
            .map_err(|e| NatsError::Publish(e.to_string()))?;

        Ok(Message {
            subject: response.subject,
            data: response.payload.to_vec(),
            headers: HashMap::new(),
            timestamp: Utc::now(),
        })
    }

    /// Subscribe to subject
    pub async fn subscribe(&self, subject: &str) -> Result<mpsc::Receiver<Message>, NatsError> {
        let (tx, rx) = mpsc::channel(100);

        let mut subscription = self.client.subscribe(subject)
            .await
            .map_err(|e| NatsError::Subscribe(e.to_string()))?;

        tokio::spawn(async move {
            while let Some(msg) = subscription.next().await {
                let message = Message {
                    subject: msg.subject,
                    data: msg.payload.to_vec(),
                    headers: HashMap::new(),
                    timestamp: Utc::now(),
                };

                if tx.send(message).await.is_err() {
                    break;
                }
            }
        });

        debug!("Subscribed to: {}", subject);
        Ok(rx)
    }

    /// Create JetStream stream
    pub async fn create_stream(&self, name: &str, subjects: Vec<String>) -> Result<StreamInfo, NatsError> {
        let config = StreamConfig {
            name: name.to_string(),
            subjects: subjects.clone(),
            retention: natsio::jetstream::RetentionPolicy::WorkQueue,
            max_messages: -1,
            max_bytes: -1,
            ..Default::default()
        };

        self.jetstream.create_stream(config)
            .await
            .map_err(|e| NatsError::Stream(e.to_string()))?;

        info!("Stream created: {}", name);

        Ok(StreamInfo {
            name: name.to_string(),
            subjects,
            retention: "workqueue".to_string(),
            max_messages: -1,
            max_bytes: -1,
        })
    }

    /// Publish to JetStream stream (durable)
    pub async fn publish_to_stream(&self, stream: &str, subject: &str, data: &[u8]) -> Result<(), NatsError> {
        self.jetstream.publish(subject, data)
            .await
            .map_err(|e| NatsError::Publish(e.to_string()))?;

        debug!("Published to stream {} on {}", stream, subject);
        Ok(())
    }

    /// Pull consumer for JetStream
    pub async fn pull_consumer(&self, stream: &str, consumer: &str, batch: usize) -> Result<Vec<Message>, NatsError> {
        let mut messages = Vec::new();

        let mut pull = self.jetstream.consumer(consumer)
            .pull()
            .max_messages(batch as i64);

        let mut sub = pull.stream(stream)
            .build()
            .await
            .map_err(|e| NatsError::Subscribe(e.to_string()))?;

        while let Some(msg) = sub.next().await {
            if let Ok(msg) = msg {
                messages.push(Message {
                    subject: msg.subject,
                    data: msg.payload.to_vec(),
                    headers: HashMap::new(),
                    timestamp: Utc::now(),
                });
            }
        }

        Ok(messages)
    }

    /// Health check
    pub async fn health(&self) -> bool {
        self.client.ping().await.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = Message {
            subject: "test".to_string(),
            data: b"hello".to_vec(),
            headers: HashMap::new(),
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(msg.subject, parsed.subject);
    }
}
