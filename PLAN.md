# Pecking Order — Project Plan

*A 2.5D isometric action game about a lovesick Carolina wren on a quest to find his cockatiel soulmate.*

---

## Concept

You play as a small, scrappy Carolina wren. Through your neighbor's window, you've been admiring Mango — a radiant cockatiel — for weeks. One morning, a moving truck takes her away. You set out across five increasingly dangerous environments to find her.

The game plays like a streamlined Vampire Survivors — auto-peck combat, waves of enemies, light strategic decisions — but with authored levels, a narrative arc, and one core constraint: **you can only carry one tool at a time.** Every pickup is a tradeoff. Every swap is a decision.

### Design Pillars

- **One tool at a time** — the constraint IS the strategy layer
- **Cute but stakes-y** — charming enemies, genuine tension
- **Authored, not infinite** — five handcrafted levels with a story, not a roguelike treadmill
- **Juicy minimalism** — small scope, high polish on what's there

---

## Story

### Opening Cutscene
Morning. Your backyard. You're perched on a fence post. Through the neighbor's window, Mango preens in her cage. A pixel heart floats up. Next frame: a moving truck. Movers carry the cage out. The truck drives away. A single golden feather drifts to the sidewalk.

*"She was the color of a sunset someone forgot to finish."*

### Level 1 — The Backyard (Tutorial)
Mild enemies. Learn movement, pecking, tool pickups. Ends when you reach the fence line.

**After:** You hop down from the fence, look east toward the alley. A rat scurries past. You press on.

*"The alley smelled like ambition and old tuna cans."*

### Level 2 — The Alley
Urban gauntlet. Tighter corridors, more claustrophobic. Stray cats, rats, a raccoon mini-boss. You find one of Mango's feathers caught on a chain-link fence.

**After:** You emerge scraped but intact. A friendly pigeon on a wire has seen the truck. Three blocks east, past the park.

*"'You're either brave or stupid,' he said. I didn't see the difference."*

### Level 3 — The Park
Open space, swarm-heavy. Aggressive geese, territorial mockingbirds, a hawk swooping as an environmental hazard.

**After:** You crest a hill. Below, a cul-de-sac. One house has a sunroom. Inside — a flash of orange and yellow.

*"There she was. Behind glass, again. But this time, so was I."*

### Level 4 — The Suburbs
Sprinklers as traps, lawn roombas as roaming hazards, a dog on a chain with a fixed aggro radius. HOA hedges as level geometry.

**After:** You reach the house. The sunroom door is cracked open. You slip inside.

*"The hardest door to walk through is the last one."*

### Level 5 — The Sunroom
Interior level. Houseplants as cover, a ceiling fan hazard, the family cat as the final boss. Multiple boss phases: stalking, pouncing, stunned, grooming (healing).

### Ending Cutscene
You land on the windowsill. Mango hops to the edge of her cage. You can't get in. She can't get out. But every morning after, you come back. Final frame: the two of you, side by side, separated by glass.

*"Some love stories don't end with freedom. Some end with showing up."*

---

## Gameplay Systems

### Movement
- 8-directional movement at fixed speed
- No physics engine — simple grid-based tile collision
- Smooth interpolation on the visual position, snappy on the logical position

### Combat — Auto-Peck
- The bird automatically pecks nearby enemies within a base range
- Peck has a short cooldown (~0.3s)
- Holding no tool increases peck speed (incentivizes the empty slot)
- All damage flows through a central `DamageEvent`:

```rust
#[derive(Event)]
struct DamageEvent {
    target: Entity,
    amount: u32,
    knockback: Option<Vec2>,
    source: DamageSource,
}
```

### One-Tool-at-a-Time

Press a key near a ground item to pick it up. Your current tool drops where you're standing.

| Tool | Found In | Type | Input | Effect | Consumed? |
|------|----------|------|-------|--------|-----------|
| **Bottlecap Shield** | Backyard, Alley | Passive | Auto on hit | Blocks one hit, flies off nearby (recoverable) | No |
| **Pinecone** | Backyard, Park | Active | Press | Lobbed AoE + knockback | Yes |
| **Chicken Bone** | Alley | Passive | Always on | Extended peck range | No |
| **Dandelion Puff** | Park | Active | Press | Brief float/dodge with invincibility frames | Yes |
| **Rubber Band** | Suburbs | Active | Hold + Release | Charged ranged shot | No (cooldown) |
| **Twist Tie** | Sunroom | Active | Press | Immobilize one enemy briefly | No (cooldown) |
| **Worm** | All levels (rare) | Active | Press | Full health restore | Yes |

