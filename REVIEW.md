# Code review — idiomatic Rust + test coverage

Reviewed at `ca2e3e7`. Supersedes the review of 2026-08-23; section numbers are unchanged so that existing issue
cross-references still resolve. Items closed since then are listed in the appendix.

Baseline: 885 LOC across `src/`, 16 tests pass, 12 distinct clippy warnings on default lints (8 in the bin target, 4
more under `--tests`; 3 are auto-fixable).

```
cargo clippy --all-targets --fix
```

Two of the three bugs in the previous review are closed. The traversability rule was rewritten correctly (§0.2). The
crossing fix (§0.1) rejects crossings but also rejects legal placements, so it is reopened as §0.3. Four further
correctness bugs are new to this review: §0.4 – §0.7.

---

## 0. Correctness bugs

### 0.3 Crossing check rejects legal T-junctions — `grid.rs:191`

`do_blocks_intersect` reads the two perpendicular *segments* a crossing wall would occupy and rejects if **either** is
occupied.

```rust
Orientation::Horizontal => {
! ( self.vertical_blocks[p.y][p.x].is_free()
&& self.vertical_blocks[p.y + 1][p.x].is_free())
}
```

Probe (verified 2026-08-28):

| Existing wall | Requested wall | Result  | Correct |
|---------------|----------------|---------|---------|
| `V(3,2)`      | `H(3,3)`       | `false` | `true`  |
| `V(3,4)`      | `H(3,3)`       | `false` | `true`  |
| `H(3,3)`      | `V(3,3)`       | `false` | `false` |

`V(3,2)` spans the vertical grooves of rows 2–3; `H(3,3)` spans the horizontal grooves of columns 3–4. They meet at one
lattice point but share no groove, so both are legal in Quoridor.

Root cause: the two matrices record *segment occupancy*, which does not say where a wall is anchored.
`vertical_blocks[3][3]` occupied is consistent with a wall anchored at `(3,2)` or at `(3,3)`, and only the second
crosses. The information needed for an exact check is not stored.

Anchoring the representation on the intersection fixes overlap and crossing exactly. This replaces the edge-array shape
proposed in §1.1, which has the same defect:

```rust
/// `[y][x]` is the intersection shared by cells (x,y), (x+1,y), (x,y+1), (x+1,y+1).
/// `true` means a wall of that orientation is *centred* there.
pub struct Board {
    horizontal: [[bool; GRID_SIZE - 1]; GRID_SIZE - 1],
    vertical: [[bool; GRID_SIZE - 1]; GRID_SIZE - 1],
}
```

Derived predicates, all exact:

- crosses, horizontal at `(x,y)`: `vertical[y][x]`
- overlaps, horizontal at `(x,y)`: `horizontal[y][x] || (x > 0 && horizontal[y][x-1]) || (x < 7 && horizontal[y][x+1])`
- edge `(x,y)–(x,y+1)` blocked: `(x < 8 && horizontal[y][x]) || (x > 0 && horizontal[y][x-1])`
- edge `(x,y)–(x+1,y)` blocked: `(y < 8 && vertical[y][x]) || (y > 0 && vertical[y-1][x])`

Measured: wall state 3456 B → 128 B, `Copy`, zero heap, `#[derive(Default)]` works, and §3.3's hand-written rollback
disappears.

### 0.4 Every board cell renders as occupied — `tile.rs:55`, `grid.rs:24`

`TileState`'s `#[default]` is `Occupied`. `generate_row` builds each tile as
`Tile { position, ..Tile::default() }`, so all 81 cells start `Occupied` (probe: 81, expected 2), and the two `occupy()`
calls in `Grid::new` are no-ops. `cargo run` prints a solid block:

```
■■■■■■■■■   ×9
```

Fix: `#[default]` on `TileState::Free`. Under §1.1 the field disappears and pawn positions become the single source of
truth, which is the durable fix.

