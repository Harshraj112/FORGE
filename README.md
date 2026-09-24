# FORGE

## Forensic Operations & Runtime for Guided Examination

**FORGE** is a cross-platform, domain-specific programming language and runtime designed for **automated digital forensics, malicious-activity analysis, evidence correlation, and investigation automation**.

FORGE combines a programming language, execution runtime, forensic evidence model, OS collectors, security-tool adapters, analysis engines, and detection capabilities into one programmable environment.

> **Write forensic logic once. Collect → Normalize → Correlate → Analyze → Detect → Report.**

---

## Vision

Traditional digital-forensic workflows often require investigators to manually operate multiple specialized tools and then correlate their outputs.

FORGE aims to provide a programmable abstraction over this process.

Instead of writing separate scripts for every tool, operating system, and evidence source, investigators will eventually be able to express an investigation directly in the FORGE language:

```forge
case incident_001 {

    collect processes
    collect network
    collect filesystem

    correlate processes with network

    detect suspicious_activity

    report {
        timeline
        findings
        evidence
    }
}
```

The FORGE runtime executes this investigation and handles the underlying collection, normalization, analysis, and reporting.

---

# Core Concept

FORGE consists of two major layers:

```text
                         FORGE
                           │
             ┌─────────────┴─────────────┐
             │                           │
       FORGE LANGUAGE              FORGE RUNTIME
             │                           │
       Source Programs              Execution Engine
             │                           │
       Lexer / Parser              Evidence APIs
             │                           │
            AST                    Collectors
             │                           │
      Semantic Analysis            Tool Adapters
             │                           │
        IR / Bytecode              Analysis Engine
             │                           │
             └─────────────┬─────────────┘
                           │
                        FORGE VM
                           │
                           ▼
                  Forensic Investigation
```

### FORGE Language

Provides the syntax investigators use to describe:

* Evidence collection
* Queries
* Correlation
* Detection rules
* Timeline analysis
* Investigation workflows
* Reporting

### FORGE Runtime

Provides the underlying capabilities required to execute those programs:

* Evidence management
* OS interaction
* Tool integration
* Analysis
* Correlation
* Graph construction
* Detection
* Security controls
* Provenance tracking

---

# Architecture

```text
                         FORGE PROGRAM
                              │
                              ▼
                           Lexer
                              │
                              ▼
                           Parser
                              │
                              ▼
                             AST
                              │
                              ▼
                    Semantic Analysis
                              │
                              ▼
                          FORGE IR
                              │
                              ▼
                         Bytecode
                              │
                              ▼
                           FORGE VM
                              │
                              ▼
                       FORGE Runtime
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
          ▼                   ▼                   ▼
      Collectors          Adapters            Analysis
          │                   │                   │
    ┌─────┼─────┐       ┌─────┼─────┐       ┌───┼────┐
    │     │     │       │     │     │       │   │    │
 Linux Windows macOS   Nmap  YARA  Other   Query Graph Rules
    │     │     │       │     │     │       │   │    │
    └─────┴─────┴───────┴─────┴─────┴───────┴───┴────┘
                              │
                              ▼
                       Evidence Model
                              │
                              ▼
                       Evidence Storage
                              │
                              ▼
                     Investigation Results
                              │
                              ▼
                           Reports
```

---

# 1. FORGE Language

The language is designed specifically around forensic concepts.

Instead of treating everything as generic strings, arrays, and objects, FORGE will eventually provide domain-specific types such as:

```text
Process
File
NetworkHost
NetworkService
NetworkConnection
User
Event
Artifact
TimelineEvent
Relationship
Evidence
Finding
```

Example:

```forge
processes
    | where parent == "powershell.exe"
    | select pid, name, executable
```

A more complete investigation:

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

The exact language syntax is under development.

---

# 2. FORGE Compiler Pipeline

The language will eventually follow a conventional compiler/interpreter architecture:

```text
FORGE Source
     │
     ▼
   Lexer
     │
     ▼
   Tokens
     │
     ▼
   Parser
     │
     ▼
    AST
     │
     ▼
Semantic Analysis
     │
     ▼
   FORGE IR
     │
     ▼
  Bytecode
     │
     ▼
  FORGE VM
```

The initial implementation may use an interpreter before introducing bytecode and VM optimizations.

---

# 3. FORGE Runtime

The runtime is the execution layer underneath the language.

It provides:

```text
Runtime
│
├── Execution Context
├── Values
├── Environments
├── Functions
├── Evidence APIs
├── Collector APIs
├── Analysis APIs
├── Security / Capabilities
└── Plugin APIs
```

The runtime allows FORGE programs to interact with the forensic subsystem without directly depending on operating-system-specific implementations.

---

# 4. Evidence Model

Evidence is a first-class concept in FORGE.

The runtime will normalize information from different sources into a common model.

For example:

```text
Process
│
├── PID
├── Parent PID
├── Name
├── Executable
├── Command Line
├── User
├── Timestamp
└── Provenance
```