**Design notes:**
- The empty slot is always viable (faster peck rate), so dropping a tool is a real option
- The Worm creates the game's best micro-decision: "Do I drop my rubber band to heal?"
- The Bottlecap flying off and landing nearby creates scramble moments mid-combat
- Each level naturally introduces 1–2 new tools through placement; no tutorial popups needed

### Wave System

Each level has a sequence of authored waves defined as data:

```rust
struct Wave {
    groups: Vec<SpawnGroup>,
    delay_after: f32,
}

struct SpawnGroup {
    enemy_type: EnemyType,
    count: usize,
    spawn_zone: SpawnZone,
    delay: f32, // stagger between individual spawns
}

enum SpawnZone {
    Edge(CardinalDirection),
    Point(Vec2),      // specific location (dumpster, burrow)
    Ring(f32),         // radius closing in on player
    Zone(Rect),        // rectangular area
}
```

Waves advance only when all enemies from the current wave are dead. Brief breathing room between waves. Level completes when all waves are cleared.

### Enemies

Behaviors are components, not class hierarchies. Mix and match to create variety:

| Enemy | Levels | Behavior | Health | Notes |
|-------|--------|----------|--------|-------|
| Ant | 1 | Rush (slow) | 1 | Tutorial fodder |
| Garden Snake | 1 | Rush (medium) | 2 | Mini-boss of level 1 |
| Rat | 2 | Rush (fast) | 2 | Spawns from dumpsters |
| Stray Cat | 2 | Circle + Lunge | 5 | Circles then dashes in |
| Raccoon | 2 | Rush (slow) + AoE slam | 10 | Mini-boss, telegraphed attacks |
| Goose | 3 | Rush (fast) + knockback | 3 | Annoying, comes in swarms |
| Mockingbird | 3 | Circle + Ranged (chirp projectile) | 2 | Keeps distance |
| Hawk | 3 | Swoop (environmental) | — | Can't be killed, just dodged |
| Sprinkler | 4 | Environmental hazard | — | Periodic spray in fixed pattern |
| Lawn Roomba | 4 | Patrol (fixed path) | — | Environmental, pushes you on contact |
| Dog | 4 | Rush but tethered (fixed aggro radius) | — | Avoidable if you read the chain length |
| Suburban Squirrel | 4 | Rush (fast) + Knockback | 3 | Territorial |
| House Cat (Boss) | 5 | Multi-phase AI | 25 | See Boss section |

### The House Cat — Final Boss

Four-phase AI cycling based on health thresholds and timers:

1. **Stalking** — Patrols the sunroom. Doesn't aggro until you get close or deal damage. Slow, menacing.
2. **Pounce** — Rushes at high speed in a straight line. If it hits a wall/furniture, transitions to Stunned.
3. **Stunned** — Briefly vulnerable. This is your damage window. Short duration.
4. **Grooming** — Flees to a corner, heals slowly. The indignity of being pecked by a wren. Lasts a few seconds, then returns to Stalking.

The Twist Tie (found in the sunroom) can immobilize the cat during Pounce, creating the Stunned window without needing the wall collision. This teaches the player the tool's purpose through encounter design.

---

## Technical Architecture

### Stack
- **Bevy** (latest stable) — ECS game engine
- **LDtk** + `bevy_ecs_ldtk` — visual level editor and tilemap loading
- **Aseprite** (or AI generation) — sprite art
- **jsfxr** — sound effects
- Rust, obviously

### Project Structure

```
pecking_order/
├── src/
│   ├── main.rs              # App builder, plugin registration
│   ├── states.rs             # GameState enum, LevelSequence resource
│   ├── menu/
│   │   ├── mod.rs            # MenuPlugin
│   │   ├── main_menu.rs      # Title screen UI
│   │   ├── options.rs        # Settings screen
│   │   └── pause.rs          # Pause overlay
│   ├── game/
│   │   ├── mod.rs            # GamePlugin
│   │   ├── player.rs         # Movement, facing, peck attack
│   │   ├── enemies.rs        # Spawn, behavior systems
│   │   ├── tools.rs          # Pickup/swap, per-tool activation + effects
│   │   ├── waves.rs          # WaveManager, wave advancement
│   │   ├── combat.rs         # DamageEvent processing, death, knockback
│   │   ├── camera.rs         # Follow cam + screenshake
│   │   └── level.rs          # LDtk loading, spawn point processing
│   ├── hud/
│   │   ├── mod.rs            # HudPlugin
│   │   ├── health.rs         # Heart display
│   │   ├── tool_display.rs   # Current tool icon
│   │   └── wave_indicator.rs # "Wave 3/5"
│   ├── cutscene/
│   │   ├── mod.rs            # CutscenePlugin
│   │   └── player.rs         # Frame display, typewriter, transitions
│   ├── juice/
│   │   ├── mod.rs            # JuicePlugin
│   │   ├── screenshake.rs    # Trauma-based camera shake
│   │   ├── hit_flash.rs      # White flash on damage
│   │   ├── particles.rs      # Death bursts, impact puffs
│   │   └── bobble.rs         # Ground item float animation
│   ├── audio.rs              # Sound effect triggers, music management
│   └── loading.rs            # Asset loading state
├── assets/
│   ├── sprites/
│   ├── tilemaps/             # LDtk project files
│   ├── cutscenes/            # Full-frame illustrations
│   ├── fonts/
│   └── audio/
└── Cargo.toml
```

