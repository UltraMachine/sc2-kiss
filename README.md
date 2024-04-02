# sc2-kiss

Stupidly simple Rust framework for Starcraft 2 AI bots

> [!WARNING]
> **WIP: Not ready for use yet. Just a preview**

## Goals

- Simple to use and understand
- Solutions for common problems
	(resorce management, building placement, unit pathing, unit micro control, ...)
- Quickly modifiable and replacable things

## Get help

Read the docs: `cargo doc --open`

Join Starcraft 2 AI Discord: https://discord.gg/Emm5Ztz

DM me in discord: @armageddon1337

## Contribution

*Keep it simple!*
- Simple code is easier to understand and modify
- Compiler is smarter than you. It will very likely optimize most things

Use `cargo fmt` and `cargo clippy` to format and check your code

### Workspace structure

- [`kiss`](kiss) - Main crate of the framework
- [`core`](core) and [`async-core`](async-core) - Crates with basic functionality
	to connect and communicate with SC2 instances
- [`pb`](pb) - Proto definitions and build scripts
