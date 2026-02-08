//! # Pecking Order
//!
//! A 2.5D action game about a Carolina wren on a quest to find his cockatiel soulmate.
//! Built with Bevy 0.18.
//!
//! ## Architecture
//!
//! The game is organized as a set of Bevy plugins:
//!
//! - [`game::GamePlugin`] — core gameplay: player, enemies, combat, tools, waves
//! - [`hud::HudPlugin`] — UI overlays: health hearts, tool slot, wave counter
//! - `menu::MenuPlugin` — main menu
//! - `loading::LoadingPlugin` — asset loading state
//!
//! Game state flows through [`states::GameState`]:
//! `Loading → MainMenu → Playing` (with Escape returning to MainMenu).

pub mod game;
pub mod hud;
pub mod states;