### Game States

```rust
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    Cutscene,
    Playing,
    Paused,
}

#[derive(Resource)]
struct CurrentLevel(usize);

#[derive(Resource)]
struct LevelSequence {
    stages: Vec<Stage>,
    current: usize,
}

enum Stage {
    Cutscene(fn() -> Cutscene),
    Level(fn() -> LevelConfig),
}
```

All game systems run gated on `in_state(GameState::Playing)`. All UI is tagged with `StateScoped(...)` for automatic cleanup on state transitions.

### Level Loading via LDtk

- Paint tilemaps visually in the LDtk editor
- Place entity markers for spawn points, tool locations, hazard zones
- `bevy_ecs_ldtk` loads the `.ldtk` project and spawns tile entities
- A `process_spawn_points` system converts LDtk entity markers into game bundles on level enter
- Collision is an IntGrid layer — simple `is_solid(grid_x, grid_y)` lookup, no physics engine

### The Isometric Look

The 2.5D feel comes from art, not engine: tiles drawn in isometric perspective, sprites at a slight angle. The coordinate system is flat 2D. A Y-sorting system sets `Transform.translation.z` based on Y position each frame so entities overlap correctly.

### ECS Patterns

**Tool effects** use an event-driven pattern:
- One system handles activation input → emits `ToolActivated` event
- Per-tool systems listen for their specific tool type and spawn projectiles / apply effects
- Passive tools (Bottlecap, Chicken Bone) modify other systems by querying `HeldTool` directly

**Enemy behaviors** are component-driven:
- `Behavior::Rush`, `Behavior::Circle`, `Behavior::Swoop`, etc.
- One system per behavior type queries enemies with that behavior
- New enemy "types" are just new component combinations — no new code needed

**Damage flow:**
`hit detection system` → `DamageEvent` → `process_damage system` → health reduction, knockback, death marking → `cleanup system` despawns dead entities

**HUD reads game state with zero mutability concerns** — the health display system takes `Query<&Health>` (immutable borrow), the damage system takes `Query<&mut Health>` (mutable borrow). Bevy's scheduler handles ordering automatically. A one-frame display lag is invisible; explicit ordering with `.after()` is available if needed.

---

## Cutscene System

### Data Model

```rust
struct Cutscene {
    frames: Vec<CutsceneFrame>,
    current: usize,
}

struct CutsceneFrame {
    background: AssetPath,
    text: Option<String>,
    portrait: Option<AssetPath>,
    speaker: Option<String>,
    transition: Transition,
}

enum Transition {
    Cut,
    FadeToBlack { duration: f32 },
    FadeThrough { duration: f32 },
}
```

### Behavior
- Full-screen background image + semi-transparent text box at the bottom
- Typewriter text effect (characters appear one at a time with a soft tick)
- First keypress while typing → complete text instantly
- Second keypress → advance to next frame
- Fade-to-black transitions between frames where appropriate
- Cutscene ends → `LevelSequence` advances → next state loads

### Art Pipeline
Cutscene backgrounds are standalone illustrations — the best use case for AI image generation. No tiling, no animation, no consistency requirements between frames. Generate at a fixed resolution (480×270 or 640×360) for a chunky pixel look upscaled to 1080p.

---

## Audio

### Implementation
- Bevy built-in audio for simplicity
- Sound effects as one-shot entities with `PlaybackSettings::DESPAWN`
- Level music as looping entities tagged with `StateScoped(GameState::Playing)` — auto-stops on state change
- Pitch-randomize repeated sounds (±10%) to prevent machine-gun repetition

### Sound Design
- **jsfxr** for retro sound effects: pecks, pops, pickups, death poofs, UI clicks
- **CC0 chiptune** or AI-generated loops for level themes
- Silence before the cat boss entrance — drop the music, let ambient hum carry, then hit the boss theme

