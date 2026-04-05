# Plan: Fully Independent `cosmos` and `world`

## Goal

Make `cosmos` and `world` truly independent libraries.

That means:

- `cosmos` can be used in another project with no `world`
- `world` can be used in another project with no `cosmos`
- `cosmos` does not depend on `world`
- `world` does not depend on `cosmos`
- no shared tiny crate
- no required root-owned contract
- integration happens through explicit conversion code in whatever application uses both

This is the only model that actually satisfies “each crate works on its own”.

## Core Rule

Do not share required public types between `cosmos` and `world`.

Instead:

- `cosmos` owns its own output types
- `world` owns its own input and output types
- an app that uses both maps one into the other

So the architecture is:

```text
cosmos -> exports external facts
world  -> accepts world simulation inputs
app/root -> converts cosmos facts into world inputs
```

Not:

- shared contract crate
- root-owned required contract
- direct crate dependency

## Ownership Split

### `cosmos` owns

Everything outside the body:

- stars
- stellar age
- stellar size/class/luminosity
- habitable zone / goldilocks region
- orbital mechanics
- orbital points
- distance from star
- eccentricity
- axial tilt
- rotation/day length
- tidal locking
- tidal heating as an external forcing term
- moon count
- ring presence
- system/neighborhood/galaxy/universe generation
- high-level external body classification

`cosmos` should expose those as its own public facts type.

Suggested example:

```rust
pub struct ExternalBodyFacts {
    pub body_id: u32,
    pub star_age_gyr: f32,
    pub star_luminosity: f32,
    pub orbital_distance_au: f64,
    pub in_habitable_zone: bool,
    pub eccentricity: f32,
    pub axial_tilt_deg: f32,
    pub rotation_period_days: f32,
    pub day_length_days: f32,
    pub tidally_locked: bool,
    pub tidal_heating: u32,
    pub moon_count: u32,
    pub has_rings: bool,
    pub body_mass_earth: f64,
    pub body_radius_earth: f64,
    pub density_g_cm3: f32,
    pub gravity_g: f32,
    pub blackbody_temp_k: u32,
}
```

Important:

- this type belongs to `cosmos`
- it must not reference `world`

### `world` owns

Everything inside the body:

- atmosphere
- atmospheric escape
- greenhouse state
- climate regulation
- tidally locked climate state
- winds
- glaciation
- hydrology
- ocean chemistry
- impacts
- subsurface oceans
- geology
- surface materials
- photochemistry
- final planetary detail assembly

`world` should define its own input type.

Suggested example:

```rust
pub struct PlanetSimulationInput {
    pub star_age_gyr: f32,
    pub orbital_distance_au: f64,
    pub in_habitable_zone: bool,
    pub eccentricity: f32,
    pub axial_tilt_deg: f32,
    pub rotation_period_days: f32,
    pub day_length_days: f32,
    pub tidally_locked: bool,
    pub tidal_heating: u32,
    pub moon_count: u32,
    pub has_rings: bool,
    pub body_mass_earth: f64,
    pub body_radius_earth: f64,
    pub density_g_cm3: f32,
    pub gravity_g: f32,
    pub blackbody_temp_k: u32,
}
```

Important:

- this type belongs to `world`
- it must not reference `cosmos`

Then `world` should also own:

- `PlanetaryDetail`
- `PlanetSurfaceMap`
- `LifeLevel` if `world` needs it
- any atmosphere/climate/geology/ocean/etc enums and structs

## What Must Not Exist

These are all wrong for the desired architecture:

- `cosmos::PlanetaryDetail`
- `cosmos::LifeLevel`
- `world` importing `cosmos` types
- `cosmos` importing `world` types
- root-owned required shared handoff types
- any “bridge” API in `world` that takes `cosmos::CelestialBody` or `cosmos::OrbitalPoint`

The app/root may have convenience adapter functions, but those adapters are optional glue, not required shared type ownership.

## Correct Integration Model

If an app uses both crates, it does this explicitly:

```rust
let facts: cosmos::ExternalBodyFacts = cosmos::extract_external_body_facts(...);

let input = world::PlanetSimulationInput {
    star_age_gyr: facts.star_age_gyr,
    orbital_distance_au: facts.orbital_distance_au,
    in_habitable_zone: facts.in_habitable_zone,
    eccentricity: facts.eccentricity,
    axial_tilt_deg: facts.axial_tilt_deg,
    rotation_period_days: facts.rotation_period_days,
    day_length_days: facts.day_length_days,
    tidally_locked: facts.tidally_locked,
    tidal_heating: facts.tidal_heating,
    moon_count: facts.moon_count,
    has_rings: facts.has_rings,
    body_mass_earth: facts.body_mass_earth,
    body_radius_earth: facts.body_radius_earth,
    density_g_cm3: facts.density_g_cm3,
    gravity_g: facts.gravity_g,
    blackbody_temp_k: facts.blackbody_temp_k,
};

let detail = world::planet::generate_planetary_detail(&input, ...);
```

