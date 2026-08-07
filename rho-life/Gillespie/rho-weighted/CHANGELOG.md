# rho-weighted changelog

## v0.3.1 — the (R3) bypass, and project scaffolding

Verification pass over v0.3.0. The patch was authored without a toolchain
available; it has now been applied to a clean v0.2.0 tree, compiled, and run.

**It applies cleanly and every claim in the v0.3.0 notes below checks out**, at
`-p0` from the `publications` repo root (`rho-life/Gillespie/`), producing a tree
byte-identical to the shipped tarball apart from `Cargo.lock`. Measured, not
merely asserted:

| Claim | Measured |
|---|---|
| entry `= √(mλ)`, weight `= mλ` | exact for m = 2, 3, 5, 8 |
| conservativity at `H = 0`, **non-diagonal fixture** | max abs. residual `5.6e-17` |
| amplitudes unbounded | `\|z\| = 5` accepted, `λ = 25`; only non-finite refused |
| basis keyed by configuration | plastic theory: dim 10, **10 keys, 2 markings** |
| self-transition convention | `a₀ = 2.0` kept, `Q[0][0] = -0.0` dropped, row sum 0 |

The conservativity row is worth isolating: the suite checks it on the diagonal
two-state chain, where it cannot discriminate. Checked on a fixture with several
derivations into one target — the case the normalisation exists to settle — the
residual is at machine precision. `examples/verify-claims.rs` prints all of it.

### Behaviour

**(R3) could still be bypassed by a struct literal.** v0.3.0 enforced
structurality in `complete_with_default_checked` and described it as "the single
funnel every partition in the crate is built through, so a modal key cannot reach
a theory by any route". It was not quite: `Partition`'s fields were `pub`, so
`Partition { keys, default_index }` sidestepped the check, and a modal key
reached a theory that way in about ten lines. `Partition::keys` and
`Partition::default_index` are now private with accessors of the same names, and
`Partition::from_checked_keys` is the only constructor. The claim now holds by
construction rather than by convention.
Regression: `law_r3_cannot_be_bypassed_by_a_struct_literal`.

*Breaking, minimally:* `p.keys` → `p.keys()`, `p.default_index` →
`p.default_index()`. Two call sites in the tree.

### Project

Scaffolding for building and testing outside the workspace, which is how the
crate is developed today and will be until `f1r3node-rust` builds from a clean
checkout.

* `README.md` — what it is, the simulator/interpreter distinction, build and
  test commands, a reading order mapping modules to note sections.
* `justfile` — `just`, `just verify`, `just sim rnn`, `just doc`, `just lint`.
* `.github/workflows/ci.yml` — fmt, build, clippy, both suites, and an assertion
  that the dependency set is empty (the crate must never be able to pull
  anything onto the consensus path).
* `examples/verify-claims.rs` — prints the numbers above rather than asserting
  them, so a reader can see magnitudes instead of a green tick.
* `Cargo.toml` — `rust-version = "1.75"`, features documented at their
  definitions, `[profile.test] opt-level = 2` (the statistical suites run 10⁵
  trajectories; debug arithmetic made them minutes rather than seconds).
* `.gitignore`.

Module paths are unchanged. The note's §9.2 cites this API and a reshuffle would
invalidate the citation for no gain.

## v0.3.0 — the normalisation and (R3) corrections

Brings the crate into line with the second version of the note
(`rho-life/weighted-gslt-v2.pdf`), which was revised in response to an external
referee. Three of the changes below are corrections to behaviour; the rest are
corrections to names and comments that were making claims the code did not
support.

### Behaviour

**A jump channel's amplitude is the square root of its aggregate rate**
(`quantum::QctmcModel::from_graph_unchecked`). Classical rates are now summed
over the derivations belonging to one channel `(rule, class, source, target)`
before a single square root is taken. v0.2.0 took a root per derivation and
summed those, which for `m` indistinguishable reactants gave a transition weight
`m²λ` against a classical rate of `mλ`; that superlinearity was recorded as an
open question about the physics and is in fact a normalisation artefact.
`|c⟩` is one *normalised* basis vector for a configuration whose parallel
composition is a multiset, so the `m` derivations are one route with degeneracy
`m`, and summing roots charges for the degeneracy twice. Second quantisation
fixes the factor for exactly this case: `a|m⟩ = √m|m-1⟩`.

The criterion selecting root-of-sum is **conservativity** — at `H = 0` the
populations must solve the forward equation of the classical chain — and it is
checkable rather than a matter of taste. Two consequences: the degeneration
theorem no longer needs a diagonality hypothesis, and no coherence arises from
the rewrite relation at all, so all of it is carried by `H`.

**The basis is keyed by configuration, not by marking** (`QctmcModel.basis`, new
`QctmcModel.labels`). Node *identity* in the exhaustive graph has always been the
configuration key, but the model stored the marking, so under a plastic theory
several basis vectors carried identical labels and `index_of` silently returned
whichever came first. `index_of` now resolves a configuration key; `index_of_term`
returns every index carrying a marking, and `term_population` sums the marginal.

