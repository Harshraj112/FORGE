# FORGE

**Forensic Operations & Runtime for Guided Examination**

FORGE is an open-source, cross-platform forensic runtime and analysis framework designed for **automated digital forensics, evidence collection, malicious-activity analysis, and investigation automation**.

The long-term goal is to develop a specialized programming language and runtime in which **digital forensic evidence is a first-class concept**.

> **Collect → Normalize → Correlate → Analyze → Detect → Investigate**

---

## Project Status

🚧 **Early Development**

The current focus is building the **forensic core and runtime infrastructure**.

The programming language, virtual machine, advanced detection engine, and cross-platform collectors will be developed incrementally.

---

## Vision

Modern forensic investigations often require multiple specialized tools for endpoint collection, network analysis, file analysis, memory analysis, and detection.

FORGE aims to provide a unified programmable layer that can:

* Collect forensic evidence from multiple sources
* Integrate existing cybersecurity and forensic tools
* Normalize heterogeneous data into a common evidence model
* Preserve evidence provenance and integrity
* Correlate events across different evidence sources
* Build investigation timelines and evidence graphs
* Detect suspicious activity using rules and analytical models
* Automate repeatable forensic investigations
* Provide a domain-specific forensic programming language

---

## Architecture

The planned architecture is:

```text
                         FORGE
                           │
              ┌────────────┴────────────┐
              │                         │
       Forensic Runtime          Future Language
              │                         │
              └────────────┬────────────┘
                           │
                    Evidence Model
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
   Collectors           Adapters           Analysis
       │                   │                   │
       │            ┌──────┼──────┐            │
       │            │      │      │            │
      OS           Nmap   YARA   Other        │
   Collectors             Tools               │
       │            │      │      │            │
       └────────────┴──────┴──────┴────────────┘
                           │
                    Evidence Store
                           │
                 ┌─────────┴─────────┐
                 │                   │
             Timeline              Graph
                 │                   │
                 └─────────┬─────────┘
                           │
                     Detection
                           │
                       Findings
                           │
                        Reports
```

---

## Core Design Principles

### 1. Evidence First

FORGE defines its own common evidence model instead of tying analysis directly to a particular external tool.

For example:

```text
Process
File
NetworkHost
NetworkService
NetworkConnection
User
Event
Artifact
Relationship
Finding
```

---

### 2. Tool Agnostic

Existing cybersecurity tools can be integrated through adapters.

For example:

```text
Nmap
   ↓
Nmap Adapter
   ↓
FORGE Network Evidence
```

Similarly, future integrations may include:

```text
YARA
Sigma
Volatility
Zeek
osquery
Velociraptor
```

The core analysis engine should operate on **FORGE evidence**, not on vendor-specific output formats.

---

### 3. Cross Platform

The long-term target is:

```text
Windows
Linux
macOS
```

Platform-specific implementations should remain behind common interfaces.

```text
                ProcessCollector
                     ▲
          ┌──────────┼──────────┐
          │          │          │
        Linux      Windows     macOS
```

This allows the same investigation logic to operate across different operating systems.

---

### 4. Provenance and Integrity

Every important evidence item should preserve information about:

```text
Source
Collector
Collector Version
Collection Time
Evidence Hash
Relationships
```

This allows an investigator to understand where evidence came from and how a finding was produced.

---

### 5. Least-Privilege Architecture

Collectors and plugins should request only the capabilities they require.

Example:

```text
process.read
filesystem.read
network.read
```

The runtime can then enforce capability and permission boundaries.

---

# Repository Structure

The project is organized as a Rust workspace:

```text
Forge/
│
├── crates/
│   ├── forge-core/
│   ├── forge-evidence/
│   ├── forge-collectors/
│   ├── forge-platform/
│   ├── forge-storage/
│   ├── forge-provenance/
│   ├── forge-analysis/
│   ├── forge-detection/
│   ├── forge-graph/
│   ├── forge-security/
│   ├── forge-plugin/
│   ├── forge-adapters/
│   ├── forge-runtime/
│   ├── forge-lexer/
│   ├── forge-parser/
│   ├── forge-vm/
│   └── forge-cli/
│
├── collectors/
│   ├── linux/
│   ├── windows/
│   └── macos/
│
├── rules/
├── stdlib/
├── examples/
├── tests/
├── benchmarks/
├── tools/
├── fixtures/
└── docs/
```

Some components are currently placeholders and will be implemented progressively.

---

# External Tool Integration

FORGE is not intended to reinvent every existing forensic capability.

Instead, external tools can act as **evidence sources**.

For example, Nmap:

```text
FORGE CLI
    │
    ▼
Nmap Adapter
    │
    ▼
Nmap
    │
    ▼
XML Output
    │
    ▼
Parser
    │
    ▼
Network Evidence
    │
    ▼
FORGE Evidence Store
```