The default is also load-bearing in the other direction: `Grid::new` overwrites all 144 wall tiles with
`TileState::Free` in two index loops purely to undo it. Flipping the default lets both loops go.

### 0.5 Debug `println!` in the reachability path — `grid.rs:160`

```rust
if t.position.y == goal_y {
println!("Found neighbour {:?} for {:?}", t.position, p.position);
return true;
}
```

`can_player_reach_goal` runs twice per `place_block`, so this writes to stdout on every accepted wall and interleaves
with the board render. Delete it.

### 0.6 The start square is never compared against the goal — `grid.rs:137`

The BFS tests `goal_y` only on *neighbours*. A pawn standing on its goal row is reported as having reached it only
because some neighbour also lies on that row. The predicate should open with
`if from.y == goal_row { return true; }`. Required before §0.7 and #6 (win condition), which asks the same question of
the square the pawn is on.

### 0.7 Wall budget cannot be enforced through this signature — `grid.rs:230`

```rust
pub fn place_block(&mut self, p1: &Player, p2: &Player, b: &Block) -> bool
```

Both players are borrowed immutably, so no accepted placement can debit anyone. Probe: 32 walls accepted on one board
against a legal maximum of 20; `Player.blocks` unchanged throughout. Tracked as #7; noted here because the fix is a
signature change, not an added check. `Grid` should not own the budget at all — see §1.7.

### 0.8 Unreachable `continue` would spin forever — `grid.rs:151`

```rust
let curr_pos = tiles_for_exploring.iter().cloned().next().unwrap();
if visited.contains(curr_pos) {
    continue;
}
```

`continue` re-enters the loop without removing `curr_pos` from `tiles_for_exploring`, so the same element is selected
again. The branch is currently dead — the insert is guarded by the same condition, and `curr_pos` is removed before the
next iteration — so this is latent, not a live hang. The stack-and-visited-matrix form in §2.2 removes the shape.

---

## 1. Data modelling

### 1.1 `Tile` fields are derivable or overloaded — `tile.rs:4`

| Field       | Problem                                                                      |
|-------------|------------------------------------------------------------------------------|
| `tile_type` | Always `Playable` in `player_tiles`, always `BlockPosition` in wall matrices |
| `state`     | Means "pawn here" in one array, "wall segment here" in the other             |
| `position`  | Duplicates the array index used to reach it; desync possible                 |

The predicted desync is now in the tree: `grid.rs:544` writes `position: block.position` into the tile at `[y][x + 1]`,
so that tile's `position` reports `x`. Harmless only because nothing reads
`position` out of a wall matrix.

`position` inside the struct is also why `Hash`/`Eq` cover mutable state.

Target shape: the anchor-indexed `Board` in §0.3. Pawns move out of the tile grid and into the players, killing
`TileState`, `TileType` and `Tile` outright.

### 1.3 x/y convention is implicit

The `// x and y are flipped around` comment is gone, but the convention is still carried by discipline. `grid.rs`
indexes `[p.y][p.x]` at 14 sites and mixes `Position::new(x, y)` with
`Position { y, x }` field order in the same file (`grid.rs:41` vs `grid.rs:264`). One accessor, and callers that never
index directly:

```rust
impl Board { fn tile(&self, p: Position) -> Tile { self.tiles[p.y][p.x] } }
```

Stronger: `Row(u8)` / `Col(u8)` newtypes, which make `Position::new(3, 5)` un-writable in the wrong order.

### 1.4 `Player.blocks: [Block; 10]` — `player.rs:20`

240 B of ten identical `Block::default()`. All `Block` fields are dead (clippy: *associated items
`new` and `set_position` are never used*; *fields ... never read*). Replace with
`walls_remaining: u8`. Tracked as #7.

### 1.5 `Uuid` → `String` — `player.rs:25`, `game.rs:18`

`Uuid::new_v4().to_string()`, ×3 per game: 24 B of heap and a hex format per id, where `Uuid` is
`Copy` and 16 B. Store `id: Uuid`; format at the display boundary. `Player` is 312 B, of which the
`String` id and name are two heap pointers.

