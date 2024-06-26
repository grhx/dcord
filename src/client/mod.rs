mod event_emitter;

pub use event_emitter::*;

use crate::gateway::{GatewayIntents, Shard};
use crate::utils::*;

use futures::future::BoxFuture;
use std::future::IntoFuture;

/// [`Client`] builder.
pub struct ClientBuilder {
    intents: GatewayIntents,
    token: String,
}
impl ClientBuilder {
    pub(crate) fn _new(token: String, intents: GatewayIntents) -> Self {
        Self { intents, token }
    }

    pub fn new(token: impl AsRef<str>, intents: GatewayIntents) -> Self {
        Self::_new(token.as_ref().to_string(), intents)
    }

    pub fn event_handler<H: EventHandler + 'static>(_event_handler: H) {
        unimplemented!();
    }
}
impl IntoFuture for ClientBuilder {
    type Output = Result<Client, ()>;
    type IntoFuture = BoxFuture<'static, Result<Client, ()>>;

    fn into_future(self) -> Self::IntoFuture {
        // client fields
        let intents = self.intents;
        let token = self.token;

        Box::pin(async move {
            let client = Client {
                intents,
                token,
                connection: None,
                last_heartbeat_sent: None,
                last_heartbeat_acknowledged: false,
            };

            Ok(client)
        })
    }
}

/// Client
#[derive(Debug)]
#[allow(dead_code)]
pub struct Client {
    intents: GatewayIntents,
    token: String,
    connection: Option<Shard>,
    last_heartbeat_sent: Option<Instant>,
    last_heartbeat_acknowledged: bool,
}
impl Client {
    pub fn builder(token: impl AsRef<str>, intents: GatewayIntents) -> ClientBuilder {
        ClientBuilder::new(token, intents)
    }

    /// Login with [`Client`]
    ///
    /// Automatically shards the client and handles all payloads
    /// then send to event handler.
    pub async fn login(&mut self) -> Result<(), ()> {
        self.connection = Some(
            Shard::new(self.token.clone(), self.intents.clone())
                .await
                .unwrap_or_else(|err| panic!("{err:?}")),
        );

        self.connection.as_mut().unwrap().start().await?;

        Ok(())
    }
}

