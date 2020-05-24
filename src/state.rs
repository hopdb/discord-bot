use hop::{backend::MemoryBackend, Client as HopClient};
use std::{
    env,
    error::Error,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use twilight::http::Client as HttpClient;

pub struct StateRef {
    pub http: HttpClient,
    pub hop: HopClient<MemoryBackend>,
    pub token: String,
}

#[derive(Clone)]
pub struct State(Arc<StateRef>);

impl State {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let token = env::var("DISCORD_TOKEN")?;

        Ok(Self(Arc::new(StateRef {
            http: HttpClient::new(&token),
            hop: HopClient::memory(),
            token,
        })))
    }
}

impl Deref for State {
    type Target = Arc<StateRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
