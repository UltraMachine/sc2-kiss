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
- [ ] Easy unit actions
- [ ] Batch and filter actions
- [ ] Resource harvesting
- [ ] Unit training
- [ ] Structure placement
- [ ] Micro control
*/

#[cfg(feature = "ids")]
pub mod ids;
