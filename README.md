# Vector Index Engine

    Compact vector index with cosine scoring and top-k search.

    ## Why this exists

    This repository is a compact implementation of a production-flavored system component. It focuses on clear domain boundaries, deterministic behavior and readable extension points instead of framework noise.

    ## Highlights

    - Small core with explicit domain types.
    - Deterministic sample data or simulation path.
    - Separation between parsing, business logic and output formatting.
    - No generated artifacts checked in.

    ## Run

    ```bash
    cargo test
cargo run
    ```

    ## Notes

    The code is intentionally local-first so it can be reviewed quickly. The structure leaves room for storage adapters, HTTP handlers, persistence and automated tests without rewriting the core.