The first Nmap integration uses structured XML output rather than scraping terminal output.

Example:

```bash
nmap -oX result.xml localhost
```

The eventual FORGE interface will abstract this implementation:

```bash
forge network scan localhost
```

---

# Development Roadmap

## Phase 1 — Forensic Core

* [ ] Evidence data model
* [ ] Process evidence
* [ ] File evidence
* [ ] Network evidence
* [ ] Event model
* [ ] Evidence serialization

## Phase 2 — Collection

* [ ] Linux process collector
* [ ] Linux filesystem collector
* [ ] Linux network collector
* [ ] Windows collectors
* [ ] macOS collectors

## Phase 3 — Tool Adapters

* [ ] Nmap adapter
* [ ] YARA adapter
* [ ] Sigma integration
* [ ] Zeek adapter
* [ ] Volatility integration
* [ ] osquery integration

## Phase 4 — Evidence Infrastructure

* [ ] SQLite evidence store
* [ ] Evidence hashing
* [ ] Provenance tracking
* [ ] Evidence relationships
* [ ] Chain-of-custody metadata

## Phase 5 — Analysis

* [ ] Query engine
* [ ] Filtering
* [ ] Aggregation
* [ ] Timeline engine
* [ ] Correlation engine
* [ ] Evidence graph

## Phase 6 — Detection

* [ ] Detection rule engine
* [ ] Indicator matching
* [ ] Behavioral correlation
* [ ] Anomaly detection
* [ ] Explainable findings

## Phase 7 — Forensic Runtime

* [ ] Runtime API
* [ ] Execution context
* [ ] Capability system
* [ ] Plugin architecture
* [ ] Sandboxed extensions

## Phase 8 — FORGE Language

* [ ] Lexer
* [ ] Parser
* [ ] AST
* [ ] Semantic analysis
* [ ] Forensic types
* [ ] Runtime bindings
* [ ] Standard library

## Phase 9 — Virtual Machine

* [ ] Intermediate representation
* [ ] Bytecode
* [ ] Virtual machine
* [ ] Runtime optimization

---

# Example Future Investigation

A future FORGE program may look conceptually like:

```forge
investigate host {

    collect processes
    collect network
    collect files

    correlate processes with network

    detect suspicious_activity

    generate report
}
```

The runtime would translate this into:

```text
Investigation
     │
     ├── Collection
     │
     ├── Normalization
     │
     ├── Correlation
     │
     ├── Detection
     │
     └── Reporting
```

The exact language syntax is still under development.

---

# Example Evidence Flow

A network investigation might produce:

```text
Nmap
 │
 ▼
Network Host
 │
 ├── Port 22
 ├── Port 80
 └── Port 443
        │
        ▼
FORGE Evidence
        │
        ▼
Correlation Engine
        │
        ▼
Investigation Graph
        │
        ▼
Finding
```

The finding retains references to the underlying evidence so that the investigator can trace the conclusion back to its source.

---

# Technology Stack

The current planned stack is:

| Component            | Technology                      |
| -------------------- | ------------------------------- |
| Core Runtime         | Rust                            |
| CLI                  | Rust                            |
| Evidence Storage     | SQLite / JSON                   |
| Serialization        | JSON / CBOR                     |
| Platform Integration | Native OS APIs                  |
| Network Integration  | Nmap and other adapters         |
| Detection            | Rule engine + analytical models |
| ML Integration       | ONNX Runtime                    |
| Future Language      | Custom DSL                      |
| Future VM            | Custom bytecode VM              |

---

# Security Scope

FORGE is intended for **defensive security, authorized forensic investigation, incident response, and security research**.

External scanners and analysis tools should only be used against systems and networks for which the operator has authorization.

The project focuses on:

* Evidence collection
* Incident investigation
* Threat detection
* Behavioral analysis
* Evidence correlation
* Forensic automation
* Reproducible analysis

It is not designed to provide mechanisms for bypassing security controls or evading endpoint defenses.

---

# Contributing

The project is currently in the architectural and early implementation stage.

Areas that will eventually need contributions include:

* Evidence modeling
* Rust runtime development
* OS-specific collectors
* Tool adapters
* Detection rules
* Graph analysis
* Forensic research
* Language design
* Testing and benchmarking

---

# License

License: **To be determined**

---

## Project Goal

The long-term objective of FORGE is to provide a programmable forensic environment where investigators can write high-level investigation logic while the runtime handles evidence collection, normalization, correlation, analysis, and reporting.

```text
                 FORGE

        Collect → Normalize
                   ↓
              Correlate
                   ↓
                Analyze
                   ↓
                Detect
                   ↓
              Investigate
                   ↓
                Report
```

**FORGE — Forensic Operations & Runtime for Guided Examination**
# FORGE
