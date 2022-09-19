# PokemonRNGTools

This is a repository of web based Pokemon RNG tools hosted on https://chatot.pokemonrng.com.

# Features

- Sword & Shield Overworld Generator
- Brilliant Diamond & Shining Pearl RNG Generator

## How do I use these tools?

The tools are hosted on https://chatot.pokemonrng.com under these pages:

- Sword & Shield
  - Overworld Generator: https://chatot.pokemonrng.com/#/swsh
- Brilliant Diamond & Shining Pearl RNG Generator
  - Wild: https://chatot.pokemonrng.com/#/bdsp
  - Stationary: https://chatot.pokemonrng.com/#/bdsp/stationary
  - Underground: https://chatot.pokemonrng.com/#/bdsp/underground
  - TID: https://chatot.pokemonrng.com/#/bdsp/tid

For help with RNG, or to report issues using Chatot, join the Pokemon RNG Discord

[<img src="https://discord.com/api/guilds/285269328469950464/widget.png?style=banner2">](https://www.discord.gg/d8JuAvg)

## Building

- Install node, yarn, and [rust](https://www.rust-lang.org/tools/install).
- Run `wasm-pack build` in the `wasm` directory.
- Run `yarn start` in the main directory.

## Credits

- [CaptureSight](https://github.com/zaksabeast/CaptureSight) for some of the Rust logic
- [PokeFinder](https://github.com/Admiral-Fish/PokeFinder) for the Pokemon generation logic
- Lean and [this site](https://leanny.github.io/bdsp_roamers.html) for the roamer generation logic
- zaksabeast, ezpzstreams, vlad, real.96, Admiral-Fish, Lean, and lincoln for their help with figuring out how the RNG works and help testing the tools