### 1.6 Naming: `Block` vs `TileType::BlockPosition`

Domain term is *wall*. `Block` → `Wall`, `place_block` → `place_wall`, `do_blocks_intersect` →
`crosses`.

### 1.7 Pawn position is stored twice and never synchronised — `grid.rs:16`, `player.rs:19` (new)

`Grid.player_tiles[y][x].state` and `Player.position` both claim to say where a pawn is. Nothing writes both. They agree
today only because `Grid::new` and `Player::new` compute the same start squares independently, and no move function
exists.

`place_block` takes `&Player, &Player` and trusts the caller to pass players consistent with the grid; `Game` holds
`_p1`, `_p2` and `g` side by side and never calls `place_block`, so the contract is untested. §0.4 has already broken
the tile copy without any test noticing.

One owner. `Board` holds walls; `Game` holds `[Player; 2]` with the positions; reachability takes the position and goal
row as arguments:

```rust
fn can_reach_goal(&self, from: Position, goal_row: usize) -> bool
```

This also drops `Grid`'s dependency on `Player` and `StartingPosition`, making the board a leaf module.

---

## 2. Allocations

### 2.1 12 heap allocations per `Grid` — `grid.rs:16`

```rust
pub player_tiles: Box<[Box<[Tile; GRID_SIZE] >; GRID_SIZE] >,
```

1 outer + 9 inner + 2 wall matrices, all fixed-size `Copy` data; each inner `Box` adds a pointer chase per row.
`size_of::<Grid>()` is 24 B on the stack behind 5400 B of heap. Inline it:
`[[Tile; GRID_SIZE]; GRID_SIZE]` is 1944 B, and §0.3 takes the wall matrices to 128 B.

### 2.2 Two `HashSet<&Position>` per reachability query — `grid.rs:143`

The per-step `Vec` allocation is gone. Remaining: two sets SipHashing 16-B `Position` keys, allocated per call, called
twice per `place_block`, plus `iter().cloned().next().unwrap()` to read one element from an unordered set.

```rust
let mut visited = [[false; GRID_SIZE]; GRID_SIZE];
let mut stack = vec![start];
visited[start.y][start.x] = true;
while let Some(p) = stack.pop() {
if p.y == goal_row { return true; }
for n in self.reachable_neighbours(p) {
if ! std::mem::replace( & mut visited[n.y][n.x], true) {
stack.push(n);
}
}
}
false
```

162 B of stack, no hashing, and it fixes §0.6 and §0.8 in passing.

### 2.3 `Vec<&Tile>` for ≤4 items — `grid.rs:112`

Heap allocation per call, 81 calls per reachability query, two queries per placement. Return
`impl Iterator<Item = Position> + '_` or `ArrayVec<Position, 4>`. Yielding `Position` drops the borrow and decouples
callers from storage layout — required by §1.7's signature.

### 2.5 `vec![]` for test fixtures

`grid.rs:358` (`get_reachable_neighbour_tiles`) → array literal. The `Vec<Block>` fixtures at
`grid.rs:321`, `447` need a heap allocation each but are inside `[TestCase; N]`, so only the inner
`vec!` calls remain.

---

## 3. Control flow

### 3.1 Index loops → `array::from_fn` — `grid.rs:23`, `78`, `95`

`generate_row` allocates, fills with defaults, then overwrites every element. The two wall-matrix loops exist only to
undo `TileState`'s default (§0.4).

```rust
fn generate_row(y: usize) -> [Tile; GRID_SIZE] {
    std::array::from_fn(|x| Tile { position: Position { x, y }, ..Tile::default() })
}
```

### 3.2 Unrolled 9-way constructor — `grid.rs:59`

Nine hand-written `generate_row(n)` calls; breaks when `NUM_TILES` changes.

