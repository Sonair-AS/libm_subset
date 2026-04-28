# libm_subset

Sonair fork of `libm` from the [Ferrocene](https://github.com/ferrocene/ferrocene) repository (`library/compiler-builtins/libm`), a pure-Rust implementation of the C math library (`libm`).

The fork introduces a `sonair_certified` feature flag that gates out all functions not used by the Sonair certified firmware (consumed via the `float_math` crate). When `sonair_certified` is enabled, only the required `f32` math functions are compiled, and all unused code (`f64`-only functions, `f16`/`f128` variants, etc.) is excluded via `#[cfg(not(feature = "sonair_certified"))]`. This ensures the certified build contains only the code that has been reviewed and tested.

The certified subset exposes the following `f32` functions:

- `atan2f` — four-quadrant inverse tangent
- `atanf` — inverse tangent
- `ceilf` — round up to nearest integer
- `copysignf` — copy sign of a number
- `cosf` — cosine
- `fabsf` — absolute value
- `floorf` — round down to nearest integer
- `fmaf` — fused multiply-add
- `powf` — power function
- `roundf` — round to nearest integer
- `sinf` — sine
- `sqrtf` — square root
- `tanf` — tangent
- `truncf` — round towards zero

This repository also includes the `libm-test` harness for testing against known reference oracles (MPFR and musl libc). These oracles are used exclusively for testing and are not part of the certified build.

## Repository structure

```
libm/                  # The libm crate (Sonair certified subset)
libm-test/             # Test harness (MPFR, musl, standalone tests)
crates/libm-macros/    # Proc-macros for function enumeration
crates/musl-math-sys/  # FFI bindings to musl C math (optional)
scripts/               # Scripts (e.g. update-musl.sh)
```

## Prerequisites

- **Rust nightly** (required for `llvm-cov` and `f16`/`f128` support)
- **GMP/MPFR dev libraries** (for the `build-mpfr` feature):
  ```bash
  sudo apt install libgmp-dev libmpfr-dev
  ```
- **musl sources** (for the `build-musl` feature):
  ```bash
  ./ci/update-musl.sh
  ```

## Running tests

```bash
cargo test --workspace --lib \
    --test compare_built_musl --test multiprecision \
    --test standalone --test sonair_coverage \
    --features build-musl,build-mpfr --no-default-features
```

## Running coverage

```bash
cargo +nightly llvm-cov --workspace --lib \
    --test compare_built_musl --test multiprecision \
    --test standalone --test sonair_coverage \
    --features build-musl,build-mpfr --no-default-features \
    --ignore-filename-regex '(libm-test/|musl-math-sys/|libm-macros/|crates/)' \
    --html --output-dir target/coverage-libm
```

The `--ignore-filename-regex` flag filters out test infrastructure so the report only covers `libm` source files.

Some upstream test suites (e.g., `multiprecision`, `compare_built_musl`) use deterministic random inputs seeded by `LIBM_SEED`. Because not every run exercises every code path, coverage on certain files may vary slightly between runs. The `sonair_coverage` tests use targeted inputs to cover specific edge cases that random testing alone may not reliably hit.

Code that is not part of the certified subset (non-certified functions, generic trait infrastructure, macro-generated default trait methods) is annotated with `#[cfg_attr(coverage_nightly, coverage(off))]`. This keeps the coverage report focused on certified code while ensuring these annotations are only active under the nightly coverage toolchain. The excluded code is still implicitly exercised through the public functions that depend on it (e.g., trait methods used by `powf` and `atan2f`).

### Known uncovered lines

The following lines in certified files are not covered. Each is structurally unreachable for `f32` or only compiled under `debug_assertions`.

| File | Lines | Reason |
|------|-------|--------|
| `generic/sqrt.rs` | 222–224 | `d2 == 0` path — exhaustive `f32` search confirms unreachable |
| `generic/sqrt.rs` | 232 | `if F::BITS > 16` else branch — only reachable for `f16` |
| `generic/scalbn.rs` | 88–116 | `if F::BITS > 16` else branch — `f16`-specific prescaling |
| `rem_pio2_large.rs` | 239 | `debug_assert!` closing brace — not present in release builds |
| `rem_pio2_large.rs` | 342 | `_ => {}` match arm — `q0 >= 3` unreachable from `f32` path |
| `rem_pio2_large.rs` | 364, 390 | Second iteration of consecutive-zero `iq[]` loops — requires 3+ consecutive zeros, not achieved with `f32`-derived inputs |
| `rem_pio2_large.rs` | 444–468 | `prec == 3` (quad precision) — only used by `f128`, not in certified build |
| `rem_pio2_large.rs` | 471 | `debug_assertions`-only `unreachable!()` arm |