That conversion code can live in:

- the root crate of this repo
- another app crate
- tests
- a game runtime

But it must not define ownership of the two libraries.

## Why This Works

This model gives real independence:

- another project can use `cosmos` only
- another project can use `world` only
- neither crate has hidden compile-time reliance on the other
- no one is forced to adopt a shared contract crate or a root crate

The cost is deliberate duplication at the boundary.

That duplication is acceptable because:

- the crates are independent by design
- adapters are cheap
- the boundary stays explicit instead of magical

## Refactor Phases

### Phase 1: Remove any direct crate dependency

Make sure:

- `cosmos/Cargo.toml` has no `world`
- `world/Cargo.toml` has no `cosmos`

Deliverable:

- crates compile independently

### Phase 2: Remove cross-crate public types

Delete or replace any `world` usage of:

- `cosmos::BodyExternalContext`
- `cosmos::ChemicalComponent`
- `cosmos::LifeLevel`
- `cosmos::MagneticFieldStrength`
- `cosmos::CelestialBodyWorldType`
- `cosmos::TelluricBodyComposition`
- `cosmos::PlanetaryDetail`
- `cosmos::PlanetSurfaceMap`

And delete or replace any `cosmos` usage of `world` types if any exist.

Deliverable:

- each crate’s public API is self-owned

### Phase 3: Create `cosmos`-owned external facts types

Add explicit public types in `cosmos` such as:

- `ExternalStarFacts`
- `ExternalOrbitFacts`
- `ExternalBodyFacts`

These should be plain exported structs with no `world` references.

Deliverable:

- `cosmos` has a stable external-facing boundary for apps

### Phase 4: Create `world`-owned simulation input types

Add explicit public types in `world` such as:

- `PlanetSimulationInput`
- optional `AtmosphereInput`
- optional `ClimateInput`

These should be plain exported structs with no `cosmos` references.

Deliverable:

- `world` has a stable self-owned input boundary

### Phase 5: Make `world` internals use only `world` input types

Refactor:

- `atmosphere`
- `climate`
- `photochemistry`
- `ocean`
- `impacts`
- `subsurface`
- `detail`

So they accept only `world` input types and `world` enums.

Deliverable:

- `world` no longer needs knowledge of any `cosmos` data model

### Phase 6: Move `PlanetaryDetail` fully into `world`

Right now the type ownership is still historically messy.

The final state should be:

- `PlanetaryDetail` lives in `world`
- the related detail structs live in `world`
- `cosmos` stops being the source of truth for internal planet detail

Deliverable:

- `world` fully owns internal planet state

### Phase 7: Shrink `cosmos` to external-only facts

Remove internal-body ownership from `cosmos`:

- no atmosphere/climate/ocean detail ownership
- no `LifeLevel` ownership if it is not truly part of astronomy
- no `PlanetaryDetail` ownership

Deliverable:

- `cosmos` becomes astronomy/system/orbit only

### Phase 8: Add optional adapters in the app/root

If this repo wants convenience glue, add adapter helpers in the root crate:

- `fn to_world_input(facts: &cosmos::ExternalBodyFacts) -> world::PlanetSimulationInput`
- `fn enrich_body(...)`

These are optional convenience APIs.

Important:

- they do not define shared ownership
- they are just app glue

Deliverable:

- the repo can still offer easy combined usage without coupling the crates

## Current Repository Direction

The current repo should move toward this end state:

- `world` owns its own internal planet types and logic
- `life` should consume `world`-owned world/life-facing enums where appropriate
- `cosmos` should emit its own independent facts structs
- the root crate should only provide optional mapping helpers

## Immediate Next Task

The next code step should be:

1. delete the root-owned required contract idea
2. replace it with `world`-owned input structs
3. add `cosmos`-owned external facts structs
4. add explicit conversion code only in the root crate or another integrating layer

Start with:

- replacing [src/planet_context.rs](/home/bresilla/code/game/cosmos/src/planet_context.rs)

It should stop being “the required contract”.

Instead:

- `world` should define its own `PlanetSimulationInput`
- `cosmos` should define its own `ExternalBodyFacts`

Then the root crate can optionally convert one into the other, but neither library should rely on the root to define its public API.
