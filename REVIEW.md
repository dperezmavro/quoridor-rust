# Code review — idiomatic Rust + test coverage

Baseline: 587 LOC, 13 tests pass, 23 clippy warnings on default lints (14 auto-fixable).

```
cargo clippy --fix --all-targets
```

---

## 0. Correctness bugs

### 0.1 Crossing walls accepted — `grid.rs:168`

`place_block` checks only the matrix matching the requested orientation. Horizontal + vertical at the same intersection
both succeed.

Probe: `h=true v_crossing_same_intersection=true` at `(3,3)`.

At `(0,0)` rejected only as a side effect of the connectivity check, not by crossing detection.

### 0.2 `is_grid_traversable` implements the wrong rule — `grid.rs:144`

Requires all 81 cells mutually reachable. Quoridor rule: each pawn retains a path to its own goal row. Sealing an empty
pocket is legal; rejected here.

Probe: 2x2 corner pocket → `placed=false`.

Source of `//todo bug orientation` at `grid.rs:342`; test asserts the buggy behaviour.

Required shape:

```rust
fn player_can_reach_goal(&self, from: Position, goal_row: usize) -> bool
```

Run per pawn. Current fn never reads pawn positions.

---

## 1. Data modelling

### 1.1 `Tile` fields are derivable or overloaded — `tile.rs:4`

| Field       | Problem                                                                       |
|-------------|-------------------------------------------------------------------------------|
| `tile_type` | Always `Playable` in `player_tiles`, always `BlockPosition` in block matrices |
| `state`     | Means "pawn here" in one array, "wall segment here" in the other              |
| `position`  | Duplicates the array index used to reach it; desync possible                  |

`position` in the struct is why `Hash`/`Eq` cover mutable state, making `HashSet<&Tile>` (§2.2) work by accident.

Replacement — walls are edges:

```rust
pub const GRID_SIZE: usize = 9;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Board {
    /// `[y][x]` blocks the edge between `(x, y)` and `(x, y + 1)`.
    horizontal: [[bool; GRID_SIZE]; GRID_SIZE - 1],
    /// `[y][x]` blocks the edge between `(x, y)` and `(x + 1, y)`.
    vertical: [[bool; GRID_SIZE - 1]; GRID_SIZE],
}
```

Measured: wall state 3456 B → 144 B. `Copy`, zero heap, `#[derive(Default)]` works, eliminates rollback (§3.3).

### 1.2 Board size const vs parameter

`Grid` hardcodes `GRID_SIZE`; `Player::new(name, colour, grid_size)` takes it as an argument (`player.rs:21`). Tests
build players on a 99-wide board (`player.rs:48`) that `Grid` cannot represent. Fix: drop the parameter, or
`const N: usize` generic.

### 1.3 `// x and y are flipped around` — `player.rs:32`

`grid.rs` alternates `Position { x, y }` and `Position { y, x }`. Fix: callers never index `[y][x]` directly.

```rust
impl Board { fn tile(&self, p: Position) -> Tile { self.tiles[p.y][p.x] } }
```

Stronger: `Row(u8)` / `Col(u8)` newtypes.

### 1.4 `Player.blocks: [Block; 10]` — `player.rs:15`

10 identical `Block::default()`. All `Block` fields dead (clippy: *fields `orientation`, `position`, `is_placed` never
read*). Replace with `walls_remaining: u8`.

Literal `10` in struct; `NUM_BLOCKS` defined below it at `player.rs:18`. Move const to module top, use it.

### 1.5 `Uuid` → `String` — `player.rs:22`, `game.rs:17`

`Uuid::new_v4().to_string()`, ×3 per game. `Uuid` is `Copy`, 16 B. Store `id: Uuid`; format at display boundary.

### 1.6 Naming: `Block` vs `TileType::BlockPosition`

Domain term is *wall*. `Block` → `Wall`, `place_block` → `place_wall`.

---

## 2. Allocations

### 2.1 12 heap allocations per `Grid` — `grid.rs:15`

```rust
pub player_tiles: Box<[Box<[Tile; GRID_SIZE] >; GRID_SIZE] >,
```

1 outer + 9 inner + 2 block matrices. Fixed-size `Copy` data; inner `Box` adds a pointer chase per row. Use
`[[Tile; GRID_SIZE]; GRID_SIZE]` (1944 B inline).

### 2.2 `Vec` allocated per BFS step — `grid.rs:152`

```rust
while left_to_explore.len() > 0 {
let t_v = Vec::from_iter(left_to_explore.iter().cloned());   // full Vec alloc
let t = t_v[0];                                              // to read one element
```

Plus two `HashSet<&Tile>` SipHashing 24-B structs. Called on every `place_block`.