```rust
let mut player_tiles: [[Tile; GRID_SIZE]; GRID_SIZE] = std::array::from_fn(generate_row);
player_tiles[0][GRID_SIZE / 2].occupy();
player_tiles[GRID_SIZE - 1][GRID_SIZE / 2].occupy();
```

### 3.3 Mutate-then-rollback — `grid.rs:178-251`

`add_block` and `free_block` are a hand-written do/undo pair, each duplicated across two arms that differ only in which
index gets `+1`. `place_block` mutates, re-tests reachability, then reverses itself. With a `Copy` board:

```rust
pub fn place_wall(&mut self, p: Position, o: Orientation) -> Result<(), WallError> {
    let candidate = self.with_wall(p, o)?;
    if !candidate.both_pawns_can_reach_goal(pawns) {
        return Err(WallError::BlocksLastPath);
    }
    *self = candidate;
    Ok(())
}
```

The four `match o` arms across `add_block`, `free_block`, `do_blocks_intersect` and
`is_block_position_free` collapse into one orientation-parameterised helper once §0.3's anchor indexing makes both
matrices the same shape.

### 3.6 `.iter().for_each(|t| assert!(..))` — `grid.rs:371`, `437`, `531`

→ `for t in &tests`. Gives a usable failure line number, and allows `break` / `?`.

### 3.9 `&t.0` / `&t` on values already behind a reference (new)

`grid.rs:241`, `242`, `584` — clippy `needless_borrow`, auto-fixable. `grid.rs:584` is `&t` where `t`
is already `&Block`.

---

## 4. Naming

API Guidelines C-GETTER: no `get_` prefix.

| Current                                 | Idiomatic                                                                               |
|-----------------------------------------|-----------------------------------------------------------------------------------------|
| `Grid::get_reachable_neighbour_tiles`   | `Grid::reachable_neighbours`                                                            |
| `Tile::get_print_char`                  | `impl fmt::Display for Tile`                                                            |
| `Grid::do_blocks_intersect`             | `Board::crosses` — verb phrase reads as a question, and it tests crossing, not overlap  |
| `Game { _id, _p1, _p2, g }`             | `Game { id, players: [Player; 2], board }`                                              |
| `Game::new(n1, n2)`                     | `Game::new(p1_name: &str, p2_name: &str)`                                               |
| `place_block(p1, p2, b)`                | `place_wall(position, orientation)` — see §1.7                                          |
| `t`, `tt`, `g`, `b`, `o`, `p`           | `tile`, `grid`, `wall`, `orientation`, `position`                                       |
| `Game::_id`, `_p1`, `_p2` (`game.rs:9`) | Underscore prefixes silence dead-code warnings; `#[allow(dead_code)]` states the intent |

Test names are stale or wrong:

- `test_get_available_tiles` (`grid.rs:259`) tests `can_player_reach_goal`, not available tiles
- `test_player_white` / `test_player_black` (`player.rs:50`, `57`) — colour was replaced by
  `StartingPosition::{Top, Bottom}`; nothing in the crate has a colour
- `can_players_reach_end` (`grid.rs:424`) asserts nothing — see §8.1
- `do_blocks_cross` and `test_wall_crossing` (`grid.rs:590`, `602`) overlap in scope
- inconsistent `test_` prefix inside `mod tests`; drop it throughout

---

## 5. Traits

### 5.2 Missing derives (C-COMMON-TRAITS)

`Orientation`, `Block`, `StartingPosition`, `Player`, `Grid` and `Game` now carry `Debug` and, where applicable,
`PartialEq`/`Eq`. Remaining: `Grid::new` and `Game::new` take no arguments and there is no
`Default` impl → `clippy::new_without_default` fires as soon as a lib target exists (§7).

### 5.3 `Display` instead of `println!` — `game.rs:29`

`render` prints to stdout, so no rendering test can exist — which is why §0.4 shipped. It also draws only
`player_tiles`, making every placed wall invisible. `impl fmt::Display for Grid` covering both wall matrices, then
`print!("{}", game.board)` and assert on `board.to_string()`.

