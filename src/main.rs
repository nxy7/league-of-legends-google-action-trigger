use std::time::Duration;

use lol_game_client_api::{api::{GameClient}, event_listener::EventListener, async_trait::async_trait, model::{Ace, GameEnd, ChampionKill}};
use thiserror::Error;

#[tokio::main]
async fn main() {
    let client = GameClient::new();

    let event_listener = EventManager {game_client: client};

    // Start a task that will check every second for new events
    lol_game_client_api::start_listener(event_listener, Duration::from_secs(1));
    
    // let active_player = client.active_player().await.unwrap();
    // let z = client.player_list().await.unwrap();

    println!("Hello, world!");
}

pub struct EventManager  {
    game_client: GameClient, // Is used to query the team if needed, or other info on the fly
}

#[derive(Error, Debug)]
pub enum EventManagerError {
    // #[error("Game client API error: {}", _0)]
    // GameClientApi(#[from] QueryError),
    #[error("Failed to find active player in player list")]
    PlayerNotFound,
}

#[async_trait]
impl EventListener for EventManager {
    type Error = EventManagerError;
    async fn on_ace(&mut self, event: Ace) -> Result<(), Self::Error> {
        // if self.get_current_team().await? == event.acing_team {
        //     println!("Yahou, we aced")
        // }

        Ok(())
    }

    async fn on_game_end(&mut self, event: GameEnd) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn on_champion_kill(&mut self, event: ChampionKill) -> Result<(), Self::Error> {
        if self.game_client.active_player().await.unwrap().summoner_name == event.killer_name {
            println!("brawo, zabiłeś kogoś gruba świnio");
        }
        Ok(())
    }
}