```rust
let mut visited = [[false; GRID_SIZE]; GRID_SIZE];
let mut stack = vec![start];
visited[start.y][start.x] = true;
while let Some(p) = stack.pop() {
for n in self.reachable_neighbours(p) {
if ! std::mem::replace( & mut visited[n.y][n.x], true) {
stack.push(n);
}
}
}
```

Also `grid.rs:146`: seeds with a fabricated `Tile`, not `player_tiles[0][0]` — different hash entry if `(0,0)` is
occupied. Key on `Position`, or the `[[bool; 9]; 9]` above.

### 2.3 `Vec<&Tile>` for ≤4 items — `grid.rs:111`

Heap alloc per call; 81 calls per traversability check, per placement. Return `impl Iterator<Item = Position> + '_` or
`ArrayVec<Position, 4>`. Returning `Position` drops the borrow and decouples from storage layout.

### 2.4 `.clone()` on `Copy`

`grid.rs:54`, `grid.rs:56`, `grid.rs:261`.

### 2.5 `vec![]` for test fixtures

`grid.rs:250`, `grid.rs:268` → array literal.

---

## 3. Control flow

### 3.1 Index loops → `array::from_fn` — `grid.rs:22`, `77`, `94`

Current `generate_row` allocates, fills with defaults, overwrites every element.

```rust
fn generate_row(y: usize) -> [Tile; GRID_SIZE] {
    std::array::from_fn(|x| Tile { position: Position { x, y }, ..Tile::default() })
}
```

Nests for the block matrices: `from_fn(|y| from_fn(|x| ...))`.

### 3.2 Unrolled 9-way constructor — `grid.rs:58`

Nine hand-written `generate_row(n)` calls; breaks when `GRID_SIZE` changes.

```rust
let mut player_tiles: [[Tile; GRID_SIZE]; GRID_SIZE] = std::array::from_fn(generate_row);
player_tiles[0][GRID_SIZE / 2].occupy();
player_tiles[GRID_SIZE - 1][GRID_SIZE / 2].occupy();
```

### 3.3 Mutate-then-rollback — `grid.rs:175-208`

Hand-written undo paired with hand-written do, duplicated across two arms differing only by which index gets `+1`. With
a `Copy` board:

```rust
pub fn place_wall(&mut self, p: Position, o: Orientation) -> Result<(), WallError> {
    let candidate = self.with_wall(p, o)?;
    if !candidate.all_pawns_can_reach_goal(&self.pawns) {
        return Err(WallError::BlocksLastPath);
    }
    *self = candidate;
    Ok(())
}
```

Arms should share one orientation-parameterised helper.

### 3.4 Collapsible `if` ×4 — `grid.rs:116-139`

`if p.y > 0 { if cond {` → `if p.y > 0 && cond {`

### 3.5 `len() > 0` — `grid.rs:152`

→ `!x.is_empty()`.

### 3.6 `.iter().for_each(|t| assert!(..))` — `grid.rs:259`, `281`

→ `for t in &tests`. Gives usable failure line numbers, allows `break`/`?`.

### 3.7 `Vec::from_iter(it)` — `grid.rs:153`

→ `it.collect()`.

---

## 4. Naming

API Guidelines C-GETTER: no `get_` prefix.

| Current                                   | Idiomatic                                                                                  |
|-------------------------------------------|--------------------------------------------------------------------------------------------|
| `Grid::get_reachable_neighbour_tiles`     | `Grid::reachable_neighbours`                                                               |
| `Tile::get_print_char`                    | `impl fmt::Display for Tile`                                                               |
| `Grid::is_grid_traversable`               | `Grid::is_traversable` (stutter on `Grid`)                                                 |
| `Game { p1, p2, g }`                      | `Game { players: [Player; 2], board }`                                                     |
| `Game::new(n1, n2)`                       | `Game::new(p1_name: &str, p2_name: &str)`                                                  |
| `place_block(p, ..)`                      | `place_wall(position, ..)`                                                                 |
| `t`, `tt`, `t_v`, `g` (`grid.rs:153-160`) | `tile`, `neighbour`, `pending`, `grid`                                                     |
| `p_x` (`player.rs:23`) — holds a **y**    | `start_row`                                                                                |
| `Game::_id` (`game.rs:8`)                 | Underscore-prefixed field to silence dead-code; use `#[allow(dead_code)]` or use the field |

Test names inconsistent (`test_player_white` vs `place_block`). Already in `mod tests` — drop `test_` prefix.

`StartingPosition` vs ecosystem `Color` — low priority; matters if a rendering crate exports `Color`.

---

## 5. Traits

### 5.1 Inherent `default()` shadows `Default` — `tile.rs:22`, `block.rs:25`

`pub fn default() -> Self` as an inherent method. `Tile::default()` compiles; `Tile` does not implement `Default`.
Broken:

```rust
Tile { position: p,..Default::default () }               // no impl
#[derive(Default)]
struct Grid {
    tiles: [[Tile; 9]; 9]
} // no impl
opt.unwrap_or_default()                                  // no impl
```

Already worked around at `grid.rs:24` (`let default = Tile::default();` bound purely for struct-update syntax).

Fix: `impl Default for Tile`, or `#[derive(Default)]` after §1.1.

### 5.2 Missing derives (C-COMMON-TRAITS)

| Type                                                  | Missing                                                                                |
|-------------------------------------------------------|----------------------------------------------------------------------------------------|
| `Orientation` (`block.rs:4`)                          | `Debug, PartialEq, Eq` — cause of the `.clone()` at `grid.rs:261`; blocks `assert_eq!` |
| `StartingPosition`, `Block`, `Grid`, `Game`, `Player` | `Debug` — blocks `assert_eq!`, `dbg!`, `{:?}` in failure output                        |
| `Grid::new`, `Game::new`                              | No `Default` impl → `clippy::new_without_default` once a lib target exists             |

### 5.3 `Display` instead of `println!` — `game.rs:33`

`render` prints to stdout ⇒ untestable ⇒ no rendering test exists. `impl fmt::Display for Grid`, then
`print!("{}", game.grid)` and assert on `grid.to_string()`.

`Grid::player_tiles` is `pub` (`grid.rs:15`) only so `render` can reach it; `Display` allows making it private.

`println!("")` (`game.rs:40`) → `println!()`.

---

## 6. Error handling

### 6.1 `assert_eq!(true, cond)` as input validation — `grid.rs:112`, `grid.rs:173`

Two issues: `assert_eq!(true, x)` → `assert!(x)` (10 clippy hits incl. tests); and panicking is wrong for a public API
that will receive coordinates typed at a TUI.

### 6.2 `place_block -> bool` — `grid.rs:168`

`false` conflates "occupied", "crosses", "cuts off a player". UI has nothing to display.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallError {
    OutOfBounds { position: Position },
    Overlaps,
    Crosses,
    BlocksLastPath,
    NoWallsRemaining,
}

impl std::error::Error for WallError {}
impl fmt::Display for WallError { /* .. */ }

pub fn place_wall(&mut self, p: Position, o: Orientation) -> Result<(), WallError>;
```

Result is `#[must_use]`; current `bool` is not, and `grid.rs:331` discards it. If `bool` stays, add `#[must_use]`.

---

## 7. Structure & tooling

- **Binary-only crate.** All logic under `src/main.rs`'s module tree ⇒ tests must be in-file `#[cfg(test)] mod tests`;
  no integration tests, no doctests. Split into `src/lib.rs` + ~10-line `src/main.rs`.
- **`ratatui = "0.30.2"` unused.** No `src/` import (added in `7bc32e4`); `game.rs::render` is still `println!`.
  Compile-time cost for nothing.
- **`Cargo.toml`**: `[dependencies.uuid]` precedes `[dependencies]`. Conventional:
  ```toml
  [dependencies]
  ratatui = "0.30.2"
  uuid = { version = "1.24.0", features = ["v4"] }
  ```
- **No lint gate, no CI.** Add `cargo clippy --all-targets -- -D warnings`; consider `#![warn(clippy::pedantic)]`.
- **Dead code**: `Block::set_position`, `Player::set_position`, all `Block` fields, `Orientation::Vertical` never
  constructed outside tests. 8 clippy dead-code warnings.
- **`rustfmt.toml`** sets `unstable_features = true` ⇒ requires nightly; on stable `cargo fmt` silently ignores the
  unstable keys.

---

## 8. Tests

13 pass; effective coverage is lower than the count.

### 8.1 Tests that assert nothing

| Test                                                  | Problem                                                                                                                     |
|-------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| `test_get_available_tiles` (`grid.rs:218`)            | No assertions. Constructs a grid, writes a private field, returns. Cannot fail. Never calls `get_reachable_neighbour_tiles` |
| `test_place_block_illegal_horizontal` (`grid.rs:228`) | `assert_eq!(success, false)` unreachable — panic occurs on the preceding line                                               |
| `test_place_block_illegal_vertical` (`grid.rs:237`)   | Same                                                                                                                        |

Bare `#[should_panic]` passes on *any* panic, including an unrelated index-out-of-bounds. Use
`#[should_panic(expected = "..")]`. After §6.1 these become
`assert_eq!(grid.place_wall(..), Err(WallError::OutOfBounds { .. }))`.

### 8.2 Tests pinning wrong or over-constrained behaviour