Network evidence:

```text
NetworkService
│
├── Host
├── Port
├── Protocol
├── State
├── Service
└── Provenance
```

This allows different tools to contribute to the same evidence model.

---

# 5. Cross-Platform Collectors

FORGE is designed to support:

```text
Linux
Windows
macOS
```

Platform-specific implementations will be hidden behind common interfaces.

For example:

```text
                 ProcessCollector
                       ▲
             ┌─────────┼─────────┐
             │         │         │
           Linux     Windows    macOS
             │         │         │
           /proc     OS APIs    OS APIs
```

A FORGE program can therefore request:

```forge
collect processes
```

without needing to know how each operating system exposes process information.

---

# 6. External Security Tool Integration

FORGE is not intended to reinvent every existing cybersecurity tool.

Instead, external tools can be integrated through adapters.

The general architecture is:

```text
External Tool
      │
      ▼
   Adapter
      │
      ▼
    Parser
      │
      ▼
  Normalization
      │
      ▼
FORGE Evidence
```

Potential integrations include:

```text
Nmap
YARA
Sigma
Volatility
Zeek
osquery
Velociraptor
```

### Example: Nmap

```text
FORGE
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
XML Parser
  │
  ▼
Network Evidence
```

The analysis engine operates on `NetworkHost`, `NetworkService`, and related FORGE objects rather than directly depending on Nmap's output format.

---

# 7. Evidence Provenance

Forensic evidence must remain traceable.

Each evidence item should preserve information such as:

```text
Source
Collector
Tool
Tool Version
Collection Timestamp
Evidence Hash
Relationships
```

Example:

```json
{
  "type": "network_service",
  "host": "192.168.1.10",
  "port": 22,
  "protocol": "tcp",
  "state": "open",
  "provenance": {
    "source": "nmap",
    "collector_version": "0.1.0"
  }
}
```

This allows findings to be traced back to their underlying evidence.

---

# 8. Analysis Engine

The analysis engine operates on normalized evidence.

Initial capabilities:

```text
Filtering
Selection
Sorting
Aggregation
Joining
Correlation
Timeline analysis
Graph traversal
```

Example:

```text
Process
   │
   ├── creates ──► File
   │
   └── connects ─► Network
```

FORGE can eventually represent this as an evidence graph:

```text
User
 │
 ▼
Process
 │
 ├────────► File
 │
 ├────────► Network Connection
 │
 └────────► Child Process
```

This enables multi-source investigation rather than isolated event analysis.

---

# 9. Detection Engine

The detection engine will allow investigators to define behavioral rules.

Conceptually:

```forge
rule suspicious_chain {

    when
        process.created
        followed_by file.created
        followed_by network.connection

    within 60 seconds

    emit finding
}
```

The detection system can eventually combine:

```text
Rules
+
Indicators
+
Correlation
+
Statistical Analysis
+
Machine Learning
```

The goal is to produce explainable findings with references to the evidence that produced them.

---

# 10. Security Model

FORGE will use a capability-oriented architecture.

Components should request only the permissions they need.

Example:

```text
process.read
filesystem.read
network.read
```

The runtime can then enforce:

```text
Program
  │
  ▼
Capability Request
  │
  ▼
Permission Check
  │
  ├── Allowed
  │
  └── Denied
```

This is intended to make forensic automation more controlled, auditable, and suitable for defensive environments.

---

# 11. Project Structure

```text
Forge/
│
├── crates/
│   │
│   ├── forge-language/
│   │   ├── lexer/
│   │   ├── parser/
│   │   ├── ast/
│   │   ├── semantic/
│   │   └── ir/
│   │
│   ├── forge-vm/
│   │   ├── bytecode/
│   │   ├── interpreter/
│   │   └── execution/
│   │
│   ├── forge-runtime/
│   │   ├── context/
│   │   ├── values/
│   │   ├── functions/
│   │   └── environment/
│   │
│   ├── forge-evidence/
│   │   ├── process/
│   │   ├── file/
│   │   ├── network/
│   │   ├── event/
│   │   └── finding/
│   │
│   ├── forge-collectors/
│   │
│   ├── forge-platform/
│   │   ├── linux/
│   │   ├── windows/
│   │   └── macos/
│   │
│   ├── forge-adapters/
│   │   ├── nmap/
│   │   ├── yara/
│   │   ├── sigma/
│   │   └── volatility/
│   │
│   ├── forge-analysis/
│   │   ├── query/
│   │   ├── correlation/
│   │   ├── timeline/
│   │   └── anomaly/
│   │
│   ├── forge-detection/
│   ├── forge-graph/
│   ├── forge-storage/
│   ├── forge-security/
│   └── forge-cli/
│
├── stdlib/
│   └── forensic/
│
├── rules/
├── examples/
├── tests/
├── benchmarks/
├── fixtures/
└── docs/
    ├── language/
    ├── runtime/
    └── architecture/
```

---

# 12. Development Roadmap

## Phase 1 — Language & Runtime Foundation

