<h1 align="center">ACTIVEBREACH-ENGINE</h1>
<p align="center"><b>Protected Dual Syscall Execution Framework</b></p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white&style=for-the-badge" />
  <img src="https://img.shields.io/badge/C-00599C?logo=c&logoColor=white&style=for-the-badge" />
  <img src="https://img.shields.io/badge/C%2B%2B-00599C?logo=c%2B%2B&logoColor=white&style=for-the-badge" />
</p>

<p align="center">
  <a href="https://titansoftwork.com/insight/syscall_execution/">Read the Technical Article</a>
</p>

**ActiveBreach-Engine (ABE)** is a Windows execution capability platform designed to execute secured & direct system calls in heavily instrumented environments, protecting your process from external attackers and process hooking.

**ABE** provides a controlled, fully dynamic framework for executing system calls without reliance on user-mode API invocation or resident `ntdll.dll` code paths, while also protecting said syscall stubs from hijacking or modification by an external attacker.

This project was put together as a successor-class capability to historical syscall research tooling (e.g., SysWhispers and Hell’s Gate), addressing the limitations, static assumptions, and detectability issues inherent in earlier designs.

## SCOPE

Modern debugging & instrumentation tools rely on *API hooking* and *breakpoints*, possibly the easiest way to observe a processes behaviour is by simply setting breakpoints on codepaths you're interested in. There's an endless cat and mouse chase between anti-debug, control-flow obfuscation, indirection, lifting etc... Activebreach is focused on preventing the hook itself from exposing your progrma.

The most common API hooks are on ``ntdll.dll``, which is the system call boundary in userland which every system-call requiring API (eg; ``OpenProcess``) eventually hits. This is of course the perfect place for a hook, luckily these ``ntdll.dll`` stubs are also extremely generic and easily copyable.

**ABE** builds upon this, building its own protected, encrypted & optimized stub rings that it uses to execute secured system calls, without any sort of reliance on ``ntdll.dll`` or other DLL's, **ABE** operates assuming everything in the environment is hostile, using its own dedicated dispatcher thread, and multiple anti-debug & protection mechanisms to prevent *your* system calls.

![Hooking Diagram](./Diagram/AB_DIRECT_SYSCALL.png)

## USE

For ease of integration, **ABE** is provided in three trims: C, C++, and Rust. Rust is the most technically advanced implementation, while C++ offers an integrated debugger.

### Why three versions?

Primarily due to integration complexity. Linking cryptographic libraries and using Windows internal structures in C++ introduces development friction and unnecessary complexity. The goal of **ABE** is ease of integration, which means no external dependencies. As a result, the C and C++ versions are provided as single-include header files (`.h`).

The Rust version includes exclusive features such as build-time encrypted stub templates, a custom stub ring allocator, and TLS callbacks.

## RUST INTEGRATION & FFI

The Rust SDK is modular and portable:

- Native Rust integration (path dependency or crate dependency)
- C ABI integration via `activebreach.dll` (runtime loading or import-lib linking)
- Static linking via `activebreach.lib`

This C ABI surface is intentionally language-agnostic and can be consumed from C/C++, C#, Zig, Nim, Odin, D, Python (`ctypes`/`cffi`), and similar FFI-capable runtimes.

Build outputs from `SDK/Rust/target/{debug,release}` include:
- `libactivebreach.rlib` (Rust-to-Rust)
- `activebreach.dll` (shared library, C ABI)
- `activebreach.dll.lib` (MSVC import library for the DLL)
- `activebreach.lib` (static library)

C ABI header:
- `SDK/Rust/include/activebreach.h`

Exported symbols:
- `activebreach_launch`
- `ab_call`
- `ab_violation_count`
- `ab_set_violation_handler`
- `ab_clear_violation_handler`

Integration models on Windows:
1. DLL only (runtime dynamic loading with `LoadLibrary`/`GetProcAddress`)
2. DLL + import LIB (implicit link with `activebreach.dll.lib`)
3. Static LIB (`activebreach.lib`, no runtime DLL dependency for this library)

## USAGE

See [Usage Overview](./USAGE.md)

## RUST FEATURE MODES

The Rust SDK ships with the following feature flags:

- `secure` (default): Stub pages are protected at rest with `PAGE_NOACCESS` and protections are flipped during acquire/patch/execute/release. This reduces cleartext stub exposure in memory at the cost of more `VirtualProtect` transitions. This is intended for operators who want to maximize in-process security and want absolute certainty that their syscalls will not be tampered with.
- `stealth`: Marker feature for "no `secure`". To actually run without protection flips, build with `--no-default-features` (and optionally add `--features stealth` to make the intent explicit). In this mode stub pages remain writable/executable, reducing flip noise but leaving stubs more exposed in memory. This is intended for operators who want to minimize page-flipping visibility
- `long_sleep`: Enables an idle teardown path in the dispatcher. After a configurable idle interval (default 30_000ms), the dispatcher drops the stub pool and syscall table, then blocks until new work arrives. The public API `ab_set_long_sleep_idle_ms(ms)` is only available with this feature. This is intended for operators developing **long-living** or **LOTL** processes who want to avoid memory-scanners.
- `ntdll_backend`: Prefer jumping into an intact loaded-`ntdll.dll` syscall prologue (inside `ntdll` `.text`) instead of issuing `syscall` from the ActiveBreach stub. If the loaded export stub/prologue is detected as hooked or invalid, the dispatcher falls back to the direct-syscall stub. Stack spoofing is disabled in this backend (no synthetic return chain is constructed). This is intended for operators who want minimal EDR foresnsics and are operating in a heavily instrumented environment.

Example builds:

- Default (recommended baseline):
  - `cargo build`
- Stealth-ish (no `secure` protection flips):
  - `cargo build --no-default-features --features stealth`
- Long-idle friendly:
  - `cargo build --features long_sleep`
- Prefer loaded-NTDLL prologues when intact:
  - `cargo build --features ntdll_backend`

## License

Copyright © 2026 TITAN Softwork Solutions

Licensed under the Apache License, Version 2.0 (the "License") **with the Commons Clause License Condition v1.0**.

- You may not use this software for commercial purposes ("Sell the Software" as defined in the Commons Clause).
- Full text is provided in `LICENSE`.

Apache 2.0: <http://www.apache.org/licenses/LICENSE-2.0>  
Commons Clause details: <https://commonsclause.com/>
