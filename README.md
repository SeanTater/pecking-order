# Pecking Order

A 2.5D action game about a lovesick Carolina wren on a quest to find his cockatiel soulmate. Built with [Bevy](https://bevyengine.org/) (0.18).

## Gameplay

You play as **Pip**, a small, scrappy Carolina wren. Auto-peck enemies that get close, pick up tools for special abilities, and survive escalating waves. The catch: **you can only carry one tool at a time.** Every pickup is a tradeoff.

### Controls

| Key | Action |
|-----|--------|
| WASD / Arrows | Move |
| E | Pick up / swap tool |
| Space | Use held tool |
| Escape | Back to menu |

### Tools

- **Pinecone** — lobbed AoE projectile that lands as a ground item after detonation (reusable!)

More tools coming soon (Bottlecap Shield, Chicken Bone, Rubber Band, etc).

### Characters

- **Pip** — Carolina wren protagonist
- **Mango** — cockatiel love interest
- **Biscuit** — house cat, final boss
- **Noodle** — garden snake
- **Cashew** — chipmunk
- **Gumbo & Jumbo** — frogs

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
├── lib.rs            # Library root (for integration tests)
├── states.rs         # GameState enum (Loading, MainMenu, Playing, ...)
├── loading.rs        # Asset loading state
├── menu/             # Main menu UI
├── game/
│   ├── mod.rs        # GamePlugin, player/camera/enemy/tool wiring
│   ├── player.rs     # Player movement, walk/peck animation
│   ├── camera.rs     # Smooth camera follow
│   ├── enemy.rs      # Enemy component + rush behavior
│   ├── combat.rs     # Health, auto-peck, damage events, iframes
│   ├── tools.rs      # Ground items, pickup/swap, pinecone projectile
│   ├── waves.rs      # WaveManager, wave data, spawning
│   └── juice.rs      # Hit flash, knockback, screenshake, death particles
└── hud/
    ├── mod.rs        # HudPlugin
    ├── health.rs     # Heart display
    ├── tool_display.rs # Current tool icon
    └── wave_indicator.rs # "Wave N/M" text
assets/
├── pip/              # Wren sprites (standing, walking, pecking, pickup)
├── ants/             # 4 ant variants (plain, leaf, stick, blueberry)
├── noodle/           # Garden snake sprites
├── biscuit/          # Cat boss sprites (stalk, pounce, dazed, heal, yarn)
├── mango/            # Cockatiel walk cycle
├── cashew/           # Chipmunk sprites
├── gumbo/            # Frog sprites
├── items/            # Pinecone, etc
├── worms/            # Worm pickup sprites
└── scenery/          # Flower decorations
```

## License

All rights reserved.
