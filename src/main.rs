mod d2;
mod map;
mod logger;

use crate::d2::d2_client::D2Client;
use crate::d2::d2_data::get_act;

use clap::Parser;
use log::LevelFilter;
use serde_json::json;
use std::io::{self, BufRead};
use std::time::Instant;

use crate::logger::configure_logging;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// D2 Game Path
    #[arg(value_name = "GAME_PATH")]
    game_path: Option<String>,

    /// Map Seed
    #[arg(short, long)]
    seed: Option<u32>,

    /// Game Difficulty (0: Normal, 1: Nightmare, 2: Hell)
    #[arg(short, long, default_value_t = 0)]
    difficulty: u32,

    /// Dump a specific act (0-4)
    #[arg(short, long)]
    act: Option<i32>,

    /// Dump a specific map by ID
    #[arg(short, long)]
    map: Option<u32>,

    /// Increase logging level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn dump_info(seed: u32, difficulty: u32, act_id: Option<i32>, map_id: Option<u32>) {
    let mut info = json!({
        "type": "info",
        "seed": seed,
        "difficulty": difficulty,
    });

    if let Some(act) = act_id {
        if let Some(obj) = info.as_object_mut() {
            obj.insert("act".to_string(), json!(act));
        }
    }

    if let Some(map) = map_id {
        if let Some(obj) = info.as_object_mut() {
            obj.insert("map".to_string(), json!(map));
        }
    }

    println!("\n{}", serde_json::to_string(&info).unwrap());
}

unsafe fn dump_maps(
    client: &mut D2Client,
    seed: u32,
    difficulty: u32,
    act_id: Option<i32>,
    map_id: Option<u32>,
) {
    let total_start = Instant::now();
    let mut map_count = 0;

    if let Some(specific_map) = map_id {
        let start = Instant::now();
        match client.dump_map(seed, difficulty, specific_map) {
            Ok(map_data) => {
                println!("\n{}", serde_json::to_string(&map_data).unwrap());
                map_count += 1;
                let duration = start.elapsed();
                log::debug!(
                    "Map generated: seed={}, difficulty={}, mapId={}, duration={}ms",
                    seed,
                    difficulty,
                    specific_map,
                    duration.as_millis()
                );
            }
            Err(e) => {
                log::warn!("Failed to generate map {}: {}", specific_map, e);
            }
        }
    } else {
        for level_id in 0..200u32 {
            if let Some(act) = act_id {
                if get_act(level_id) != act {
                    continue;
                }
            }

            let start = Instant::now();
            match client.dump_map(seed, difficulty, level_id) {
                Ok(map_data) => {
                    println!("\n{}", serde_json::to_string(&map_data).unwrap());
                    map_count += 1;
                    let duration = start.elapsed();
                    log::debug!(
                        "Map generated: seed={}, difficulty={}, actId={}, mapId={}, duration={}ms",
                        seed,
                        difficulty,
                        get_act(level_id),
                        level_id,
                        duration.as_millis()
                    );
                }
                Err(_) => {
                    // Skip levels that fail to generate
                    continue;
                }
            }
        }
    }

    let total_duration = total_start.elapsed();
    log::info!(
        "Map generation complete: seed={}, difficulty={}, count={}, duration={}ms",
        seed,
        difficulty,
        map_count,
        total_duration.as_millis()
    );
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Set up logging
    let log_level = match args.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    configure_logging(log_level);

    log::info!("d2-map starting, version: {}", VERSION);

    if args.game_path.is_none() {
        eprintln!("Usage: d2-map <GAME_PATH> [OPTIONS]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  -s, --seed <SEED>           Map Seed");
        eprintln!("  -d, --difficulty <DIFF>     Game Difficulty [0: Normal, 1: Nightmare, 2: Hell]");
        eprintln!("  -a, --act <ACT>             Dump a specific act [0-4]");
        eprintln!("  -m, --map <MAP>             Dump a specific Map");
        eprintln!("  -v, --verbose               Increase logging level");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  d2-map /path/to/d2 --seed 1122334 --difficulty 0 --act 0");
        eprintln!("  d2-map /path/to/d2 --seed 1122334 --difficulty 2");
        std::process::exit(1);
    }

    let game_path = args.game_path.unwrap();

    unsafe {
        let mut client = D2Client::new();

        let init_start = Instant::now();
        client.initialize(&game_path)?;
        let init_duration = init_start.elapsed();
        log::info!(
            "Initialization complete, version: {}, duration: {}ms",
            VERSION,
            init_duration.as_millis()
        );

        // If arguments provided, run in batch mode
        if args.seed.is_some() || args.act.is_some() || args.map.is_some() {
            let seed = args.seed.unwrap_or(0xff00ff00);
            dump_maps(&mut client, seed, args.difficulty, args.act, args.map);
            return Ok(());
        }

        // Otherwise, run in interactive mode
        println!("\n{{\"type\":\"init\"}}");

        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buffer = String::new();

        let mut current_seed = 0xff00ff00u32;
        let mut current_difficulty = 0u32;
        let mut current_act: Option<i32> = None;
        let mut current_map: Option<u32> = None;

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let line = buffer.trim();

                    if line == "$exit" {
                        break;
                    } else if line.starts_with("$seed ") {
                        if let Some(seed_str) = line.strip_prefix("$seed ") {
                            if let Ok(seed) = seed_str.parse::<u32>() {
                                current_seed = seed;
                                dump_info(current_seed, current_difficulty, current_act, current_map);
                            }
                        }
                    } else if line.starts_with("$difficulty ") {
                        if let Some(diff_str) = line.strip_prefix("$difficulty ") {
                            if let Ok(diff) = diff_str.parse::<u32>() {
                                current_difficulty = diff;
                                dump_info(current_seed, current_difficulty, current_act, current_map);
                            }
                        }
                    } else if line.starts_with("$act ") {
                        if let Some(act_str) = line.strip_prefix("$act ") {
                            if let Ok(act) = act_str.parse::<i32>() {
                                current_act = Some(act);
                                dump_info(current_seed, current_difficulty, current_act, current_map);
                            }
                        }
                    } else if line == "$map" {
                        dump_maps(
                            &mut client,
                            current_seed,
                            current_difficulty,
                            current_act,
                            current_map,
                        );
                        current_act = None;
                        current_map = None;
                        println!("\n{{\"type\":\"done\"}}");
                    }
                }
                Err(e) => {
                    log::error!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}