* [ ] Define language specification
* [ ] Define forensic type system
* [ ] Design AST
* [ ] Implement lexer
* [ ] Implement parser
* [ ] Implement basic interpreter
* [ ] Implement runtime context

## Phase 2 — Forensic Core

* [ ] Evidence model
* [ ] Process model
* [ ] File model
* [ ] Network model
* [ ] Event model
* [ ] Finding model
* [ ] Provenance model

## Phase 3 — Collection

* [ ] Linux process collector
* [ ] Linux filesystem collector
* [ ] Linux network collector
* [ ] Windows collectors
* [ ] macOS collectors

## Phase 4 — External Tool Adapters

* [ ] Nmap
* [ ] YARA
* [ ] Sigma
* [ ] Zeek
* [ ] Volatility
* [ ] osquery
* [ ] Velociraptor

## Phase 5 — Analysis

* [ ] Query engine
* [ ] Correlation engine
* [ ] Timeline engine
* [ ] Evidence graph
* [ ] Relationship analysis

## Phase 6 — Detection

* [ ] Detection rule engine
* [ ] Indicator matching
* [ ] Behavioral detection
* [ ] Anomaly detection
* [ ] Explainable findings

## Phase 7 — VM

* [ ] Intermediate representation
* [ ] Bytecode
* [ ] Virtual machine
* [ ] Runtime optimization

## Phase 8 — Ecosystem

* [ ] Standard library
* [ ] Plugin API
* [ ] Documentation
* [ ] Testing framework
* [ ] Investigation templates
* [ ] Reporting system

---

# 13. Example Future Workflow

An investigator could eventually write:

```forge
case incident_001 {

    host "workstation-01"

    collect processes
    collect network
    collect filesystem

    scan network using nmap

    correlate {
        processes
        files
        network
    }

    detect suspicious_activity

    report {
        timeline
        graph
        findings
        evidence
    }
}
```

Execution:

```text
investigation.forge
        │
        ▼
      Lexer
        │
        ▼
      Parser
        │
        ▼
       AST
        │
        ▼
    FORGE VM
        │
        ▼
   FORGE Runtime
        │
 ┌──────┼─────────┐
 ▼      ▼         ▼
OS    Nmap      Other
API   Adapter    Tools
 │      │         │
 └──────┴─────────┘
        │
        ▼
 Evidence Model
        │
        ▼
 Correlation Engine
        │
        ▼
 Detection Engine
        │
        ▼
 Investigation Report
```

---

# Technology Stack

| Component         | Technology           |
| ----------------- | -------------------- |
| Core Language     | Rust                 |
| Language Frontend | Custom lexer/parser  |
| Runtime           | Rust                 |
| Future VM         | Custom bytecode VM   |
| Evidence Storage  | SQLite + JSON        |
| Serialization     | JSON / CBOR          |
| OS Integration    | Native platform APIs |
| Network Analysis  | Nmap + adapters      |
| Detection         | Custom rule engine   |
| ML                | ONNX Runtime         |
| Testing           | Rust test framework  |

---

# Development Philosophy

FORGE will follow these principles:

### Language first, but incrementally

Build a small usable language before attempting a sophisticated VM.

### Runtime independent of operating systems

OS-specific functionality belongs behind interfaces.

### Evidence independent of tools

Nmap, YARA, Volatility, and other tools are data sources—not the foundation of the evidence model.

### Reproducible investigations

The same evidence and investigation logic should produce reproducible results.

### Explainable findings

A finding should be traceable to the evidence and rules that produced it.

### Security by design

Capabilities, permissions, provenance, and auditing should be part of the architecture rather than added later.

---

# Security Scope

FORGE is intended for:

* Digital forensics
* Incident response
* Defensive security research
* Authorized security analysis
* Evidence collection
* Threat detection
* Investigation automation
* Security analytics

External scanning and analysis capabilities must only be used against systems and networks for which the operator has authorization.

FORGE is not designed to bypass security controls or provide mechanisms for evading endpoint defenses.

---

# Project Status

🚧 **Early Development**

Current focus:

```text
FORGE Language Architecture
          ↓
FORGE Runtime
          ↓
Evidence Model
          ↓
Nmap / Tool Integration
          ↓
Analysis Engine
```

The language syntax, runtime APIs, evidence schema, and VM architecture are subject to change during development.

---

# Long-Term Goal

FORGE aims to become a **programmable forensic computing environment** where investigators can express complex investigations using a specialized language while the runtime handles the underlying collection, evidence management, analysis, correlation, detection, and reporting.

```text
                 FORGE

          Programming Language
                    │
                    ▼
                 FORGE VM
                    │
                    ▼
              FORGE Runtime
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
    Collection   Analysis    Detection
        │           │           │
        └───────────┼───────────┘
                    ▼
              Evidence Graph
                    │
                    ▼
             Investigation
                    │
                    ▼
                  Report
```

**FORGE — Forensic Operations & Runtime for Guided Examination**
