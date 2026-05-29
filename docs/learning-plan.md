# Ray Tracing 학습 계획

## 1. 개요

본 학습 계획은 세 단계로 구성된다. 본업과 무관한 사이드 프로젝트로, Ray Tracing의 동작 원리를 깊이 있게 이해하는 것을 최종 목표로 한다.

| Phase | 주제 | 언어 | 예상 기간 | 핵심 목표 |
|---|---|---|---|---|
| 1 | Ray Tracing in One Weekend | Rust | 1~2주 | Rust 언어 습득 및 RT 기초 |
| 2 | Ray Tracing: The Next Week | Rust | 2~3주 | Modern RT 개념 및 BVH 직접 구현 |
| 3 | DXR (D3D12) | C++ | 4~8주 | 하드웨어 가속 RT API의 표준 모델 이해 |

### 1.1 학습자의 사전 배경

본 계획의 AI 활용 규칙은 학습자의 다음 사전 배경을 전제로 한다.

- iOS/Android 미디어 및 렌더링 엔진 개발 경험 (OpenGL ES, Metal, C++, Swift, Objective-C)
- D3D12 학부 시절 학습 경험 및 D3D9 기반 게임 프로젝트 경험
- 현 직무에서 Rust 및 wgpu를 활용한 렌더러 개발 진행 중

이로 인해 일부 환경 구축 작업(특히 Phase 3의 Windows API, DXGI swapchain 등)은 학습자에게 이미 익숙한 영역이며, 본 계획에서 ceremony로 분류되어 AI 적극 활용이 허용된다.

---

## 2. AI 활용 원칙 (전 Phase 공통)

### 2.1 기본 원칙

학습의 본질은 실패와 디버깅의 누적이다. AI에게 코드를 받을 경우 컴파일은 성공할 수 있으나, 학습은 일어나지 않는다.

본 원칙에서 "자제"와 "금지"는 구분된다. 학습 효과를 해치지 않는 범위 내에서는 AI를 적극 활용한다.

AI의 응답은 코드 블록이 아닌 자연어로 요청한다. 개념, 방법론, 진단 방향을 수용하되, 구현 코드는 직접 작성한다.

30분 룰을 적용한다. 문제에 막혀도 최소 30분은 단독으로 디버깅을 수행한다.

### 2.2 공통 허용 및 제한 사항

**허용 사항**
- 개념 질문 (예: BVH의 SAH가 무엇이며 어떤 목적으로 사용되는가)
- 학습 자료 및 도서 추천
- 본인 코드에 대한 사후 리뷰 (PR 단위, 6장 GitHub 워크플로우 참조)

**제한적 허용 사항**
- 컴파일러 에러 해석: 원인 설명만 수용하며, 수정 코드는 수용하지 않는다
- Rust 문법 질문: `Box<dyn T>`와 `Arc<dyn T>`의 차이 등 검색 대체재로 활용한다
- 디버깅 힌트: 답이 아닌 가설 및 방법론 형태로 수용한다

**금지 사항**
- 알고리즘 및 수학 코어 코드 작성 의뢰
- 본인 코드 전체를 제공하고 수정 또는 리팩토링을 요청하는 행위
- 30분 룰을 통과하지 않은 상태에서의 디버깅 도움 요청

### 2.3 AI 사용 가능 여부 판별 기준

특정 코드 작성 시 AI 사용 여부가 모호한 경우, 다음 세 질문을 적용한다.

1. 이 코드가 RT 또는 RT API의 핵심 추상에 해당하는가?
2. 이 코드를 직접 타이핑하는 것이 본인의 그래픽스 또는 Rust 학습에 기여하는가?
3. 이 코드는 그래픽스 API와 무관한 ceremony(빌드 시스템, 윈도우 생성 등)에 해당하지 않는가?

위 세 질문 모두에 "예"로 답할 수 없는 경우 AI 활용을 허용한다. 모두 "예"인 경우(즉 RT 본질부에 해당)에는 직접 작성한다.

판별 예시:

| 작업 | Q1 | Q2 | Q3 | AI 활용 |
|---|---|---|---|---|
| Vec3 구현 | 예 | 예 | 예 | 금지 |
| BVH 구현 | 예 | 예 | 예 | 금지 |
| BLAS/TLAS 빌드 | 예 | 예 | 예 | 금지 |
| SBT 셋업 | 예 | 예 | 예 | 금지 |
| RT 셰이더 작성 | 예 | 예 | 예 | 금지 |
| Cargo.toml 작성 | 아니오 | 아니오 | 아니오 | 허용 |
| CMakeLists.txt 작성 | 아니오 | 아니오 | 아니오 | 허용 |
| Win32 윈도우 코드 | 아니오 | 아니오 | 아니오 | 허용 |
| DXGI swapchain 셋업 | 아니오 | 부분적 | 아니오 | 허용 (단, 정독 권장) |