| Test                                                                   | Problem                                                                                                                                             |
|------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| `can_not_place_illegal_blocks` (`grid.rs:336`)                         | Carries `//todo bug orientation`; asserts `false` for a legal Quoridor placement (§0.2). Pins the bug                                               |
| `get_reachable_neighbour_tiles_assert` (`grid.rs:310`)                 | Asserts neighbour *order*, coupling to the top/bottom/left/right `if` sequence. Compare as a set                                                    |
| `is_grid_traversable` (`grid.rs:322`)                                  | Only `true` cases (empty board, one wall). The `false` branch — the reason the fn exists — is untested                                              |
| `place_block` / `get_reachable_neighbour_tiles` (`grid.rs:248`, `267`) | `struct TestCase(usize, usize, Orientation, bool)` accessed as `.0 .1 .2 .3`. Use named fields + a `name: &str` per case; or `rstest` / `test-case` |
| `player.rs:48`, `55`, `62`                                             | `grid_size: 99` — a board `Grid` cannot produce (§1.2). Tests arithmetic, not the game                                                              |

Set comparison for neighbour tests:

```rust
let got: HashSet<Position> = grid.reachable_neighbours(Position::new(0, 0)).collect();
assert_eq!(got, HashSet::from([Position::new(0, 1), Position::new(1, 0)]));
```

### 8.3 Tests to add

| #  | Test                                         | Note                                                                                                            |
|----|----------------------------------------------|-----------------------------------------------------------------------------------------------------------------|
| 1  | `is_traversable` returns `false`             | Staircase separating a pawn from its goal row; plus the one-wall-short case returning `true`                    |
| 2  | Crossing walls rejected                      | Bug §0.1; e.g. H then V at `(3,3)`. Currently `true`                                                            |
| 3  | Legal corner-pocket sealing accepted         | Bug §0.2; inverse of current `can_not_place_illegal_blocks`                                                     |
| 4  | Failed placement leaves board byte-identical | Guards the hand-written undo in §3.3                                                                            |
| 5  | Exhaustive empty-board placement             | All `(x, y)` in `0..8` accept both orientations; all `x == 8 \|\| y == 8` rejected (not panic) after §6.2       |
| 6  | Wall blocking is symmetric                   | Wall between A and B ⇒ `B ∉ neighbours(A)` **and** `A ∉ neighbours(B)`. Asymmetric bug passes all current tests |
| 7  | Exhaustive neighbour counts                  | All 81 cells: count == `4 - (x==0) - (x==8) - (y==0) - (y==8)`, computed independently                          |
| 8  | Property tests (`proptest`)                  | See below                                                                                                       |
| 9  | Wall budget: 11th placement rejected         | Not modelled today (§1.4)                                                                                       |
| 10 | Pawn rules                                   | Not implemented — see below                                                                                     |
| 11 | Win condition                                | Not implemented                                                                                                 |
| 12 | Render snapshot                              | After §5.3; `insta` on `grid.to_string()`                                                                       |

Property invariants (#8):

- Any sequence of accepted placements ⇒ `is_traversable()` holds after each.
- Rejected placement never mutates state.
- ≤20 walls accepted total.
- `neighbours(a).contains(b) == neighbours(b).contains(a)` for all pairs.

Pawn rules (#10): `get_reachable_neighbour_tiles` ignores `TileState::Occupied` on player tiles ⇒ a pawn can move onto
the opponent. Missing: blocked-by-pawn, straight jump, diagonal jump when a wall backs the opponent, diagonal jump at
board edge.

### 8.4 Test infrastructure

- No `tests/` dir — blocked on the lib/bin split (§7).
- No coverage measurement. `cargo llvm-cov --html` surfaces §8.1 and the untested `false` branch immediately.
- No doctests (no lib target, no `///` docs). `place_wall` needs a doc example for the coordinate convention (§1.3).

---

## 9. Order of attack

| # | Step                                                                                | Refs                                                 |
|---|-------------------------------------------------------------------------------------|------------------------------------------------------|
| 1 | `cargo clippy --fix --all-targets`                                                  | clears 14/23 warnings                                |
| 2 | Fix assertion-free test + unreachable asserts                                       | §8.1                                                 |
| 3 | Inherent `default()` → `impl Default`; add derives                                  | §5.1, §5.2                                           |
| 4 | Collapse `Tile` → bool `Board`; drop `Box`es                                        | §1.1, §2.1 — keystone; §2.2, §3.1, §3.2, §3.3 follow |
| 5 | BFS via `while let Some(p) = stack.pop()`; neighbours → iterator                    | §2.2, §2.3                                           |
| 6 | `place_wall -> Result<(), WallError>`; crossing check; per-player goal reachability | §6.2, §0.1, §0.2                                     |
| 7 | Tests 1–7, then lib/bin split + property tests                                      | §8.3, §7                                             |
