mod d2;
mod map;
mod logger;
mod json;

use crate::d2::d2_client::D2Client;
use crate::d2::d2_data::get_act;
use crate::json::SeedData;

use clap::Parser;
use log::LevelFilter;
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

    /// Save to path
    #[arg(short, long)]
    json_path: Option<String>,

    /// Increase logging level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
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
        eprintln!("  -j, --json-path <PATH>      Save output to specified path");
        eprintln!("  -v, --verbose               Increase logging level");  
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  d2-map /path/to/d2 --seed 1122334 --difficulty 0 --act 0");
        eprintln!("  d2-map /path/to/d2 --seed 1122334 --difficulty 2");
        std::process::exit(1);
    }

    let game_path = args.game_path.unwrap();

    let json_path = args.json_path.unwrap_or_else(|| "".to_string());
    if !json_path.is_empty() {
        std::fs::create_dir_all(&json_path)?;
        log::info!("JSON output path set to: {}", json_path);
    }

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

        if args.seed.is_some() || args.act.is_some() || args.map.is_some() {
            let seed = args.seed.unwrap_or(0xff00ff00);
            dump_maps(&mut client, seed, args.difficulty, args.act, args.map, json_path);
            return Ok(());
        }
    }

    Ok(())
}




unsafe fn dump_maps(
    client: &mut D2Client,
    seed: u32,
    difficulty: u32,
    act_id: Option<i32>,
    map_id: Option<u32>,
    json_path: String,
) {
    let total_start = Instant::now();
    let mut map_count = 0;
    let mut json_maps = vec![];

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
                    if json_path.is_empty() {
                        println!("\n{}", serde_json::to_string(&map_data).unwrap());
                    } else {
                        json_maps.push(map_data);
                    }
                    
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
        if !json_path.is_empty() {
            let file_path = format!("{}/{}_{}.json", json_path, seed, difficulty);
            
            let json_data = SeedData {
                seed,
                difficulty,
                levels: json_maps,
            };
            
            let json_output = serde_json::to_string_pretty(&json_data).unwrap();
            std::fs::write(&file_path, json_output).unwrap();
            log::info!("Maps saved to {}", file_path);
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
    
}