`Grid::player_tiles` is `pub` (`grid.rs:16`) only so `render` can reach it; `Display` allows making it private. Tracked
as #8.

---

## 6. Error handling

### 6.1 `assert!` as input validation — `grid.rs:113`, `grid.rs:234`

`assert_eq!(true, cond)` is gone from the source. Panicking on out-of-range coordinates remains, and is wrong for an API
that will receive coordinates typed at a TUI. Probe: `place_block` at `(8,8)`
panics rather than returning a rejection.

One `assert_eq!(.., false)` is left in the tests (`grid.rs:649`, clippy `bool_assert_comparison`).

### 6.2 `place_block -> bool` — `grid.rs:230`

`false` conflates "occupied", "crosses", "cuts off a player" and — after §0.7 — "out of walls". The UI has nothing to
display.

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

`Result` is `#[must_use]`; the current `bool` is not, and `grid.rs:438` discards it. If `bool` stays, add `#[must_use]`.
Tracked as #9.

---

## 7. Structure & tooling

- **Binary-only crate.** All logic sits under `src/main.rs`'s module tree, so tests must be in-file
  `#[cfg(test)] mod tests`; no integration tests, no doctests. Split into `src/lib.rs` + a ~10-line
  `src/main.rs`.
- **`ratatui = "0.30.2"` unused.** No `src/` import since `7bc32e4`; `game.rs::render` is still
  `println!`. Compile-time cost for nothing — adopt it or drop it. Tracked as #8.
- **No lint gate, no CI.** `prek.toml` runs `cargo fmt` pre-commit, which is why formatting is clean, but nothing runs
  `clippy` or `cargo test`. Add
  `cargo clippy --all-targets -- -D warnings` and `cargo test` to both `prek.toml` and a CI workflow; the 12 open
  warnings are what an unenforced lint gate accumulates. Consider
  `#![warn(clippy::pedantic)]`.
- **Dead code**, 8 clippy warnings: `Block::new`, `Block::set_position`, all `Block` fields,
  `Player::set_position`, `Tile::{occupy, is_free, free}` (unreached through the bin target),
  `Grid::{free_block, add_block, ...}`, `Orientation::Vertical` never constructed outside tests,
  `Grid.horizontal_blocks`/`vertical_blocks` never read outside `grid`. Most of this is the API that #4 and #5 will
  call; `#[allow(dead_code)]` with a reason beats deleting it, but `Block`'s fields should go now (§1.4).

---

## 8. Tests

16 pass; effective coverage remains lower than the count. The two assertion-free tests named in the previous review now
assert, and the test that pinned §0.2 was inverted.

### 8.1 Tests that assert nothing

| Test                                       | Problem                                                                                            |
|--------------------------------------------|----------------------------------------------------------------------------------------------------|
| `can_players_reach_end` (`grid.rs:424`)    | Places walls, discards every result, asserts nothing. `TestCase.1` is unread (clippy). Cannot fail |
| `test_place_block_illegal` (`grid.rs:277`) | Bare `#[should_panic]`; the loop's second case is unreachable, and `assert!(!success)` never runs  |

Bare `#[should_panic]` passes on *any* panic, including an unrelated index-out-of-bounds. Use
`#[should_panic(expected = "..")]`, or — after §6.1 —
`assert_eq!(grid.place_wall(..), Err(WallError::OutOfBounds { .. }))`.

### 8.2 Tests pinning wrong or over-constrained behaviour

