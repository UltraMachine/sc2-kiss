#![allow(clippy::tabs_in_doc_comments)]

/*!
Stupidly simple Rust framework for Starcraft 2 AI bots

<div class="warning"><b>
	WIP: Not ready for use yet. Just a preview
</b></div>

## Todo:
- [x] Id generation
- [ ] Parse ladder args:
	- `--LadderServer <IpAddr>`
	- `--GamePort <u16>`
	- `--StartPort <u16>`
	- `--OpponentId <str>`
- [ ] Filter units and extract data
- [ ] Work with positions, vectors, distances
	- [x] linalg crate
	- [ ] trait extension for iterators
- [ ] Easy actions
	- [x] Action construction and storage
	- [x] Batch actions with same ability and target
	- [ ] Action making methods for unit
	- [ ] Filter unnecessary actions
- [ ] Resource harvesting
- [ ] Unit training
- [ ] Structure placement
- [ ] Micro control
*/

#[cfg(feature = "ids")]
pub mod ids;

#[cfg(feature = "unit")]
pub mod unit;

#[cfg(all(feature = "act", feature = "unit"))]
pub mod act;

#[cfg(feature = "linalg")]
pub mod linalg;