---

## 3. Phase 1: Rust + Ray Tracing in One Weekend

### 3.1 목표

Rust의 ownership, trait, lifetime을 RT 코드 작성을 통해 자연스럽게 습득한다. Ray-Sphere intersection, 카메라, 머티리얼의 기본 개념을 이해한다.

### 3.2 Pre-Phase 1: 환경 구축

본 항목은 Phase 1 본 학습 진입 전 환경 준비 작업이며, **AI 적극 활용을 허용한다.**

**AI 활용 허용 범위**
- `cargo new`를 통한 프로젝트 생성
- Cargo.toml 초기 구성 및 의존성 추가 (rand, 이후 rayon, image)
- .gitignore 작성
- 기본 main.rs 스캐폴드
- 기본 폴더 구조 결정

**Pre-Phase 1에서도 금지되는 항목**
- Vec3, Ray, HitRecord 등 Phase 1 본 학습의 대상 코드
- PPM 파일 출력 로직 (RTIOW의 첫 챕터 내용이므로 본 학습에서 직접 작성)

**산출물**
- `cargo run`이 성공적으로 실행되어 "Hello, world!"가 출력되는 상태
- 의존성 추가가 정상 동작함을 확인

본 단계는 30분 이내 완료를 목표로 한다. 이 시간을 초과하면 환경 구축 외 영역에 진입한 것이다.

### 3.3 핵심 학습 항목 (본 학습)

- [ ] Vec3 직접 구현 (`std::ops` 트레잇을 활용한 연산자 오버로딩)
- [ ] Ray, HitRecord
- [ ] Sphere intersection
- [ ] Antialiasing (다중 샘플링)
- [ ] Lambertian, Metal, Dielectric 머티리얼 (trait 및 `Arc<dyn Material>` 활용)
- [ ] Defocus blur 카메라
- [ ] PPM 및 PNG 형식 출력
- [ ] 확장 항목: rayon을 활용한 픽셀 루프 병렬화

### 3.4 Rust 적응 시 결정 사항

- Vec3 타입: `Copy` trait 적용 여부 및 연산자 오버로딩 방식
- 머티리얼 및 도형: `Box<dyn Trait>`, `Arc<dyn Trait>`, 제네릭 중 적절한 선택
- 에러 처리: PPM 및 PNG IO에서 `Result` 및 `?` 연산자 사용
- 외부 수학 라이브러리(nalgebra 등)는 의도적으로 사용하지 않는다. 수학 코드를 직접 작성하는 것이 본 Phase 학습 가치의 핵심이다.

### 3.5 Phase 1 본 학습의 AI 활용 규칙

Pre-Phase 1과 달리, 본 학습 단계에서는 가장 엄격한 제한을 적용한다.

**허용**
- Rust 문법 및 관용구 질문 (자연어 답변만 수용)
- 컴파일러가 특정 lifetime을 거부하는 원인에 대한 분석
- RT 개념 질문 (Lambertian, dielectric refraction 원리 등)

**금지**
- Vec3, Ray, HitRecord, Sphere intersection, scatter 함수 등 알고리즘 코어 코드 작성 요청
- 본인이 작성한 코드를 전체 제공하고 검토 또는 리팩토링을 요청하는 행위 (본 Phase에서는 PR 리뷰만 활용)
- "어떻게 구현하는가" 유형의 광범위한 질문 (도서 및 자료만 활용)

특히 Phase 1 전반부(처음 5~6개 챕터)에는 본인이 작성한 코드를 AI에게 노출하지 않는다. 잘못된 추상화가 발생했을 때 도서와 자료를 통해 스스로 인지하고 교정하는 경험이 핵심이다.

### 3.6 학습 자료

