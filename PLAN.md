# Fix Plan - Priority Order

## Tier 0 - Architecture (BROKEN)

- [x] 1. Fix cosmos/world duplication: removed 5000 lines of duplicate generators from world
- [x] 2. Fix life crate: remove world_types.rs stubs, depend on world crate for real types
- [x] 3. Add integration layer to lib.rs: provide `generate_universe()` end-to-end function using all crates
- [x] 4. Add integration test: universe → galaxy → system → planet → species pipeline

## Tier 1 - Bugs

- [x] 5. Fix life crate: use `climate` parameter in species generation (now drives traits)
- [x] 6. Fix life crate: use `lifespan_years` in history generation (now scales timeline)
- [x] 7. Fix life crate: implement missing 5 SpeciesTraits (Metamorphic, Amphibious, Venomous, LongLived, ShortLived)
- [x] 8. Fix crafting: rename malformed substances (ite→Gypsum, duplicate Slate/Ite resolved)
- [x] 9. Fix crafting: recipe count test (updated to 600+)

## Tier 2 - Code Quality

- [x] 10. Remove commented-out Exotic body type references (dead code)
- [x] 11. Fix gaseous generator stubs (gas cloud, gas belt, brown dwarf - 3 TODOs)
- [x] 12. Fix moon generator TODO (line 1033)
- [x] 13. Fix main.rs to show version + crate info

## Tier 3 - Test Coverage

- [x] 14. Add tests for contents/generator.rs (2300 lines, 0 tests)
- [x] 15. Add tests for celestial_body/world/generator.rs (3700 lines, 0 tests)
- [x] 16. Add tests for crafting graph connectivity
