use miuu_replay::{Replay, ReplayError};
use std::fs::read_dir;

// This simply just reads all replay files from `./replays/{level}/*.replay` and parses them.

fn main() -> Result<(), ReplayError> {
    for level in read_dir("replays").unwrap() {
        let entry = level.unwrap();
        let name = entry.file_name().into_string().unwrap();

        for level_replay in read_dir(entry.path()).unwrap() {
            let l_entry = level_replay.unwrap();
            let l_name = l_entry.file_name().into_string().unwrap();

            let replay = std::fs::read(l_entry.path()).unwrap();

            println!("parsing: {name}/{l_name}");
            Replay::parse(&replay)?.decode_replay_buffer()?;
        }
    }

    Ok(())
}
