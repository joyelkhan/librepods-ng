# ADR 0002: Protocol Parser Design

## Status
Accepted

## Context
The AAP (Apple AirPods Protocol) has 15 message types with varying payloads. We need safe, efficient parsing without unsafe code.

## Decision
Use nom parser combinators for safe, zero-copy parsing.

## Rationale
- nom provides compile-time safety guarantees
- Zero-copy parsing for performance
- Composable parsers for maintainability
- No unsafe code required

## Consequences
- Positive: Type-safe, efficient, composable
- Negative: Learning curve for nom DSL
- Mitigation: Comprehensive documentation and examples

## Message Types
1. Battery Status (0x01)
2. ANC Control (0x02)
3. Ear Detection (0x03)
4. Firmware Info (0x04)
5. Spatial Audio (0x05)
6. Heart Rate (0x06)
7. Find My (0x07)
8. Conversation Awareness (0x08)
9. Hearing Aid (0x09)
10. Device Rename (0x0A)
11. Multipoint Control (0x0B)
12. Adaptive Transparency (0x0C)
13. Long Press Actions (0x0D)
14. Custom Transparency (0x0E)
15. Head Gestures (0x0F)
