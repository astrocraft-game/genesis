# Fix Plan - Priority Order

## Tier 0 - Architecture (BROKEN)

- [ ] 1. Fix cosmos/world duplication: world should only have celestial_body + celestial_disk. Remove star/, orbital_point/, contents/, neighborhood/, utils/, types.rs, display.rs from world. Make cosmos depend on world for body types.
- [ ] 2. Fix life crate: remove world_types.rs stubs, depend on world crate for real types
- [ ] 3. Add integration layer to lib.rs: provide `generate_universe()` end-to-end function using all crates
- [ ] 4. Add integration test: universe → galaxy → system → planet → species pipeline

## Tier 1 - Bugs

- [ ] 5. Fix life crate: use `climate` parameter in species generation (currently unused)
- [ ] 6. Fix life crate: use `lifespan_years` in history generation (currently unused)
- [ ] 7. Fix life crate: implement missing 5 SpeciesTraits (Metamorphic, Amphibious, Venomous, LongLived, ShortLived)
- [ ] 8. Fix crafting: rename malformed substances (ite→Gypsite, Clite→Ite, SerpentineMinite→SerpentineMinite)
- [ ] 9. Fix crafting: recipe count test (asserts 100 but actual is 872)

## Tier 2 - Code Quality

- [ ] 10. Remove 3 commented-out Exotic body type references (dead code)
- [ ] 11. Fix gaseous generator stubs (gas cloud, gas belt, brown dwarf - 3 TODOs)
- [ ] 12. Fix moon generator TODO (line 1033)
- [ ] 13. Fix main.rs to actually demonstrate the library

## Tier 3 - Test Coverage

- [ ] 14. Add tests for contents/generator.rs (2300 lines, 0 tests)
- [ ] 15. Add tests for celestial_body/world/generator.rs (3700 lines, 0 tests)
- [ ] 16. Add tests for crafting graph connectivity