**Amplitudes are unbounded** (`theory::RateValue::complex`). The `|z| ≤ 1` check
and `RateError::AmplitudeTooLarge` are removed; only finiteness is checked. On a
finite-dimensional space every operator is bounded outright, GKSL imposes no norm
condition on jump operators, and the bound manufactured a dimensional
inconsistency between unbounded inverse-time rates and bounded dimensionless
amplitudes. `RateValue::rate` is documented as the interpretation map
`λ(z) = |z|²`, which is the only relation between the two codomains.

**Requirement (R3), structurality, is enforced at elaboration time.** New
`Formula::nonstructural_at` / `is_structural`, `WhyNot::NotStructural`,
`Checkable::try_key`, `Checkable::trusted_key`, `Partition::check_key_fragment`,
and `complete_with_default_checked`. Keys must lie in the structural fragment:
no `⟨K_j⟩`, no `[K_j]`, no `ν`. `Checkable::try_new` still admits all of these,
because the restriction is on keys and not on properties.

This closes a real gap. `Checkable::try_new` admitted modal and fixed-point
formulae as keys, so the locality lemma the incremental propensity scheme rests
on was false for keys the crate accepted — a modal key inspects *successors*, not
a bounded syntactic neighbourhood. Enforcement lives in
`complete_with_default_checked`, the single funnel every partition in the crate
is built through, so a modal key cannot reach a theory by any route including
`Checkable::trusted`.

### Names and comments that were overclaiming

* `quantum::interference` → `quantum::degeneracy`. The counts it reports are
  multiset degeneracy, which is what the classical multiplicity factor `h` is
  counting; they were never interference. The old name is kept as a
  `#[deprecated]` alias so external callers break at the name rather than quietly
  at the semantics.
* `quantum::is_diagonal` is now documented as a diagnostic. It was a hypothesis
  of the degeneration theorem only because of the old normalisation.
* `QctmcModel::with_phase` no longer claims to be "the knob that turns a
  classical-looking model into one that interferes." It is a carrier check. The
  suite has always said so — `an_overall_phase_leaves_populations_invariant`
  asserts the populations are untouched — and no per-class phase could ever
  cancel anything, since within one `L_{r,φ}` every derivation carries the same
  `z` while `g` and `χ` are non-negative reals.
* `tests/rnn.rs::the_response_is_logistic_and_nobody_wrote_it` →
  `the_response_saturates_and_nobody_wrote_it`. `1 - exp(-Δx)` is exponential
  saturation, not the logistic function. The body always asserted the correct
  formula.
* `graph::Generator::from_graph` documents the self-transition convention: `Q`
  drops self-loops and its diagonal is `-Σ_{j≠i} Q(i,j)`, while the sampler keeps
  them in `a₀` and fires them. A self-loop is a fictitious jump in the sense of
  uniformisation, so the two agree on the distribution of the state at every time
  and differ only on event counts and sojourns. Neither is wrong; mixing them
  silently would be.

### Tests

| Test | Establishes |
|---|---|
| `indistinguishable_reactants_do_not_interfere` | inverts its v0.2.0 namesake: weight `= m\|z\|²`, entry `= √(m·rate)` |
| `sum_of_roots_would_have_given_m_squared` | negative control — reconstructs the old construction and checks it differs by the factor `m`, so the equality above is discriminating |
| `the_degeneration_holds_without_diagonality` | `Σ‖L\|c⟩‖² = a₀` on a deliberately non-diagonal fixture |
| `law_self_transitions_are_fictitious_jumps` | a self-loop is in `a₀`, absent from `Q`, and `Q`'s row sums to zero |
| `law_lambda_is_the_interpretation_map` | `λ(z) = \|z\|²` and is phase-blind |
| `law_rates_are_nonnegative_reals_not_probabilities` | extended: `\|z\| = 5` is an ordinary amplitude |
| `the_structural_fragment_is_exactly_the_local_one` | (R3) accepts and refuses the right formulae, including buried modalities |
| `a_modal_formula_is_a_property_but_not_a_key` | both halves — refused by `try_key`, accepted by `try_new` |
| `a_modal_key_cannot_reach_a_theory` | (R3) at partition construction, against a key that lied on the way in |
| `completion_preserves_structurality` | the synthesised `default` never smuggles in a violation |
| `the_shipped_examples_satisfy_r3` | a regression in `examples.rs` would otherwise show up only as a slow simulator |

### Not verified here

This revision was written without a Rust toolchain available, so
`cargo test --features quantum` has not been run against it. Delimiter balance,
match exhaustiveness over every `Formula` variant, deref coercions, and every API
the new tests call were checked by hand, and no other construction site of
`QctmcModel` or consumer of `RateError::AmplitudeTooLarge` exists in the tree.
Compile before trusting.
