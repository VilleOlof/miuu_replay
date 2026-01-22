# miuu_replay

A Rust crate to parse and read replay files from [`Marble It Up! Ultra`](https://store.steampowered.com/app/864060/Marble_It_Up_Ultra/).    

## Usage

- [ ] how to use


## File Structure

- [ ] explain how replay files are stored and decoded.  

# TODO
- [ ] Fix so `RewindCurveFitter` and `RewindCurveFitterArray` can actually be used and their data exposed
- [ ] Fix public API functions, `parse_replay` (just MessagePack), `decode_data` (actual replayBuffer data)
- [ ] Fix error handling and remove like all `.unwrap()`  
- [ ] Give some note that `RewindCurve::read_from` uses heavily unsafe ptr casting to `T`  
- [ ] Verify that the data is actually real *(map out a replays position vector3's into a 3d space and see if it looks right)*
- [ ] More tests
- [ ] Benchmark the throughput on how many replays can be parsed and fully decoded per `/s`
- [ ] Improve `Vector2`, `Vector3` & `Quaternion` with QoL trait impls and functions
- [ ] Improve `CircularBuffer` with similar QoL and on parity with all of the functions it has in the games code

test command to process data  
`cargo test -- --nocapture | Out-File log.txt`