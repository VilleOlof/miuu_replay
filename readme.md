# miuu_replay

A Rust crate to parse and read replay files from [`Marble It Up! Ultra`](https://store.steampowered.com/app/864060/Marble_It_Up_Ultra/).    

## Usage

```rust
const replay_file = include_bytes!("dummy.replay");

// Parse the basic replay data.
let replay = miuu_replay::Replay::parse(&replay_file)?;
println!("level: {}, player: {}", replay.data.level, replay.data.player);

// Decode the internal `replay_buffer`,  
// This is where all the "visual" data is.  
let mut replay_buffer = replay.decode_replay_buffer()?;
let marble = replay_buffer.get_marble()?;
let positions = marble.position()?;
```

## File format

First the file is serialized with [`MessagePack`](https://msgpack.org/).  
This extracts the most surface level details about a replay file.  
Stuff like: `version`, `updatedAt`, `level`, `player`, `score` and cosmetics.  
Then within this data there is a field called `replayBuffer`.  
This field holds all of the positional, timestamped and moving data.  
All of this data is compressed with DEFLATE,  
then after this data is serialized with [C# BinaryWriter](https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter).  
So we can copy how the game uses the binary writer internally and  
read the same amounts of bytes and interpret those bytes in the same way.  
This allows us to get the more complicated data like `RewindCurveFitter<Vector3>`,  
for all positions in a replay. Those final data points is stored inside  
ring buffers with an identical ring buffer in size that stores the time the data  
occured from the start of the replay.  
I recommend looking at  
`src/rewind_curve_fitter.rs`, `src/rewind_curve_type.rs`,  
`src/rewind_curve.rs` and `src/rewindable.rs`  
for how those are structured
