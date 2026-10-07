# Hollow Warden

A 3D platformer hero against a soulslike boss: jump its sweeps, break its posture, spin-throw it.

**Play in your browser:** https://andrewnakas.github.io/hollow-warden/ · [aimashups.com/play/hollow-warden/](https://aimashups.com/play/hollow-warden/)

Hollow Warden pits a 3D platformer hero, with triple jumps, long jumps, wall kicks, dives and ground pounds, against a soulslike boss in a ruined arena. The Warden telegraphs a sweep you jump over, a leap slam whose shockwave you hop, and a charge you sidestep. Land hits to break its posture, then grab it, spin it around and throw it.

## A clean-room tribute
Hollow Warden is a tribute to [ER Mario (Mario in Elden Ring) by deltarooo](https://github.com/deltarooo/er-mario), one of the most-starred AI game mashups. That project runs on the original games and needs your own copies. This one recreates the mechanics from observed behaviour and published values, with original code and CC0 assets, so it runs in a browser with nothing to install and no game files. It isn't affiliated with the original project or with any publisher. See `docs/specs/` for the behaviour specs.

## Run
```sh
cargo run                      # native
cargo test                     # sims and rules
trunk serve --release          # web build at http://127.0.0.1:8080
```
CI (`.github/workflows/ci.yml`) tests, builds the WebGL2 and WebGPU web builds, publishes them to GitHub Pages, and builds Windows, macOS and Linux releases for `v*` tags. A headless-browser smoke test (`smoke.yml`) loads the published build after every deploy.

Built in Rust on [Bevy](https://bevyengine.org) and Rapier, sharing its engine with [GameMash](https://github.com/andrewnakas/GameMash).

## Asset credits
| Asset | Author / source | License |
|---|---|---|
| Textures: concrete_floor_02, concrete_pavement, asphalt_02, red_brick_03, blue_metal_plate, plywood, dirt | Poly Haven (polyhaven.com) | CC0 |
| Sky HDRI: skate_park | Poly Haven | CC0 |
| Character + animations: Universal Animation Library (UAL1 Standard) | Quaternius | CC0 |
| Human bodies, hair, eyes: Universal Base Characters (outfits painted by tools/paint_outfits.py) | Quaternius | CC0 |
| AK rifle and pistol models | loafbrr (OpenGameArt) | CC0 |
| Gunshot recordings (sks, cz) | Vincent Sevedge (OpenGameArt "gunshot-sounds") | CC-BY 3.0 |
| Reload sounds, explosion, engine loop | OpenGameArt contributors | CC0 |
| Skateboard roll / land / ollie | FOSSarts, Freesound (pack 41196) | CC0 |
| Footsteps and impacts | Kenney (kenney.nl) Impact Sounds | CC0 |