| Test                                                   | Problem                                                                                                                                                                         |
|--------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `test_wall_crossing` (`grid.rs:602`)                   | Asserts only same-intersection pairs, so §0.3's over-rejection passes. Add the T-junction cases from §0.3                                                                       |
| `get_reachable_neighbour_tiles_assert` (`grid.rs:412`) | Asserts neighbour *order*, coupling to the top/bottom/left/right `if` sequence. Compare as a set                                                                                |
| `can_player_reach_goal` (`grid.rs:444`)                | Writes both wall matrices by hand, 30 lines of duplicated `Tile` literals, because `place_block` rejects the fixtures. Bypasses the API under test and hard-codes §1.1's layout |
| `place_block_explicit` (`grid.rs:295`)                 | `struct TestCase(usize, usize, Orientation, bool)` accessed as `.0 .1 .2 .3`. Named fields + a `name: &str` per case; or `rstest` / `test-case`                                 |
| `test_place_blocks_bulk` (`grid.rs:316`)               | `result &= place_block(..)` collapses five placements into one bool, so a wrong rejection is indistinguishable from a wrong acceptance                                          |

Set comparison for the neighbour tests:

```rust
let got: HashSet<Position> = grid.reachable_neighbours(Position::new(0, 0)).collect();
assert_eq!(got, HashSet::from([Position::new(0, 1), Position::new(1, 0)]));
```

### 8.3 Tests to add

| #  | Test                                               | Note                                                                                                                           |
|----|----------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| 1  | Reachability returns `false`                       | Staircase separating a pawn from its goal row; plus the one-wall-short case returning `true`                                   |
| 2  | T-junction walls accepted                          | Bug §0.3. `V(3,2)` then `H(3,3)`; and `V(3,4)` then `H(3,3)`. Currently `false`                                                |
| 3  | Same-intersection crossing rejected                | Covered by `test_wall_crossing`; keep when §0.3 is fixed to prove the fix did not overshoot                                    |
| 4  | Empty board has exactly two occupied cells         | Bug §0.4. `(4,0)` and `(4,8)`; every other cell free                                                                           |
| 5  | Failed placement leaves the board unchanged        | Guards the hand-written undo in §3.3; `assert_eq!(before, after)` once `Board` is `PartialEq`                                  |
| 6  | Exhaustive empty-board placement                   | All `(x,y)` in `0..8` accept both orientations; `x == 8 \|\| y == 8` rejected, not panicking, after §6.2                       |
| 7  | Wall blocking is symmetric                         | Wall between A and B ⇒ `B ∉ neighbours(A)` **and** `A ∉ neighbours(B)`. An asymmetric bug passes all current tests             |
| 8  | Exhaustive neighbour counts                        | All 81 cells: count == `4 - (x==0) - (x==8) - (y==0) - (y==8)`, computed independently                                         |
| 9  | Pawn already on its goal row                       | Bug §0.6. Currently answered via a neighbour, so it passes for the wrong reason; assert with the pawn walled in on three sides |
| 10 | Property tests (`proptest`)                        | See below                                                                                                                      |
| 11 | Wall budget: 11th placement by one player rejected | Bug §0.7 / #7. 32 accepted today                                                                                               |
| 12 | Pawn rules                                         | Not implemented — #4                                                                                                           |
| 13 | Win condition                                      | Not implemented — #6                                                                                                           |
| 14 | Render snapshot                                    | After §5.3; `insta` on `board.to_string()`, with walls drawn. Would have caught §0.4                                           |

