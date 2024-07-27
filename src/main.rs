mod queue;

mod _1s;
mod game_starter;
mod game_watcher;
mod join_queue;
mod leave_region;
mod match_maker;
mod player_join;
mod player_leave;
mod pre_queue_tick;
mod queue_button;

use serde::Deserialize;

use queue::{Platform, Queue};

use _1s::generate_1s;
use game_starter::generate_game_starter;
use game_watcher::generate_game_watcher;
use join_queue::generate_join_queue;
use leave_region::generate_leave_region;
use match_maker::generate_match_maker;
use player_join::generate_player_join;
use player_leave::generate_player_leave;
use pre_queue_tick::generate_pre_queue_tick;
use queue_button::generate_queue_button;

#[derive(Deserialize)]
struct JSONPlatform {
    pub id: u8,
    pub spawn1: String,
    pub spawn2: String,
}

#[derive(Deserialize)]
struct JSONQueue {
    pub id: u8,
    pub platforms: Vec<JSONPlatform>,
}

#[derive(Deserialize)]
struct Config {
    pub queues: Vec<JSONQueue>,
    pub out_dir: String,
}

fn main() {
    let config_path = std::env::args().nth(1).unwrap_or_else(|| "config.json".to_string());
    let config = std::fs::read_to_string(config_path).expect("Unable to read file");
    let config: Config = serde_json::from_str(&config).expect("Unable to parse JSON");

    let mut queues = Vec::new();
    for q in config.queues.iter() {
        let mut platforms = Vec::new();
        for p in q.platforms.iter() {
            platforms.push(Platform {
                _id: p.id,
                spawn1: p.spawn1.clone(),
                spawn2: p.spawn2.clone(),
            });
        }
        queues.push(Queue {
            _id: q.id,
            platforms,
        });
    }

    // check if out_dir exists
    if std::fs::metadata(&config.out_dir).is_err() {
        std::fs::create_dir(&config.out_dir).expect("Unable to create directory");
    }

    generate_1s(&queues, format!("{}/1s.htsl", config.out_dir).as_str());
    generate_game_starter(&queues, format!("{}/GameStarter.htsl", config.out_dir).as_str());
    generate_game_watcher(&queues, format!("{}/GameWatcher.htsl", config.out_dir).as_str());
    generate_join_queue(&queues, format!("{}/JoinQueue.htsl", config.out_dir).as_str());
    generate_leave_region(&queues, config.out_dir.as_str());
    generate_match_maker(&queues, format!("{}/MatchMaker.htsl", config.out_dir).as_str());
    generate_player_join(&queues, format!("{}/PlayerJoin.htsl", config.out_dir).as_str());
    generate_player_leave(&queues, format!("{}/PlayerLeave.htsl", config.out_dir).as_str());
    generate_pre_queue_tick(&queues, format!("{}/PreQueueTick.htsl", config.out_dir).as_str());
    generate_queue_button(&queues, config.out_dir.as_str());
}
