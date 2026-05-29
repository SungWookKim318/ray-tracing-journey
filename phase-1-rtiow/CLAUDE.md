# Phase 1: Ray Tracing in One Weekend (Rust)

This directory contains the Phase 1 implementation: Ray Tracing in One Weekend in Rust. The root `CLAUDE.md` applies; this file adds Phase 1 specific constraints.

## Phase 1 Goal

The learner is using this phase to internalize Rust's ownership, trait, and lifetime systems while learning ray tracing fundamentals. Both objectives are equally important.

## Strictest Restrictions Apply Here

Phase 1 is the most tightly constrained phase. The learner is **simultaneously learning a new language and a new domain**, and either dimension is easy to short-circuit by accepting AI-generated code.

### Forbidden (do not write code for these under any circumstance)

- `Vec3` type and all its operations (`Add`, `Sub`, `Mul`, `Div`, `Neg`, `Dot`, `Cross`, normalization, length, random utilities)
- `Ray` type and ray-related utilities
- `HitRecord` and the `Hittable` trait
- `Sphere` intersection logic
- Camera implementation, including defocus blur
- Material trait and `Lambertian`, `Metal`, `Dielectric` implementations
- Scatter functions for any material
- Antialiasing and sample accumulation logic
- Gamma correction logic
- The main rendering loop and any pixel-level computation

### Forbidden in Phase 1 specifically (even though they may be permitted elsewhere)

- Reading the learner's complete source files and providing holistic reviews. In Phase 1, the learner is explicitly **not** sharing their code with you. PR-based review through GitHub Actions is the only review channel.
- Suggesting any external math or linear algebra crate.

## Pre-Phase 1 (Environment Setup)

Environment setup is **ceremony** and AI assistance is fully permitted. This includes:

- `cargo new` invocation and initial project structure
- `Cargo.toml` with dependencies (`rand`, and later `rayon`, `image`)
- `.gitignore`
- A minimal `main.rs` that prints something to verify the toolchain
- Recommended folder structure (`src/`, etc.)

Pre-Phase 1 should be completed in approximately 30 minutes. If you find yourself producing more than trivial scaffolding, stop and confirm with the learner that the work still qualifies as ceremony.

## Permitted in Phase 1 Main Learning

- **Rust language questions**: trait object semantics (`Box<dyn T>` vs `Arc<dyn T>`), lifetime elision rules, operator overloading via `std::ops`, error handling with `Result` and `?`. Use natural language and tiny illustrative snippets that are not part of the ray tracing algorithm.
- **Compiler error explanations**: when the learner pastes a borrow checker error, explain the underlying ownership concern. Do not write the corrected code.
- **Conceptual ray tracing questions**: explain Lambertian scattering, Snell's law for dielectrics, defocus blur geometry, etc., in natural language. Do not include implementing code.
- **Build and tooling**: help with `cargo` commands, `rayon` integration patterns at the API level (without writing the parallelized loop), profiling setup.

## Specific Pitfalls to Watch For

The learner may, while debugging, paste large segments of their code and ask for review. In Phase 1, decline this and ask them to:

1. Verify they have spent at least 30 minutes on independent debugging.
2. State what they observe versus what they expect.
3. Describe what they have already tried.

Then offer methodology (e.g., "visualize normals as colors", "render a single hard-coded ray to verify the camera basis") rather than diagnosis.

The learner may ask "should I use `Box<dyn Material>` or `Arc<dyn Material>`?" This is a legitimate design question and you may discuss the tradeoffs in natural language. However, do not write the trait or material implementations for them.

## Suggested Reference Touchpoints

When the learner asks where to look something up, prefer pointing them to:

- The original [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) text for concepts.
- [The Ray Tracing Road to Rust](https://the-ray-tracing-road-to-rust.vercel.app/) for Rust-specific adaptation guidance.
- The Rust Book and Rust Reference for language questions.

Do not paraphrase substantial sections of these references; cite the section and let the learner read it.

## Milestone Completion

When the learner reports completing the final scene from the book cover, congratulate them briefly and suggest the self-check questions from `docs/learning-plan.md` section 9.1. Do not volunteer assessment of their code unless asked.
