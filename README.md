# sc2-kiss

Stupidly simple Rust framework for Starcraft 2 AI bots

**WIP: Not yet ready for use. Just a preview**

## Goals

- Simple to use and understand
- Solutions for common problems
	(resorce management, building placement, unit pathing, unit micro control, ...)
- Quickly modifiable and replacable things

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
