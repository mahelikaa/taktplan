taktplan

A production scheduling tool that turns messy ERP exports into clean, validated and audited data before generating schedules.

Current Progress

Duration Parser — Implemented and tested in Rust.

Supports:

* Numeric durations with configurable default units (90)
* Explicit units: seconds (2700s), minutes (45 min) and hours (1.5h)
* Decimal and comma notation (1.5h, 1,5 h)
* Hours-and-minutes notation (1:30)
* Input validation, overflow protection and descriptive errors

Roadmap

* Duration parser
* Work-center ingestion with data audit reports
* Routings and production orders
* Finite-capacity scheduler

Running Tests

cargo test