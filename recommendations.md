# Recommendations for todo-rust-cli

## 1. Replace panics with user-facing errors
The biggest issue is the heavy use of `.unwrap()`. Invalid IDs, corrupt JSON, or file I/O failures currently crash the program instead of returning a useful message.

**Recommendation:** Return `Result` from `main` and helper functions, use `?`, and print clear errors to stderr.

## 2. Add real tests
`cargo test` currently passes with zero tests, so there is very little protection against regressions.

**Recommendation:** Add unit tests for parsing, add/edit/delete/done behavior, missing-ID handling, and JSON load/save round trips.

## 3. Fix the current hygiene issues
The project still has small tool-reported issues: `cargo clippy` warns about needless late initialization in `load_todos()`, and `cargo fmt -- --check` currently fails.

**Recommendation:** Clean up `load_todos()` into a direct expression and format the file with `cargo fmt`.

## 4. Make writes atomic
`save_todos()` uses `File::create`, which truncates `todo.json` before serialization completes. A crash during write can lose data.

**Recommendation:** Write to a temporary file first, then rename it over `todo.json`.

## 5. Use a real CLI parser
Manual `if/else if` argument parsing works for this tiny version, but it is brittle and hard to extend.

**Recommendation:** Use `clap` for subcommands, help text, argument validation, and clearer usage errors.

## 6. Split responsibilities
`main.rs` currently handles command parsing, business logic, storage, and output formatting in one file.

**Recommendation:** Move persistence and todo operations into a small `TodoManager` or separate modules so the code is easier to test and evolve.

## 7. Improve Rust idioms
There are a few easy quality upgrades that would simplify the code.

**Recommendation:** Use `match` for command dispatch, prefer `&[Todo]` over `&Vec<Todo>`, switch to `Path::new(PATH).exists()`, and use iterator helpers like `.max().unwrap_or(0)` for ID generation.

## 8. Tighten the CLI behavior
The current UX is functional but rough: `done` toggles rather than only marking done, `list` prints raw comma-separated fields, and errors like `id not found` are minimal.

**Recommendation:** Decide on explicit command semantics, improve output formatting, and return consistent exit codes for failures.

## 9. Keep this document aligned with the code
An earlier note here said the current path check would not compile, but the repo does build today.

**Recommendation:** Treat this file as a living summary of verified issues, not a static wish list.