**주요 자료**
- [Ray Tracing in One Weekend (영문 원본)](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [The Ray Tracing Road to Rust](https://the-ray-tracing-road-to-rust.vercel.app/) — Rust 친화적 각색본

**한국어 참고 자료**
- [velog @parksj3205 — RTIOW 번역 시리즈](https://velog.io/@parksj3205/Ray-Tracing-in-one-weekend-2.-Output-an-Image-%EB%B2%88%EC%97%AD)
- [GitHub: sejinpark12/RayTracing_in_One_Weekend_Kor](https://github.com/sejinpark12/RayTracing_in_One_Weekend_Kor)

한국어 번역본은 구버전 기준일 가능성이 있다. 개념 이해는 한국어 자료를 활용하되, 코드 구현은 영문 최신판 및 Road to Rust를 기준으로 한다.

**참고 구현체 (디버깅 시 비교 용도)**
- [fralken/ray-tracing-in-one-weekend](https://github.com/fralken/ray-tracing-in-one-weekend) — 챕터별 git tag 제공
- [perliedman/raytracing-in-one-weekend](https://github.com/perliedman/raytracing-in-one-weekend)

### 3.7 Milestone

- [ ] 책 표지의 final scene 렌더링 완성
- [ ] rayon 병렬화 적용 후 렌더 시간 단축 측정 및 기록
- [ ] 결과물을 GitHub 공개 저장소로 정리

---

## 4. Phase 2: Rust + Ray Tracing: The Next Week

### 4.1 목표

Modern RT의 핵심 개념을 학습한다. 특히 BVH를 직접 구현한다. 이는 Phase 3의 Acceleration Structure 이해를 위한 가장 중요한 자산이다.

### 4.2 시작 방법

본 Phase는 별도의 Pre-Phase를 두지 않는다. Phase 1의 결과물(`phase-1-rtiow` 디렉토리)을 그대로 복사하여 `phase-2-rtnw`로 시작한다.

```
cp -r phase-1-rtiow phase-2-rtnw
cd phase-2-rtnw
# Cargo.toml의 package name을 phase-2-rtnw로 변경
```

이는 RTNW 도서가 RTIOW에서 발전된 형태로 진행되는 구조를 따른다.

### 4.3 핵심 학습 항목

- [ ] Motion Blur
- [ ] BVH (Bounding Volume Hierarchy) — Phase 3과 직접 연결됨
- [ ] Solid texture, Checker texture
- [ ] Image texture mapping
- [ ] Perlin noise
- [ ] Quadrilaterals
- [ ] Lights (emissive material)
- [ ] Cornell Box
- [ ] Instances (translation, rotation)
- [ ] Volumetric rendering (참여 매질)

### 4.4 BVH 학습 시 유의 사항

Phase 3에서 GPU의 BLAS 및 TLAS를 사용하게 되는데, 그 내부 동작은 BVH다. 본 단계에서 다음을 확보한다.

- SAH(Surface Area Heuristic)까지 도달하지 않더라도, axis-aligned 분할은 직접 구현한다
- BVH 적용 전후의 traversal 비용을 측정하여 기록한다
- 직접 구현한 BVH 노드가 BLAS에, 인스턴스 단위 BVH가 TLAS에 대응됨을 명확히 인지한다

### 4.5 Phase 2의 AI 활용 규칙

Phase 1 본 학습과 거의 동일하되, 일부 항목을 완화한다.

**허용 (Phase 1과 동일)**
- Phase 1 허용 사항 전체
- 디버깅 방법론 질문 (예: Perlin noise 결과가 비정상적일 때의 시각화 디버깅 접근법)
- PR 단위의 AI 코드 리뷰 (6장 워크플로우 참조)

**금지**
- BVH 알고리즘 코드는 절대 AI에게 작성 의뢰하지 않는다. Phase 3 이해의 핵심 자산이다.
- intersection 함수, scatter 함수 등 알고리즘 코어
- "이 챕터를 한 번에 구현해달라" 유형의 광범위한 요청

**신중하게 활용**
- OBJ 및 glTF 로더 구현 (선택 도전 과제). 파서 자체는 학습 대상이 아니므로 부분적으로 도움받을 수 있다. 다만 텍스처 및 머티리얼을 본인의 path tracer와 연결하는 부분은 직접 작성한다.

### 4.6 학습 자료

- [Ray Tracing: The Next Week (영문 원본)](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- [Physically Based Rendering 4판 (online 무료)](https://pbr-book.org/) — Phase 2 및 3 진행 중 사이드로 chapter 단위 발췌 독서를 권장한다.

### 4.7 Milestone

- [ ] Cornell Box, 회전된 박스, 안개 렌더링 완성
- [ ] BVH 적용 전후 렌더 시간 비교 기록
- [ ] (도전 과제) 본인 path tracer로 임의의 OBJ 또는 glTF 모델 로드 및 렌더링

---

## 5. Phase 3: D3D12 + DXR

### 5.1 언어 전환에 대하여

Rust에서 C++로 의도적으로 전환한다. 사유는 다음과 같다.

- DXR 및 Vulkan RT 학습 자료, 공식 샘플, Ray Tracing Gems 도서 코드 등이 모두 C++로 작성되어 있다.
- Phase 1 및 2에서 RT 알고리즘은 이미 습득한 상태이므로, Phase 3에서는 RT 자체보다 API의 추상 모델 이해에 집중한다.
- 언어 friction 없이 RT API에 집중하기 위함이다.

### 5.2 목표

하드웨어 가속 RT API의 표준 모델(BLAS, TLAS, SBT, 5종 셰이더 스테이지)을 이해한다. Phase 1 및 2에서 작성한 씬을 DXR로 재현한다.

### 5.3 Pre-Phase 3: 환경 구축

본 항목은 Phase 3 본 학습 진입 전 환경 준비 작업이며, **AI 적극 활용을 허용한다.**

**배경**

학습자는 학부 시절 D3D9 기반 게임 프로젝트 경험 및 별도의 D3D12 학습 경험을 보유하고 있다. DXGI 및 Windows API를 통한 swapchain 연결에 대한 사전 지식이 있다. 따라서 본 Pre-Phase는 학습이 아닌 환경 구축 ceremony에 해당하며, AI 활용을 적극 권장한다.

**AI 활용 허용 범위**
- CMake 셋업 (D3D12, DXC, Windows SDK 통합)
- Win32 윈도우 코드 (HWND, WndProc, 메시지 루프)
- DXGI swapchain 셋업
- D3D12 device, command queue, basic descriptor heap 초기화
- Validation layer 활성화
- 정상 동작 확인용 코드 (clear color 렌더링)

**권장 시작점**

AI에게 처음부터 생성하도록 요청하는 것보다, Microsoft 공식 샘플을 시작점으로 활용하는 것을 권장한다.

- [DirectX-Graphics-Samples — D3D12HelloWorld](https://github.com/microsoft/DirectX-Graphics-Samples/tree/master/Samples/Desktop/D3D12HelloWorld) — Known-good baseline
- 본 샘플의 CMake 구조 및 Win32 윈도우 코드를 lift하고, 필요한 부분만 AI를 통해 수정한다.

**권장 절차**

1. MS 공식 샘플의 윈도우 및 swapchain 코드를 본 프로젝트로 복사
2. CMake는 AI에게 본인 프로젝트 구조에 맞게 수정하도록 요청
3. 받은 코드를 줄 단위로 정독하여 각 단계의 의미를 본인이 설명할 수 있는 상태로 만든다
4. 이해가 어려운 부분은 AI에게 "이 줄이 왜 필요한가" 형태로 질의

작성은 AI 및 샘플에 위임하되, **이해는 본인이 수행한다.**

**Pre-Phase 3에서도 금지되는 항목**
- RT 관련 모든 코드 (BLAS/TLAS, SBT, RT 셰이더, RayQuery 등)

**산출물**
- D3D12 device가 생성되어 validation layer에서 에러가 없는 상태
- swapchain이 동작하여 clear color로 화면이 채워지는 상태
- PIX for Windows로 캡처가 정상 동작함을 확인

본 단계는 최대 1주 이내 완료를 목표로 한다.

### 5.4 핵심 학습 항목 (본 학습)

- [ ] BLAS 및 TLAS 빌드 라이프사이클
- [ ] Scratch buffer 관리
- [ ] Ray Tracing Pipeline State Object
- [ ] Shader Binding Table (SBT)
- [ ] 5종 셰이더 스테이지: raygen, closesthit, miss, anyhit, intersection
- [ ] HLSL의 RT extension (`TraceRay`, `ReportHit` 등)
- [ ] Inline ray tracing (RayQuery)과 RT pipeline 비교
- [ ] PIX for Windows를 활용한 RT 디버깅

### 5.5 단계별 진행

Pre-Phase 3 완료 이후 다음 순서로 진행한다.

**5.5.1 DXR hello triangle (1~2주)**

삼각형 하나를 RT로 렌더링한다. BLAS 및 TLAS 빌드, SBT 셋업, raygen, closesthit, miss 셰이더 작성을 포함한다. 보일러플레이트가 가장 크게 증가하는 단계이며, SBT 학습의 핵심 단계다.

**5.5.2 RTIOW 씬을 DXR로 재현 (2~3주, 핵심 마일스톤)**

Phase 1에서 작성한 final scene을 DXR로 재현한다. 동일한 결과 이미지를 목표로 하므로 픽셀 단위 비교 검증이 가능하다. RT 알고리즘은 이미 숙지된 상태이므로 API 학습에 집중할 수 있다.

**5.5.3 Path tracing 확장 (선택)**

Cook-Torrance BRDF, importance sampling, NEE(Next Event Estimation), Russian Roulette 등을 다룬다. PBR 영역에 진입한다.

### 5.6 Phase 3 본 학습의 AI 활용 규칙

Pre-Phase 3과 달리, 본 학습 단계에서는 RT 본질부에 대해 엄격한 제한을 유지한다.

**허용 (Pre-Phase 3에서 이어짐)**
- Pre-Phase 3에서 허용된 모든 항목
- HRESULT 코드 해석 및 validation layer 에러 메시지 해석
- DXC (Shader Compiler) 사용법 및 root signature 작성 지원
- HLSL 일반 문법 질문 (RT 관련 syntax 제외)

**금지 (Phase 3 학습의 핵심)**
- BLAS 및 TLAS 빌드 로직
- SBT 레이아웃 및 셋업 코드
- RT 셰이더(raygen, closesthit, miss, anyhit, intersection)는 직접 작성한다
- "DXR로 path tracer를 작성해달라" 유형의 광범위한 요청

**신중하게 활용**
- DXR API 사용 패턴 질문 (`CreateStateObject`에 전달되는 subobject 구조 등) — 설명만 수용하며 코드는 직접 작성한다
- PIX 디버깅 인사이트 — AI는 PIX 캡처를 직접 확인할 수 없으므로, 본인이 캡처를 해석하여 질문을 구성하는 단계 자체가 학습의 일부다

### 5.7 학습 자료

**필독 자료**
- [Ray Tracing Gems I (무료 PDF)](https://www.realtimerendering.com/raytracinggems/)
- [Ray Tracing Gems II (무료 PDF)](https://www.realtimerendering.com/raytracinggems/rtg2/)
- [Microsoft DirectX-Graphics-Samples — D3D12 Raytracing](https://github.com/microsoft/DirectX-Graphics-Samples/tree/master/Samples/Desktop/D3D12Raytracing)

**튜토리얼**
- [Chris Wyman — A Gentle Introduction to DXR](https://intro-to-dxr.cwyman.org/)
- [NVIDIA DXR Tutorial](https://developer.nvidia.com/rtx/raytracing/dxr/tutorial/)
- [Microsoft DirectX Raytracing Spec](https://microsoft.github.io/DirectX-Specs/d3d/Raytracing.html)

**비교 학습용**
- [ChameleonRT (Will Usher)](https://github.com/Twinklebear/ChameleonRT) — 동일한 path tracer를 DXR, Vulkan, OptiX, Metal 4개 백엔드로 구현
- [Will Usher — A Dive into Ray Tracing Performance on the Apple M1](https://www.willusher.io/graphics/2020/12/20/rt-dive-m1/)

**이론 자료 (병행 독서)**
- [Physically Based Rendering 4판 (online 무료)](https://pbr-book.org/) — 5.5.3 단계 진입 시 필수

### 5.8 Milestone

- [ ] DXR로 삼각형 렌더링
- [ ] DXR로 RTIOW final scene 재현 및 픽셀 단위 비교 검증
- [ ] Inline ray tracing 버전과 RT pipeline 버전을 모두 작성하여 비교

---

## 6. GitHub 워크플로우 및 AI 코드 리뷰

본 프로젝트는 포트폴리오 목적을 겸하므로 GitHub에 공개하며, PR 단위로 진행한다.

### 6.1 레포지토리 구조

단일 레포지토리(monorepo)에 서브디렉토리로 각 Phase를 구성하며, GitHub Actions의 `paths` 필터를 활용하여 Phase별로 다른 AI 리뷰 프롬프트를 적용한다.

```
ray-tracing-journey/
├── README.md                          # 전체 진행 대시보드
├── CLAUDE.md                          # 루트 Claude Code 작업 가이드
├── docs/
│   └── learning-plan.md               # 본 문서
├── .github/
│   └── workflows/
│       ├── phase1-review.yml          # paths: phase-1-rtiow/**
│       ├── phase2-review.yml          # paths: phase-2-rtnw/**
│       └── phase3-review.yml          # paths: phase-3-dxr/**
├── phase-1-rtiow/                     # Rust 독립 프로젝트
│   └── CLAUDE.md
├── phase-2-rtnw/                      # Rust 독립 프로젝트 (Phase 1에서 복사 시작)
│   └── CLAUDE.md
└── phase-3-dxr/                       # C++ 독립 프로젝트
    └── CLAUDE.md
```

각 워크플로우 파일은 6.9절에 명시된 dual-agent 구조(reviewer 2개 + synthesizer 1개)로 작성된다.

Submodule은 사용하지 않는다. 각 Phase는 독립 발전하지 않으며 시간순으로 진행되므로, monorepo 구조가 포트폴리오 내러티브에도 유리하다.

### 6.2 핵심 원칙

AI 리뷰는 코딩 중이 아닌 코딩 후에 일어난다. PR이 작성된 시점에는 학습자가 30분 룰을 통과하여 문제를 해결하였고, 의사결정이 코드에 반영된 상태다. 이 시점의 AI 코멘트는 학습 정석 루프(시도 → 결과 → 피드백 → 교정)의 피드백 단계에 정확히 부합한다.

### 6.3 절대 규칙

**금지 (학습 파괴 모드)**
- AI가 PR에 직접 commit을 추가하는 설정 (auto-fix)
- "Suggested changes"를 일괄 적용 (Apply suggestion 버튼)
- "전체적으로 더 idiomatic하게 리팩토링해달라" 유형의 광범위한 프롬프트

**허용 (학습 강화 모드)**
- AI는 코멘트만 작성한다. 코드 수정은 학습자가 직접 수행한다.
- AI에게 문제의 원인 설명을 요청한다. 패치는 요청하지 않는다.
- 좁고 구체적인 지적만 수용한다.

### 6.4 도구 선택

본 프로젝트는 두 개의 AI 에이전트를 reviewer로, 그리고 그 결과를 통합하는 synthesizer를 별도로 운영하는 dual-agent 구조를 채택한다.

- **Reviewer 1 — Claude Code**: Rust 언어 및 추상화 관점의 리뷰 담당
- **Reviewer 2 — Codex CLI**: 알고리즘 정확성 및 수치 측면의 리뷰 담당
- **Synthesizer — Codex CLI**: 두 reviewer의 결과를 통합하여 단일 코멘트로 정리

이 구조의 설계 근거 및 운영 방식은 6.9절에 상술한다. 두 reviewer 및 synthesizer는 GitHub Actions의 self-hosted runner(local runner)에서 실행되어 비용을 절감한다.

### 6.5 워크플로우 예시 (Phase 1)

`.github/workflows/phase1-review.yml`은 세 개의 job으로 구성된다.

```yaml
name: Phase 1 Dual-Agent Review

on:
  pull_request:
    paths:
      - 'phase-1-rtiow/**'

jobs:
  reviewer-rust:
    name: Claude Code (Rust 관점)
    runs-on: self-hosted
    steps:
      - uses: actions/checkout@v4
      - name: Run Claude Code review
        run: |
          # Claude Code CLI를 호출하여 Rust 관점 리뷰 수행
          # 결과를 review-rust.md로 저장
      - uses: actions/upload-artifact@v4
        with:
          name: review-rust
          path: review-rust.md

  reviewer-algorithm:
    name: Codex CLI (알고리즘 관점)
    runs-on: self-hosted
    steps:
      - uses: actions/checkout@v4
      - name: Run Codex CLI review
        run: |
          # Codex CLI를 호출하여 알고리즘 관점 리뷰 수행
          # 결과를 review-algorithm.md로 저장
      - uses: actions/upload-artifact@v4
        with:
          name: review-algorithm
          path: review-algorithm.md

  synthesizer:
    name: Codex CLI Synthesizer
    needs: [reviewer-rust, reviewer-algorithm]
    runs-on: self-hosted
    steps:
      - uses: actions/download-artifact@v4
        with:
          name: review-rust
      - uses: actions/download-artifact@v4
        with:
          name: review-algorithm
      - name: Run Codex CLI synthesizer
        run: |
          # Codex CLI를 synthesizer 모드로 호출
          # 두 review 파일을 입력으로, 통합된 코멘트를 출력
      - name: Post synthesized review to PR
        run: |
          # GitHub API를 통해 통합 결과를 PR 코멘트로 게시
```

각 reviewer의 system prompt는 6.9절을 참조한다. Synthesizer prompt는 6.10절을 참조한다. Phase 2 및 3는 동일 구조에서 `paths` 및 prompt 내용만 변경한다.

### 6.6 리뷰 응답 규칙

- AI가 작성한 모든 코멘트에 대해 답글로 응답을 작성한다
  - 수용한 경우: "수용. 사유: ..."
  - 거절한 경우: "거절. 사유: ..."
- 답글 작성 완료 후에만 PR을 merge한다
- 응답 작성 단계 자체가 학습의 본질에 해당한다. 단순 "Resolved" 처리는 금지한다.

### 6.7 PR 단위 가이드

- 챕터 2~3개를 묶어 PR을 구성한다. 단위가 과도하게 작으면 trivial한 코멘트만 도출된다.
- 브랜치명 규약: `phase1/ch3-5-rays-and-camera` 형태
- PR 본문에 다음을 포함한다: 해당 단위의 학습 내용, 막혔던 부분, 의사결정 요약
- 1년 후 git log가 학습 기록으로 남는다.

### 6.8 포트폴리오 측면

PR 히스토리, AI 리뷰 코멘트, 학습자의 응답이 누적된 흐름은 "피드백을 수용하고 반영하는 개발자"라는 시그널을 명확하게 남긴다. README에 진행 현황 및 주요 PR 링크를 정리한다.

### 6.9 Dual-Agent Review Strategy

**채택 사유**

단일 AI reviewer를 사용할 경우 한 모델의 prior와 blind spot이 그대로 학습자에게 전달된다. 두 개의 reviewer를 운영함으로써 다음을 달성한다.

- 두 모델의 의견이 겹치는 항목은 신뢰도가 높은 지적으로 판별 가능
- 한 쪽만 지적한 항목은 학습자가 판단해야 할 회색 지대로 분류
- 두 모델이 반대 제안을 한 항목은 trade-off가 존재하는 설계 결정 지점으로 인식

**역할 분담**

두 reviewer의 prompt를 의도적으로 차별화하여 코멘트의 차원이 명시적으로 구분되도록 한다.

- **Claude Code (Rust 관점)**: ownership 설계, 불필요한 clone 및 alloc, trait 추상화의 idiomatic 여부, Rust naming 컨벤션, 에러 처리의 일관성. RT 알고리즘 자체에 대한 제안은 금지된다.
- **Codex CLI (알고리즘 관점)**: 부동소수점 비교, 0 나눗셈 또는 NaN 가능성, 벡터 정규화 시 0-length 처리, 샘플링 분포의 정확성, 명백한 RT 알고리즘 버그. Rust 관용구 측면은 다루지 않는다. "더 좋은 알고리즘" 제안은 금지된다.

**Reviewer system prompt 예시 (Claude Code, Phase 1 기준)**

```
당신은 Rust 관점에서 코드를 리뷰합니다.
다음 항목을 중심으로 살피되 알고리즘 자체에 대한 제안은 하지 마십시오.

- Ownership 설계의 적절성 (Box vs Arc vs 제네릭)
- 불필요한 clone, alloc, 또는 lifetime 명시
- trait 추상화의 idiomatic 여부
- 에러 처리의 일관성
- naming의 Rust 컨벤션 준수

다음 사항은 금지됩니다.
- 코드 수정안 직접 제시
- 외부 수학 라이브러리(nalgebra, glam 등) 사용 제안
- RT 알고리즘 측면 언급
```

**Reviewer system prompt 예시 (Codex CLI, Phase 1 기준)**

```
당신은 수치 정확성 및 알고리즘 측면에서 코드를 리뷰합니다.
다음 항목을 중심으로 살피되 Rust 관용구는 다루지 마십시오.

- 부동소수점 비교, 0 나눗셈, NaN 가능성
- 벡터 정규화 시 0-length 처리 누락
- 샘플링 분포의 정확성
- random 시드 사용 패턴
- 명백한 RT 알고리즘 버그

다음 사항은 금지됩니다.
- 코드 수정안 직접 제시
- "더 좋은 알고리즘" 제안 (학습자가 도서를 통해 진행 중)
- Rust 관용구나 ownership 설계 언급
```

Phase 2 및 3의 prompt는 동일 구조에서 학습 대상 영역(Phase 2의 BVH, Phase 3의 BLAS/TLAS, SBT 등)을 추가로 강조하여 작성한다.

### 6.10 Synthesizer Design

두 reviewer의 결과를 통합하는 synthesizer는 Codex CLI로 실행한다. 학습자가 보유한 무제한 Codex 사용권을 활용함으로써 비용 부담 없이 운영한다.

**핵심 설계 원칙**

Synthesizer는 코드 자체를 리뷰하지 않는다. 두 reviewer의 출력 텍스트만을 입력으로 받아 정리한다. 이로 인해 두 reviewer의 의견을 임의로 변경하거나 새 의견을 추가할 가능성이 차단된다.

또한 합의된 의견은 통합하되 **충돌하는 의견은 통합하지 않고 그대로 노출**한다. 충돌 지점이 곧 학습자가 trade-off를 판단해야 하는 학습의 핵심 지점이므로, synthesizer가 임의로 한 쪽을 선택하지 않는다.

**Synthesizer system prompt**

```
당신은 두 AI 리뷰어의 코드 리뷰 결과를 통합하는 정리자입니다.
코드 자체를 리뷰하지 마십시오. 두 입력 리뷰만을 분석합니다.

다음 세 카테고리로 출력을 구성하십시오.

## Agreed (both reviewers flagged)
두 리뷰어가 같은 또는 매우 유사한 지적을 한 항목.
하나의 통합된 코멘트로 정리하되 원본의 핵심 표현은 보존합니다.

## Single-reviewer observations
한 리뷰어만 지적한 항목.
원본을 그대로 인용하고 [Claude only] 또는 [Codex only] 태그를 붙입니다.
학습자 판단이 필요한 회색 지대임을 명시합니다.

## Conflicts (learner decision required)
같은 코드에 대해 서로 다른 또는 반대되는 제안을 한 항목.
두 의견을 나란히 인용하고 [Conflict] 태그를 붙입니다.
어느 쪽이 옳다는 판단은 절대 하지 마십시오.
이 영역은 학습자가 trade-off를 명시적으로 판단해야 하는 학습 지점입니다.

규칙:
- 새 의견을 추가하지 마십시오
- 리뷰어들의 표현을 가능한 한 보존하십시오
- 통합을 위해 의미를 변경하지 마십시오
- 학습자에게 "어느 쪽을 따르라" 식의 지시 금지
```

**응답 워크플로우**

학습자는 synthesized 코멘트를 받은 후 다음 순서로 처리한다.

1. Conflicts 섹션부터 처리. 본인의 결정과 근거를 답글로 작성한다. 가장 학습 가치가 큰 항목이다.
2. Single-reviewer observations를 처리. 본인이 동의 여부를 답글로 명시한다.
3. Agreed 섹션은 (대체로 명백한 수정사항이므로) 빠르게 처리한다.

이 순서는 학습 가치가 높은 항목부터 처리하도록 설계되었다.

---

## 7. 부록: 선택 확장 트랙

### 7.1 Ray Tracing: The Rest of Your Life

- Phase 1 및 2 완료 후 시간적 여유가 있고 이론에 관심이 있을 경우 진행한다.
- Importance sampling, Monte Carlo integration 등 path tracing의 통계적 기반을 다룬다.
- 5.5.3 단계와 시너지가 크다.
- [원본](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

### 7.2 WebGPU 포팅

- Phase 3 완료 후 결과물 공유 목적으로 진행한다.
- WGSL 컴퓨트 셰이더로 path tracer를 작성한다 (HW RT가 아니며 BVH를 직접 traverse한다).
- 브라우저에서 즉시 시연 가능하여 포트폴리오에 유리하다.
- 참고: [Nelarius/weekend-raytracer-wgpu](https://github.com/Nelarius/weekend-raytracer-wgpu)

### 7.3 Metal RT 포팅

- Phase 3 완료 후 비교 학습 목적으로 진행한다.
- Apple Silicon M3 이상 또는 A17 Pro 이상이 필요하다.
- Metal은 SBT 없이 inline ray tracing 중심으로 구성되어 있어, DXR과 다른 추상화 철학을 비교할 수 있다.
- 참고: WWDC20 "Discover Ray Tracing with Metal", [Apple 공식 샘플](https://developer.apple.com/documentation/Metal/accelerating-ray-tracing-using-metal)

---

## 8. 시간 예산

| Phase | 단계 | 최소 | 여유 |
|---|---|---|---|
| 1 | Pre-Phase 1 | 0.5일 | 1일 |
| 1 | 본 학습 (RTIOW) | 1주 | 2주 |
| 2 | RTNW | 2주 | 3주 |
| 3 | Pre-Phase 3 | 3일 | 1주 |
| 3 | 본 학습 (DXR) | 3주 | 7주 이상 |
| 합계 | | 약 7주 | 약 13주 이상 |

평일 저녁 및 주말 사이드 프로젝트 기준이다. 진행을 서두르지 않는다. 특히 Phase 2의 BVH와 Phase 3의 SBT에는 충분한 시간을 할애한다.

---

## 9. 자기 점검 질문

각 Phase 종료 시 다음 질문에 답할 수 있어야 한다.

### 9.1 Phase 1 종료 시
- Lambertian 머티리얼과 Metal 머티리얼의 산란 모델 차이를 설명할 수 있는가
- Defocus blur가 카메라의 aperture 크기와 어떤 관계가 있는가
- `Box<dyn Material>` 대신 `Arc<dyn Material>`을 선택한 사유는 무엇인가

### 9.2 Phase 2 종료 시
- BVH 적용 전후 ray-scene intersection의 시간 복잡도는 각각 어떻게 되는가
- Importance sampling이란 무엇이며 왜 필요한가
- Instance와 BVH는 어떻게 결합되는가 (TLAS 구조와의 연관성)

### 9.3 Phase 3 종료 시
- BLAS와 TLAS를 분리한 사유는 무엇인가
- SBT의 record란 무엇이며 왜 필요한가
- Inline ray tracing과 RT pipeline의 트레이드오프는 무엇인가
- closesthit과 anyhit은 각각 어떤 조건에서 호출되는가
- Phase 2에서 직접 구현한 BVH와 DXR의 acceleration structure 간의 대응 관계를 설명할 수 있는가

---

본 문서는 학습 진행에 따라 갱신한다.