### Sound List
| Sound | Trigger | Notes |
|-------|---------|-------|
| Peck | Auto-attack hits | Pitch-randomized |
| Pickup | Tool swap | Bright chirp |
| Drop | Tool dropped | Soft thud |
| Player hit | Taking damage | Short low whump |
| Enemy death | Health reaches 0 | Pop + pitch varies by enemy size |
| Bottlecap tink | Shield blocks | Metallic ping |
| Pinecone impact | AoE lands | Crunch |
| Rubber band snap | Charged shot fires | Twang, pitch scales with charge |
| Worm eat | Heal consumed | Satisfying gulp |
| Wave complete | All enemies dead | Brief ascending chime |
| Level complete | Final wave cleared | Longer fanfare |
| Typewriter tick | Cutscene text | Very quiet, texture only |
| Menu select | Button press | Click |
| Boss roar | Cat phase change | Hiss/yowl |

---

## Juice — Visual Polish

Implement in this priority order (hit flash alone gets 60% of the feel):

### 1. Hit Flash
On damage, set sprite color to HDR white for ~80ms (2–3 frames). Universal language for "that connected."

### 2. Knockback
Enemies slide back when pecked. Player slides back when hit. Exponential friction decay. Communicates force for free.

### 3. Screenshake
Trauma-based system: events add trauma (0.0–1.0), trauma decays over time, camera offset = trauma² × max_offset. Quadratic falloff feels natural. Small trauma (0.1) for pecks, medium (0.3) for boss hits, never go to 1.0 unless narratively justified.

### 4. Death Particles
4–6 tiny colored sprites scatter outward from death position and fade over ~0.4s. Color matches enemy type. Cheap, satisfying, sells every kill.

### 5. Pickup Bobble
Ground items float gently up and down (sine wave, ~3px amplitude). Makes tools feel like they want to be picked up.

---

## Testing Strategy

### Pure Logic Tests (Priority)
Extract game logic into pure functions that take values and return values. Systems become thin wrappers. Test the functions:

- Tool pickup/swap logic
- Damage calculation
- Wave advancement conditions
- Collision grid lookups
- Boss phase transitions
- Knockback decay math

### Headless Bevy Integration Tests
Spin up a Bevy `App` with `MinimalPlugins` (no window, no GPU). Verify:

- Wave spawns correct enemy counts
- Level completion triggers state transitions
- Tool pickup despawns ground item and updates HeldTool
- Enemy death decrements wave counter

### Visual Testing — Agent-Friendly Loop
For agent-assisted development:

1. Create a `dev` feature-gated binary that auto-plays with scripted inputs
2. Dump screenshots at key frames to `test_output/` via `bevy::render::view::screenshot::Screenshot`
3. The coding agent runs `cargo run --features dev`, examines output images, iterates

This avoids embedding vision into the test framework — the agent IS the visual verifier.

### Dev Tools
- `bevy-inspector-egui` behind a `dev` feature flag — live entity inspector, component tweaking at runtime
- Level skip keybinds in dev mode (jump to any level/wave for testing)

---

## Art Pipeline

### Tilesets and Backgrounds
1. Generate reference images establishing the visual style
2. Use reference as style anchor for img2img / inpainting to produce individual tiles
3. Clean up seams manually (or with seamless tiling ControlNet workflows)
4. Slice into tile sheets at chosen tile size (16×16 or 32×32)

### Character and Enemy Sprites
- Lean into a **paper cutout** aesthetic — minimal animation frames, charming rather than cheap
- Bird: side-ish view, 2–3 frame hop cycle, peck animation. ~6–8 frames total.
- Enemies: 2–4 frames each. Idle bob + one action frame.
- Generation works for base poses; animation frames will likely need manual editing
- Consider Slay the Spire's approach: characters mostly wobble, and it works

### Cutscene Illustrations
Best fit for pure AI generation. Standalone images, no tiling or animation. Generate at fixed aspect ratio (480×270 or 640×360). ~12–15 total illustrations needed.

### Consistency Strategy
- Fine-tune a LoRA on your reference images to maintain style coherence across assets
- Tilesets are textures (grass, stone, wood) — generation handles these well
- Characters need more manual cleanup — budget time for this

---

## Build Schedule

### Weekend 1 — Core Loop

**Saturday Morning: Scaffold** ✅
- `cargo init`, add Bevy dependency
- Implement `GameState` enum and state transitions
- Asset loading state with a placeholder font
- Main menu with Play / Quit buttons
- Verify: can navigate from menu to "Playing" state and back

