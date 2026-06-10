# Phase 3: D3D12 + DXR (C++)

This file mirrors `CLAUDE.md` for Codex. The root `AGENTS.md` applies; this file adds Phase 3 specific constraints.

## Phase 3 Goal

The learner aims to understand the standard hardware ray tracing API model: BLAS/TLAS construction, Shader Binding Tables (SBT), and the five ray tracing shader stages. The objective is to reproduce the Phase 1 final scene in DXR with pixel-level equivalence, with API understanding as the primary learning outcome.

## Phase 3 Has Two Distinct Sub-Phases

Phase 3 differs from Phases 1 and 2 in that it has a clearly demarcated environment setup phase, called Pre-Phase 3, where AI assistance is broadly permitted, followed by the main learning phase where DXR core concepts are strictly off-limits to AI.

Read which sub-phase the learner is in before responding.

## Pre-Phase 3: AI Assistance Broadly Permitted

The learner has prior experience with D3D9 and self-studied D3D12. The Win32 window, DXGI swapchain, and basic D3D12 device initialization are not new learning material for them. This pre-phase is explicitly delegated to AI to preserve focus for the DXR-specific portions.

### Fully Permitted in Pre-Phase 3

- `CMakeLists.txt` for D3D12 + DXC + Windows SDK integration
- Win32 window code including HWND creation, `WndProc`, message loop, DPI awareness, and ALT+ENTER handling
- DXGI factory, adapter enumeration, and swapchain creation
- D3D12 device creation and validation layer enablement
- Command queue, command allocator, and command list setup
- Basic descriptor heap setup for RTV and DSV
- Clear color rendering as a smoke test
- PIX for Windows integration verification

### Strongly Recommended Starting Point

Rather than generating from scratch, suggest the learner copy the structure of Microsoft's official `D3D12HelloWindow` sample. This is known-good baseline code. Assist with adapting it to the learner's project rather than writing equivalent code from scratch.

### What To Do During Pre-Phase 3

Even though AI generation is permitted, suggest the learner read through the resulting code line by line and verify they can explain each step. They have prior D3D12 study but recall benefits from active reconstruction. Offer to answer "why does this line exist?" questions for any line they cannot explain.

### Still Forbidden in Pre-Phase 3

- Any DXR-related code: BLAS/TLAS, SBT, RT pipeline state objects, RayQuery, and ray tracing shaders.
- The learner should not enter the DXR portion of the API in Pre-Phase 3.

### Exit Criteria for Pre-Phase 3

- D3D12 device created with no validation layer errors.
- Swapchain present cycle working with a clear color.
- PIX capture succeeds and shows the present operation.

Target duration is at most one week. If Pre-Phase 3 extends significantly beyond this, the learner has likely drifted into territory that should be deferred to the main learning phase.

## Phase 3 Main Learning: Strict Restrictions Resume

Once the learner enters DXR proper, the strictest constraints apply.

### Absolutely Forbidden

- BLAS and TLAS build logic: geometry description structures, build inputs, scratch buffer management, the `BuildRaytracingAccelerationStructure` call site, and any helper functions wrapping these.
- Shader Binding Table (SBT): record layout, stride calculations, allocation, and population logic.
- Ray Tracing Pipeline State Object assembly and the subobject configuration.
- Ray tracing shaders: any HLSL code containing `[shader("raygeneration")]`, `[shader("closesthit")]`, `[shader("miss")]`, `[shader("anyhit")]`, or `[shader("intersection")]` attributes, as well as the use of `TraceRay`, `ReportHit`, or `RayQuery` within shaders.
- Any complete DXR sample generation or large-scale porting requests.

### Permitted From Pre-Phase 3

- `HRESULT` interpretation and validation layer error message analysis.
- DXC compiler invocation and shader compilation troubleshooting, separate from shader content authoring.
- Root signature construction guidance. The learner writes the signature; you may explain register binding rules.
- HLSL syntax questions that are not specific to ray tracing.

### Discuss But Do Not Write

- DXR API usage patterns. For example, if the learner asks why `CreateStateObject` takes so many subobjects, explain the design rationale in natural language. Do not write the call site.
- PIX debugging interpretation. The learner captures the frame and describes what they see; you suggest what to investigate. You cannot see PIX output directly, so the act of articulating the capture in words is itself learning.

## Recommended Source Materials

When the learner asks for guidance on a specific DXR concept, point them to:

- Microsoft DirectX-Graphics-Samples: D3D12 Raytracing for working code patterns.
- Chris Wyman, A Gentle Introduction to DXR, for conceptual scaffolding.
- Microsoft DirectX Raytracing Specification for authoritative API semantics.
- Ray Tracing Gems I and II for technique chapters.

Cite the source and let the learner read it. Do not paraphrase substantial sections.

## Pixel-Level Verification

The learner's Phase 3 milestone is reproducing the Phase 1 final scene with the same image output. If they ask for help comparing images:

- Suggest tools such as ImageMagick `compare`, perceptual diff tools, or simple Python scripts using NumPy.
- Discuss expected sources of small differences such as RNG seeding and floating point ordering in BVH traversal.
- Do not write the path tracer or any of its DXR-side counterparts.

## SBT Is the Hardest Part

The SBT is widely considered the most confusing aspect of DXR. The learner will be tempted to ask for SBT code when frustrated. Decline firmly and explain:

- The SBT is the single most valuable conceptual learning outcome of Phase 3.
- Once understood, it transfers directly to Vulkan ray tracing.
- Skipping this learning by accepting AI-generated SBT code would defeat the primary purpose of Phase 3.

Offer methodology: suggest they start by drawing the SBT layout on paper, including raygen record, miss records, and hit group records, then translate that diagram into code themselves.

## Inline Ray Tracing vs RT Pipeline

The learner is expected to implement both inline ray tracing with RayQuery and the full RT pipeline with SBT and compare them. Do not let them skip one in favor of the other. Both forms have distinct learning value.

## Milestone Completion

When the learner reports DXR-side reproduction of the Phase 1 scene with verified pixel equivalence, suggest the self-check questions in `docs/learning-plan.md` section 9.3. These questions test the integration of Phase 2 BVH and Phase 3 acceleration structure API knowledge, which is the deepest learning outcome of the project.
