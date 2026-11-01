# rust-gdp

Check facts at application boundaries, express requirements in types, and carry
zero-sized evidence through the rest of your program. Inspired by Matt Noonan's
[Ghosts of Departed Proofs](https://github.com/Matt-Noonan/gdp) and its generative
naming technique, and [Tesl's proof contracts](https://github.com/mtonnberg/tesl).
This is a Rust adaptation, not a source-compatible port.

```rust
use rust_gdp::{Refined, predicates::{All, Ascii, NonEmpty, Positive}, uint::U13};

type Name = Refined<String, All<NonEmpty, Ascii>>;
type RetryCount = Refined<U13, Positive>;

fn start(name: Name, retries: RetryCount) {
    // These requirements have already been checked by the caller.
    println!("{}: {} retries", name.get(), retries.get().get());
}

let name = Name::new("worker".to_owned()).unwrap();
let retries = RetryCount::new(U13::new(3).unwrap()).unwrap();
start(name, retries);
```

Use `rust-gdp = "0.1"` after publication, or a local path dependency before then.
The Rust import is `rust_gdp`. There are **no Cargo dependencies**, including dev
dependencies. The crate supports Rust 1.85+, `no_std`, and an optional `alloc`
feature (enabled by default). Disable default features for core-only environments.

## Fresh names and departed proofs

`Refined<T, P>` is a persistent checked value. `with_name` introduces a fresh
invariant lifetime brand, so a proof about one value cannot satisfy a requirement
about another, even when both values have the same Rust type.

```rust
use rust_gdp::{with_name, Named, Holds, Proof, predicates::NonEmpty};

fn send<'n>(message: &Named<'n, &str>, _: Proof<Holds<'n, NonEmpty>>) {
    println!("{}", message.get());
}

with_name("hello", |message| {
    let nonempty = message.check::<NonEmpty>().unwrap();
    send(&message, nonempty);
    // Attach existing evidence to the value; no second predicate evaluation.
    let persistent = message.refine(nonempty);
    assert_eq!(persistent.into_inner(), "hello");
});
```

The callback is higher-ranked (`for<'n>`); a name and its proof cannot escape.
An ordinary result or a `Refined` value can escape. Names have no mutable accessor
and cannot be cloned. Moving the value out consumes its name. Refinements have
no unchecked constructors, mutable access, or Clone implementation: arbitrary
user-defined Clone implementations need not preserve a predicate.

`check` returns optional positive evidence. `decide` returns positive evidence in
`Ok` and checked negative evidence in `Err`. `Named::relate` checks `Equal`,
`Less`, `LessEqual`, or a custom `Relation` against another name; evidence records
both brands and their order.

`Proof` supports conjunction introduction/elimination, disjunction introduction,
implication identity/projections/composition/application, and disjunction
elimination using proofs that both alternatives imply the same conclusion. These
are erased structural rules, not runtime closures. `both` and `components`
connect same-value predicate conjunctions to separate checks. An erased
`Proof<Or<A,B>>` does not reveal a branch; use a runtime `Result` when you need to
choose runtime behavior. There is no public axiom/assume/proof-forging API.

## Collection facts and existential packages

`Refined<Vec<T>, Each<P>>` and `Refined<&[T], Each<P>>` establish a property for
all elements. `.iter()` and `.element(index)` yield `CheckedRef<T,P>` without
rechecking. An out-of-bounds lookup returns `None`. Owned vectors also offer
`.into_elements()` and `.map_checked(...)`; the latter accepts a callback from a
checked input to a checked output and carries output evidence through collection
without a second validation pass. `collections::filter_checked` checks each input
once and returns only passing elements with `Each<P>` evidence. Normal vector
allocation/iteration costs apply. `Refined::as_checked` borrows any checked value.

`Named::pack_with` combines two values with their existing `Related` proof into a
`RelatedPair<A,B,R>`. The package may leave the naming scopes;
`.with_names(...)` unpacks with fresh names and restores the relation without
checking again. This transports an existential relationship while preventing the
old scoped names from escaping.

`transform::trim` and `transform::sort` establish `Trimmed` and `Sorted` by
construction, without adding a validation scan after the transformation.

### Tesl correspondence

| Tesl proof feature | Rust API |
| --- | --- |
| `fact` and trusted `check` | Marker type implementing `Predicate<T>` or `Relation<A,B>` |
| Attached unary fact | `Refined<T,P>` |
| Detached fact with subject identity | `Proof<Holds<'n,P>>` plus `Named<'n,T>` |
| Fact composition and decomposition | `both`, `components`, `and`, `split` |
| Multi-subject facts | `Relation` and `Proof<Related<'a,'b,R>>` |
| `ForAll P` and element propagation | `Each<P>`, checked iterators and mapping |
| Existential witness transport | `Refined` and `RelatedPair::with_names` |
| Proof-returning transformations | `transform` module |
| Erased normal proof plumbing | Zero-sized tokens and transparent wrappers |

Rust does not provide a library hook for Tesl's automatic fact inference,
whole-language shadowing rules, privileged declaration syntax, or implicit
annotation propagation. Here signatures explicitly require evidence and the
higher-ranked API controls naming. Types represent constant-indexed facts using
ordinary marker generics and const generics. Higher-arity checks can name an owned
tuple and use a custom `Predicate` on that tuple; there is no automatic n-ary
subject unification or arbitrary theorem synthesis. These are deliberate Rust
library boundaries rather than claims of full language equivalence.

## Standard predicates

| Values | Predicates |
| --- | --- |
| `str`, `String`, slices, `Vec` | `NonEmpty`, `Length<N>`, `MinLength<N>`, `MaxLength<N>` |
| Text | `Ascii`, `Trimmed` |
| Byte slices | `Ascii`, `Utf8` |
| All primitive integers | `Positive`, `NonZero`, `Even`, `Odd`, `InRange<LOW,HIGH>` |
| `f32`, `f64` | `Positive`, `NonZero`, `Finite` |
| `Option<T>` | `IsSome`, `IsNone` |
| `Result<T,E>` | `IsOk`, `IsErr` |
| Slices, `Vec` | `Each<P>`, `Exists<P>`, `Sorted` |
| Arbitrary specifications | `All<P,Q>`, `Any<P,Q>`, `Not<P>` |
| `U13`, `UInt<BITS>` | Integer predicates above |

Shared references forward the standard predicates. Borrow an array as a slice.
Text length is UTF-8 **byte length**, not character count. `Trimmed` allows internal
whitespace and empty text; combine with `NonEmpty` when required. `Sorted` is
nondecreasing, allows duplicates, and rejects incomparable adjacent pairs.
`Each` is true for empty collections; `Exists` is false. Integer ranges are
inclusive and nonnegative; reversed bounds reject everything. Floating `Positive`
allows positive infinity; use `All<Positive, Finite>` to reject it. `NonZero`
rejects NaN and both signed zeros.

`U13` stores 0..=8191 in a transparent `u16`; `UInt<BITS>` supports widths 1..=128
in a transparent `u128`. Invalid widths, out-of-range inputs, and checked arithmetic
overflows return `None`; no truncation or implicit wrapping occurs. Width-dependent
operators deliberately require explicit checked arithmetic. Rust has no primitive
`u13` type; `U13` supplies it as a library type.

## Costs and trust model

* `Proof<P>` occupies zero bytes. `Named<T>` and `Refined<T,P>` have the same layout
  as `T`. Proof composition, moves, getters, attaching existing evidence, and
  restoring named evidence require no runtime checks or allocations.
* Establishing a new fact performs the predicate/relation check. Unknown input
  cannot acquire truthful evidence for free. Checks that scan strings/collections
  cost the corresponding scan; a `Result`/`Option` branch carries runtime state.
* The library never allocates on its own. `String`/`Vec` retain their own costs.
  Optimized code-generation tests compare representative proof paths to a raw
  identity function. Debug builds need not inline away every function call.
* The entire library forbids unsafe code. Proofs never authorize unchecked memory
  operations. Downstream code must not use a custom predicate as an unsafe-code
  precondition without separately auditing its assumptions.
* Custom `Predicate` and `Relation` implementations are trusted specifications.
  They must be pure, stable, and express the property their names promise.
  Interior mutability, changing globals, or dishonest comparison implementations
  can invalidate logical interpretations. Evidence records what was checked;
  it cannot freeze a `Cell`, `RefCell`, atomic, external system, or aliased state.
  Built-in structural predicates such as `NonEmpty` on an owned `String` avoid
  this issue. Element predicates and user comparison implementations inherit
  their implementors' specification obligations.
* This is not a theorem prover, SMT solver, dependent language, or a proof that
  arbitrary application code is correct. Review the small implementations of
  custom specifications and the code consuming raw `.get()` values. Types make
  validated preconditions visible; they do not verify a function's body.

To add a domain fact, implement `Predicate<T>` for a marker type, then use it in
`Refined` or `Named::check`. To transform a refinement, consume it with `try_map`:
old evidence is discarded and the new predicate is checked. A failing constructor
or transformation returns the rejected value unchanged.

## Development: Nix is the only prerequisite

With a flake-capable Nix installation:

```sh
nix --extra-experimental-features 'nix-command flakes' develop
bash scripts/verify.sh
# Or build the checked, publishable .crate artifact directly:
nix --extra-experimental-features 'nix-command flakes' build
# result/share/rust-gdp/rust-gdp-0.1.0.crate
```

`flake.lock` pins the entire development toolchain and host dependencies. Nix
provides Cargo, rustc, rustfmt, Clippy, and Python for code-generation checks;
Cargo manages Rust packages. No rustup, Python, or C compiler preinstallation is
required. Supported flake systems: Linux and macOS on x86-64 and AArch64.

The verification script runs formatting, warning-free Clippy, debug/release tests,
core-only tests, doctests, rustdoc, compile-fail consumer fixtures (checking their
intended diagnostics), LLVM proof-erasure checks, and `cargo package` including
rebuilding the packaged source. Tests exhaust all `u13` and `i16` values, numeric
edges/NaNs, UTF-8, predicate composition, branding invariance, proof isolation,
escape prevention, mutation restrictions, and preservation of ownership on errors.

For publication, run `nix develop -c cargo publish --dry-run --allow-dirty` and
then `nix develop -c cargo publish` from a clean checkout after checking the
package name is available and authenticating with crates.io. Publication is a
separate maintainer action; this repository does not contain registry credentials.
