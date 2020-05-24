mod state;

use crate::state::State;
use futures::StreamExt;
use std::error::Error;
use twilight::{
    gateway::shard::{Event, EventType, Shard, ShardConfig},
    model::{channel::Message, gateway::GatewayIntents},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = State::new()?;

    let current_user_mention_prefixes = {
        let current_user_id = state.http.current_user().await?.id;

        &[
            format!("<@{}> ", current_user_id),
            format!("<@!{}> ", current_user_id),
        ]
    };

    let mut config = ShardConfig::builder(&state.token);
    config.intents(Some(GatewayIntents::GUILD_MESSAGES));
    let shard = Shard::new(config.build()).await?;

    let mut events = shard.some_events(EventType::MESSAGE_CREATE).await;

    while let Some(event) = events.next().await {
        if let Event::MessageCreate(message_event) = event {
            let message = message_event.0;

            if !current_user_mention_prefixes
                .iter()
                .any(|prefix| message.content.starts_with(prefix))
            {
                continue;
            }

            tokio::spawn(handle_message(state.clone(), message));
        }
    }

    Ok(())
}

async fn handle_message(
    state: State,
    message: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut parts = message.content.splitn(2, ' ').skip(1);

    let command_text = match parts.next() {
        Some(command_text) => command_text,
        None => return Ok(()),
    };

    let output = hop_cli::process(&state.hop, command_text).await?;

    state
        .http
        .create_message(message.channel_id)
        .content(output)
        .await?;

    Ok(())
}
