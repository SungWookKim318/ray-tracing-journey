# Ray Tracing Journey

A personal study project for learning ray tracing in depth. The repository documents a three-phase progression from software ray tracing in Rust to hardware-accelerated ray tracing with DirectX Raytracing.

This is a learning-focused side project, not a production codebase. Code is written from scratch wherever the algorithm is the learning target, and external graphics math libraries are intentionally avoided.

## Structure

The project is divided into three phases.

### Phase 1: Ray Tracing in One Weekend (Rust)

Implementation of Peter Shirley's *Ray Tracing in One Weekend* in Rust. Covers `Vec3` math, ray-sphere intersection, antialiasing, materials (Lambertian, Metal, Dielectric), and defocus blur. Serves as a vehicle for learning Rust's ownership, trait, and lifetime system alongside ray tracing fundamentals.

See [phase-1-rtiow/](phase-1-rtiow/).

### Phase 2: Ray Tracing: The Next Week (Rust)

Continuation in Rust covering modern ray tracing concepts: motion blur, bounding volume hierarchies, texture mapping, Perlin noise, quadrilaterals, lights, instances, and volumetric rendering. The BVH implementation in this phase is treated as the foundational asset for understanding the acceleration structures used in Phase 3.

See [phase-2-rtnw/](phase-2-rtnw/).

### Phase 3: DirectX Raytracing (C++)

Transition to C++ to learn the hardware-accelerated ray tracing API. Covers D3D12 fundamentals, BLAS and TLAS lifecycle, Shader Binding Table layout, the five ray tracing shader stages, inline ray tracing, and debugging with PIX. The central milestone is reproducing the Phase 1 final scene in DXR for pixel-level verification.

See [phase-3-dxr/](phase-3-dxr/).

## Progress

TBD.

## Code Review Workflow

Pull requests in this repository are reviewed by two AI agents in parallel — one focused on language and abstraction concerns, the other on numerical and algorithmic correctness — with a third agent synthesizing their output. Conflicting opinions are surfaced rather than resolved, leaving each trade-off as an explicit decision for the learner. The full design rationale is documented in section 6 of [docs/learning-plan.md](docs/learning-plan.md).

## Development

### CI Setup (mock stage)

Three GitHub Actions workflows live under [.github/workflows/](.github/workflows/) — `phase1-review.yml`, `phase2-review.yml`, `phase3-review.yml` — each implementing the three-job dual-agent review structure described in section 6.5 of the learning plan. The workflows are currently in **mock mode**: they run on `ubuntu-latest`, echo trigger context and job dependencies, and produce placeholder review artifacts so that path filters, artifact passing, and the `synthesizer needs [reviewer-*]` ordering can be verified before any agent cost is incurred. Each file also includes `workflow_dispatch` for manual verification.

Activation requires:
- Registering a self-hosted runner for this repository (the design assumes a local machine running an idle runner)
- Switching each job's `runs-on` from `ubuntu-latest` to `self-hosted`
- Removing the mock step and uncommenting the real agent invocation block in each job (Claude Code, Codex CLI reviewer, Codex CLI synthesizer)
- Configuring authentication for Claude Code (Anthropic API key) and Codex CLI (per the existing subscription)

Activation is planned at the end of Pre-Phase 1 or immediately before the first 본 학습 PR is opened, whichever comes first. Until then, mock runs are the only intended use.

## Documentation

The full learning plan, including AI usage rules, per-phase milestones, and self-assessment questions, is documented in [docs/learning-plan.md](docs/learning-plan.md). The plan itself is written in Korean.
