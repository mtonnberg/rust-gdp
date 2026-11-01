# Changelog

## Unreleased

- Replace `Holds` with direct branded facts such as `NonEmpty<'n>` and
  `Proof<And<NonEmpty<'n>, Ascii<'n>>>`. Use `and`/`split` for composition.
  Persistent refinements retain the specifications in `predicates`.
- Add `Fact` and `PredicateFamily` with exact specification matching for safe
  restoration of branded evidence and custom domain extensions.

- Add `name!` for fresh lexical names, backed by generativity 1.2.1, with linear
  guard interoperation and additional isolation/divergence regression tests.
- Make root `ci.sh` the shared local/CI/Nix build gate, including security,
  dependency policy, formatting, linting, proof erasure, and packaged consumers.

## 0.1.0

Initial release: generative names, erased logical proofs, checked refinements,
binary relation evidence, core/alloc predicate library, and bounded integers.