Property invariants (#10):

- Any sequence of accepted placements ⇒ both pawns can still reach their goal rows after each.
- A rejected placement never mutates state.
- ≤20 walls accepted in total.
- `neighbours(a).contains(b) == neighbours(b).contains(a)` for all pairs.
- No two accepted walls share an intersection anchor, and no groove is claimed twice.

### 8.4 Test infrastructure

- No `tests/` directory — blocked on the lib/bin split (§7).
- No coverage measurement. `cargo llvm-cov --html` surfaces §8.1 and the untested rejection paths immediately.
- No doctests (no lib target, no `///` docs anywhere in `src/`). `place_wall` needs a doc example pinning the coordinate
  convention (§1.3), and `Board`'s two matrices need the anchor rule from §0.3 written down.

---

## 9. Order of attack

| # | Step                                                                              | Refs                                                 |
|---|-----------------------------------------------------------------------------------|------------------------------------------------------|
| 1 | `TileState::Free` as `#[default]`; delete the debug `println!`                    | §0.4, §0.5 — two lines, both user-visible            |
| 2 | `cargo clippy --all-targets --fix`; add clippy + test to `prek.toml` and CI       | §3.9, §6.1, §7 — stops the backlog regrowing         |
| 3 | Fix the assertion-free tests; add tests §8.3 #2, #4                               | §8.1 — #2 and #4 fail now, and should                |
| 4 | Collapse `Tile` → anchor-indexed `Board`; drop the `Box`es                        | §0.3, §1.1, §2.1 — keystone; §3.1, §3.2, §3.3 follow |
| 5 | Reachability: `(from, goal_row)` args, visited matrix + stack, start-square check | §0.6, §0.8, §1.7, §2.2                               |
| 6 | Neighbours → `impl Iterator<Item = Position>`                                     | §2.3                                                 |
| 7 | `place_wall -> Result<(), WallError>`; move the budget to `Player`                | §6.2, §0.7                                           |
| 8 | `impl Display for Grid` incl. walls; snapshot test                                | §5.3, §8.3 #14                                       |
| 9 | lib/bin split, then property tests                                                | §7, §8.3 #10                                         |

Steps 1–3 are independent of the refactor and each is under an hour. Step 4 is the one that pays for the rest: §0.3
cannot be fixed correctly without it.

---

## Appendix — closed since 2026-08-23

| Ref         | Item                                                              | Resolution                                                                                                                          |
|-------------|-------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| §0.1        | Crossing walls accepted at the same intersection                  | Rejected now via `do_blocks_intersect`; over-rejects, reopened as §0.3                                                              |
| §0.2        | Full-grid connectivity instead of per-pawn goal path              | `is_grid_traversable` replaced by `can_player_reach_goal(&Player)`; pocket seal now legal, and the test that pinned it was inverted |
| §1.2        | `Player::new(name, colour, grid_size)`                            | Parameter dropped; `Player::new(name, StartingPosition)`. Tests no longer build 99-wide boards                                      |
| §2.2 (part) | `Vec` allocated per BFS step; BFS seeded with a fabricated `Tile` | `Vec::from_iter` gone; keyed on `Position`, seeded from `p.position`                                                                |
| §3.4        | Collapsible `if` ×4                                               | `&&`                                                                                                                                |
| §3.5        | `len() > 0`                                                       | `!is_empty()`                                                                                                                       |
| §3.7        | `Vec::from_iter(it)`                                              | Removed with the per-step `Vec`                                                                                                     |
| §4          | `p_x` holding a y                                                 | `p_y`                                                                                                                               |
| §5.1        | Inherent `default()` shadowing `Default`                          | `#[derive(Default)]` on `Tile`, `TileType`, `TileState`, `Block`, `Orientation`                                                     |
| §5.2 (part) | Missing `Debug` / `PartialEq` / `Eq`                              | Derived on `Orientation`, `Block`, `StartingPosition`, `Player`, `Grid`, `Game`                                                     |
| §6.1 (part) | `assert_eq!(true, cond)`                                          | `assert!(cond)`; one `assert_eq!(.., false)` left in tests                                                                          |
| §1.4 (part) | `NUM_BLOCKS` declared below its use site                          | Module top, and used in the struct                                                                                                  |
| §7          | `rustfmt.toml` `unstable_features = true`                         | Now `max_width = 80`; `prek.toml` runs `cargo fmt` pre-commit                                                                       |
| §7          | `Cargo.toml` table ordering                                       | `[dependencies]` first                                                                                                              |
| §8.1        | `test_get_available_tiles` asserted nothing                       | Asserts `can_player_reach_goal` for both players; name is now wrong (§4)                                                            |
| §8.2        | `can_not_place_illegal_blocks` pinned the §0.2 bug                | Inverted to `assert!(res)`; `//todo bug orientation` gone                                                                           |
