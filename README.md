# Space Game Typings and Static Data

This repo is currently containing both typings for Rust and Typescript and Static Data.

## Typings

Typings are written in Rust and converted to TypeScript via the [rs_ts](https://github.com/) crate.

In order to use the Rust typings add a dependency to your project and specify the `git = "…"` to this repo.

The typescript typings are included with the Static Data and Deno should be able to use them out of the box.
For Node.js you probably have to copy them over to your project currently.

## Static Data

This repo also contains static data which is validated via the typings and then included in a nginx container.
This container will be publicly hosted but thats not the case yet.

Static Data is stuff that doesnt change while the game is running.
For example the solarsystem map stays the same.

This data might change / grow with updates of the game.
