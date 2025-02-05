#![allow(clippy::tabs_in_doc_comments)]

/*!
Stupidly simple Rust framework for Starcraft 2 AI bots

<div class="warning"><b>
	WIP: Not ready for use yet. Just a preview
</b></div>

## Todo:
- [ ] Design a proper bot architecture (must be simple, easily scalable and customizable)
- [x] Id generation
- [x] Parse ladder CLI args
- [ ] Add CLI parser for local play
- [ ] Simplify running games
- [x] Debug commands
- [ ] Filter units and extract data
- [ ] Work with positions, vectors, distances
	- [x] linalg crate
	- [ ] functions for common situations (get closest unit, filter units in range, ...)
- [ ] Unit actions
	- [x] Action construction and storage
	- [x] Batch actions with same ability and target
	- [ ] Return action results
	- [ ] Support queuing multiple actions at once for a single unit
- [ ] Map
	- [x] Easy access to map data at any position
	- [ ] Update map data to relevant
	- [ ] Analyze map for more data
- [ ] Structure placement
	- [ ] Expansions
	- [ ] General buildings
	- [ ] Ramps + walls
- [ ] Resource harvesting
- [ ] Unit training
- [ ] Micro control
- [ ] Build order planning
*/

pub use sc2_core;
pub use sc2_prost;

#[cfg(feature = "ids")]
pub mod ids;

#[cfg(feature = "unit")]
pub mod unit;

#[cfg(all(feature = "act"))]
pub mod act;

#[cfg(feature = "chat")]
pub mod chat;

#[cfg(feature = "map")]
pub mod map;

#[cfg(feature = "linalg")]
pub mod linalg;

#[cfg(feature = "game-loop")]
pub mod game_loop;

#[cfg(feature = "debug")]
pub mod debug;

#[cfg(feature = "cli")]
pub mod cli;
