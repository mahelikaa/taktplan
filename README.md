# taktplan

A production scheduling tool that turns messy ERP exports into clean, validated, and audited data before generating schedules.

## Current Progress

### Duration Parser

Implemented and tested in Rust.

**Supported formats:**
- Numeric durations with configurable default units: `90`
- Seconds: `2700s`
- Minutes: `45 min`, `45m`
- Hours: `1.5h`, `1.5 hrs`
- Decimal and comma notation: `1.5h`, `1,5 h`
- Hours and minutes: `1:30`

Includes input validation, overflow protection, and descriptive errors.

## Roadmap
- [x] Duration parser
- [ ] Work-center ingest (Odoo export) with data-audit report
- [ ] Routings, orders, shift calendars + cross-file checks
- [ ] Finite-capacity scheduler with KPIs
- [ ] Minimal-change replanning
- [ ] HTTP API, Docker, Kubernetes (Helm)

## Running Tests

```bash
cargo test
```
