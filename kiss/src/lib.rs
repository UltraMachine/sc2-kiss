#![allow(clippy::tabs_in_doc_comments)]

/*!
Stupidly simple Rust framework for Starcraft 2 AI bots

<div class="warning"><b>
	WIP: Not ready for use yet. Just a preview
</b></div>

## Todo:
- [x] Id generation
- [x] Parse ladder CLI args
- [ ] Add CLI parser for local play
- [ ] Filter units and extract data
- [ ] Work with positions, vectors, distances
	- [x] linalg crate
	- [ ] trait extension for iterators
- [ ] Easy actions
	- [x] Action construction and storage
	- [x] Batch actions with same ability and target
	- [ ] Action making methods for unit?
- [ ] Structure placement
	- [ ] Expansions
	- [ ] General buildings
	- [ ] Ramps + walls
- [ ] Resource harvesting
- [ ] Unit training
- [ ] Micro control
*/

pub use sc2_core;
pub use sc2_prost;

#[cfg(feature = "ids")]
pub mod ids;

#[cfg(feature = "unit")]
pub mod unit;

#[cfg(all(feature = "act", feature = "unit"))]
pub mod act;

#[cfg(feature = "linalg")]
pub mod linalg;

#[cfg(feature = "game-loop")]
pub mod game_loop;

#[cfg(feature = "debug")]
pub mod debug;

#[cfg(feature = "cli")]
pub mod cli;
