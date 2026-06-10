# Phase 2: Ray Tracing: The Next Week (Rust)

This file mirrors `CLAUDE.md` for Codex. The root `AGENTS.md` applies; this file adds Phase 2 specific constraints.

## Phase 2 Goal

The learner is consolidating Rust fluency while learning modern ray tracing concepts. The single most important learning outcome of this phase is direct BVH implementation, which forms the conceptual foundation for understanding GPU acceleration structures in Phase 3.

## How Phase 2 Begins

Phase 2 begins by copying the final state of Phase 1 (`phase-1-rtiow`) into this directory. There is no Pre-Phase 2 because no new environment setup is required. The learner extends their existing path tracer with the chapters of The Next Week.

If the learner asks for help with the initial copy and the `Cargo.toml` package name change, this is ceremony and may be assisted.

## Forbidden

In addition to all Phase 1 prohibitions, which remain in effect for any code extending or modifying the existing tracer, do not write code for:

- BVH implementation in all its parts: bounding box (AABB) type and operations, BVH node structure, recursive construction, split axis selection, and ray-BVH traversal. This is the critical learning artifact of Phase 2.
- Texture types: `SolidTexture`, `CheckerTexture`, image texture sampling, Perlin noise generation, and turbulence.
- New geometric primitives: `Quad` and lights including emissive material scatter behavior.
- Instance transformations: translation and rotation wrappers around hittable objects.
- Volumetric rendering: `ConstantMedium` and isotropic scatter.

These items collectively form the algorithmic content of Phase 2. They are not ceremony under any framing.

## Permitted

- All Phase 1 permitted items apply.
- Debugging methodology questions for visual artifacts. For example, when Perlin noise output looks incorrect, suggest visualization approaches such as rendering noise to a 2D image first or isolating frequency versus amplitude effects rather than diagnosing the algorithm.
- PR-based code review via GitHub Actions is now available. Phase 1 was excluded from review during the first chapters; by Phase 2 the learner has established their own patterns.

## Special Note on BVH

The BVH is the most important code the learner writes in this entire project. It is the conceptual bridge to Phase 3's BLAS and TLAS. If the learner asks for direct assistance with BVH code, decline firmly and explain:

- The BLAS in DXR is, fundamentally, a GPU-side BVH.
- The TLAS is a BVH whose leaves are instances containing BLAS references.
- Understanding the construction tradeoffs such as split heuristic and build time versus traversal time by writing one is the only way to develop intuition that transfers to API-driven acceleration structures.

If the learner is stuck on BVH for an extended period, encourage continued effort over assistance. A learner who eventually writes a working BVH unaided is in a fundamentally different position from one who copied a working implementation.

## OBJ and glTF Loader: Optional Challenge

If the learner pursues the optional challenge of loading external mesh files, the parser itself is partially ceremony because file format parsing is not a learning target. However:

- Parser plumbing: AI assistance is acceptable in moderation. Crates like `tobj` or `gltf` may be discussed.
- Integration with the path tracer: the connection between parsed mesh data and the existing `Hittable` trait, including how triangle intersection is added to the tracer, is learning target. Do not write this.

## Performance Measurement

When the learner reports BVH performance improvements, they should record numbers, with and without BVH, for the same scene. If they ask for help interpreting `perf` output or `cargo flamegraph` results, this is methodology and may be discussed without writing optimization code.

## Milestone Completion

When the learner completes the Cornell Box with rotated boxes and volumetric fog, suggest the self-check questions in `docs/learning-plan.md` section 9.2 before they proceed to Phase 3.
