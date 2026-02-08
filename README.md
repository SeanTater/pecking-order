# Pecking Order

A 2.5D action game about a lovesick Carolina wren on a quest to find his cockatiel soulmate. Built with [Bevy](https://bevyengine.org/) (0.18).

## Gameplay

You play as a small, scrappy wren. Auto-peck enemies that get close, pick up tools for special abilities, and survive escalating waves. The catch: **you can only carry one tool at a time.** Every pickup is a tradeoff.

### Controls

| Key | Action |
|-----|--------|
| WASD / Arrows | Move |
| E | Pick up / swap tool |
| Space | Use held tool |
| Escape | Back to menu |

### Tools

- **Pinecone** — lobbed AoE projectile, consumed on use

More tools coming soon (Bottlecap Shield, Chicken Bone, Rubber Band, etc).

## Building & Running

```sh
cargo run
```

Requires Rust 2024 edition. Bevy may need system dependencies for graphics — see [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/).

## Testing

```sh
cargo test
```

## Project Structure

```
src/
├── main.rs           # App entry, plugin registration
├── states.rs         # GameState enum (Loading, MainMenu, Playing, ...)
├── loading.rs        # Asset loading state
├── menu/             # Main menu UI
├── game/
│   ├── mod.rs        # GamePlugin, player/camera/enemy/tool wiring
│   ├── player.rs     # Player movement (8-directional)
│   ├── camera.rs     # Smooth camera follow
│   ├── enemy.rs      # Enemy component + rush behavior
│   ├── combat.rs     # Health, auto-peck, damage events, iframes
│   ├── tools.rs      # Ground items, pickup/swap, pinecone projectile
│   └── waves.rs      # WaveManager, wave data, spawning
└── hud/
    ├── mod.rs        # HudPlugin
    ├── health.rs     # Heart display
    ├── tool_display.rs # Current tool icon
    └── wave_indicator.rs # "Wave N/M" text
```

## License

All rights reserved.