**Saturday Afternoon: The Bird** ✅
- Player entity with movement (8-directional, tile collision)
- Camera follow
- Auto-peck system (find nearest enemy in range, deal damage)
- Single enemy type (Ant) that rushes toward player
- Health on both player and enemies, death/despawn
- Verify: you can move around and peck ants to death

**Saturday Evening: The Tool** ✅
- Ground item spawning with bobble animation
- Pickup/swap system (press E near item)
- Implement one tool fully: Pinecone (lobbed AoE)
- HUD: health hearts + current tool icon
- Added iframes (0.5s invincibility after contact damage)
- Verify: pick up pinecone, throw it, watch ants die in a radius

**Sunday Morning: Waves** ✅
- WaveManager resource, wave data for a test level (5 waves)
- Wave advancement logic (all dead → cooldown → next wave)
- Wave indicator in HUD ("Wave N/M")
- Added Garden Snake enemy type (faster, more health, green)
- lib.rs + integration tests, CI/CD pipeline
- Verify: survive 5 waves of mixed ants and snakes

**Sunday Afternoon: Feel**
- Hit flash on damage
- Knockback on peck and on player hit
- Screenshake (small on peck, medium on player hit)
- Death particles
- Sound effects via jsfxr (peck, hit, death, pickup)
- Verify: the test arena FEELS good with rectangles

**Sunday Evening: Polish + Commit**
- Pause menu (Escape → overlay → Resume/Quit)
- Clean up code, add dev keybinds (god mode, skip wave)
- Write/run pure logic tests
- If time: second tool (Bottlecap Shield)

**Weekend 1 exit criteria:** A single arena where you fight waves of two enemy types, pick up tools, and it feels satisfying to play — even with placeholder rectangles.

### Between Weekends — Art Sprint

- Generate and refine tileset for each of the five environments
- Generate bird and enemy sprites (base poses)
- Hand-edit animation frames (keep it minimal — 2–4 frames per character)
- Generate cutscene illustrations (~12–15 images)
- Create LDtk project, paint all five level tilemaps with collision layers
- Place entity markers in LDtk for spawn points, tool locations, hazards
- Find or generate background music tracks (one per level + boss theme)
- Expand jsfxr sound set (see sound list above)

### Weekend 2 — Content and Story

**Saturday Morning: Levels**
- Integrate `bevy_ecs_ldtk`, load tilemaps from LDtk project
- `process_spawn_points` system converts markers to game entities
- Y-sorting system for isometric depth
- Wire up `LevelSequence` to cycle through all five levels
- Verify: can load and play through backyard level with real tiles

**Saturday Afternoon: All Enemies and Tools**
- Implement remaining enemy behaviors (Circle, Swoop, Patrol, environmental hazards)
- Implement remaining tools (Chicken Bone, Dandelion Puff, Rubber Band, Twist Tie, Worm)
- Wire wave data for all five levels
- Verify: each level is playable with its unique enemies and tools

**Saturday Evening: The Cat**
- Boss AI: four-phase state machine (Stalk → Pounce → Stunned → Groom)
- Boss health bar (different from regular HUD hearts — a bar at the top of the screen)
- Twist Tie interaction with boss (stun during pounce)
- Boss music trigger
- Verify: the sunroom boss fight is completable and tense

**Sunday Morning: Cutscenes**
- Cutscene UI: full-screen image, text box, typewriter effect
- Fade-to-black transitions
- Author all cutscene data (opening, four inter-level, ending)
- Wire cutscenes into LevelSequence
- Verify: full game flow from opening to credits

**Sunday Afternoon: Audio and Polish**
- Background music per level (loop with StateScoped)
- All sound effects wired to events
- Silence-before-boss-entrance moment
- Title screen art
- Options menu (volume sliders at minimum)
- Any remaining juice (pickup bob, enemy-specific particles)

**Sunday Evening: Ship It**
- Full playthrough testing
- Difficulty tuning (wave counts, enemy health, tool placement)
- Build release binary
- Make Eric play it
- Verify: Eric smiles

---

## Open Questions

- **Ending:** Bittersweet (glass between them forever) or hopeful (window left cracked, daily visits)? Lobby for bittersweet — it's more honest and more memorable.
- **Difficulty curve:** Should the worm (healing) appear more or less often in later levels? Less feels harder but more honest to the "you're a small bird" theme.
- **Score system:** Track anything (time, enemies pecked, tools swapped)? Or just the story experience? Leaning toward no score — it's a narrative game that happens to have combat.
- **Post-game:** Any reason to replay? Could add a "hard mode" where tools don't appear and you're peck-only. Low effort, high replay value for the masochistic.
