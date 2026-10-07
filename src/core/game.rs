//! What this game is. Its modes, world and time of day are fixed.

use crate::core::modes::Mode;
use crate::core::scene::World;

pub const TITLE: &str = "Hollow Warden";
pub const TAGLINE: &str = "A 3D platformer hero against a soulslike boss: jump its sweeps, break its posture, spin-throw it.";
pub const MODES: &[Mode] = &[Mode::Jump];
pub const WORLD: World = World::Arena;
pub const NIGHT: bool = false;
/// Shown on the controls screen (Tab), after the movement basics.
pub const HELP: &[&str] = &[
    "Space: jump (chain for double/triple).  Ctrl: crouch / ground pound.  Ctrl+Space: long jump or backflip.  F: punch / dive.",
    "Jump the Warden's sweep, hop its shockwave, sidestep its charge. It gets faster below half health.",
    "Punches, dives, ground pounds and head stomps fill its posture. When it kneels, press E to grab, spin and throw it.",
    "Coins around the ring refill one wedge of your eight-wedge meter.",
];

pub fn mask() -> u8 {
    MODES.iter().fold(0, |m, x| m | x.bit())
}
