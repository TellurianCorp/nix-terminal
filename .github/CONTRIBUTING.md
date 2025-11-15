# Contributing to Tellurian nix-terminal

Thanks for investing time in improving the Tellurian nix-terminal project. This
document describes the different ways you can help, the standards we hold for
issues and pull requests, and the practical steps needed to spin up the local
environment.

## Ground rules

- Read and follow the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
- Discuss large changes in a GitHub issue before opening a pull request.
- Keep the scope of an issue or pull request focused and well documented.
- Prefer clear commit messages explaining the *why* behind a change.

## Ways to contribute

- **Report bugs** – Use the bug report template to describe reproduction steps,
  observed/expected behavior, and your runtime environment.
- **Request features** – Capture the user problem, the desired behavior, and any
  proposed UX ideas in a feature request issue.
- **Improve docs** – README improvements, architecture notes, or UI mockups are
  all appreciated.
- **Build features/fix bugs** – Coordinate in an issue, then send a PR that
  includes the relevant tests or screenshots.

## Development workflow

1. Install the latest stable [Rust toolchain](https://rustup.rs) plus the Wasm
   target if you plan on experimenting with web builds:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Clone the repository and install dependencies (Cargo will fetch crates on
   first build).
3. Run the application in debug mode:
   ```bash
   cargo run
   ```
4. Keep code formatted using `cargo fmt` and ensure `cargo clippy` (if installed)
   runs clean for any touched code.
5. Add or update tests whenever logic changes. UI-only work should include
   screenshots in the pull request body.

## Pull requests

- Fill out the entire pull request template so reviewers understand intent,
  testing, and potential risk.
- Reference the issue you are closing using `Fixes #123` style syntax.
- Expect at least one approving review before merging to `main`.
- CI (if configured) must succeed before maintainers will merge.

## Triaging and reviews

Maintainers prioritize issues and reviews based on project roadmap alignment and
severity. If you are waiting on feedback, feel free to comment with any new data
or context, but please avoid bumping issues more than once every few days.

## Questions?

If you need additional help, start with [`SUPPORT.md`](SUPPORT.md) for the list
of available channels. We're happy to mentor first-time contributors.
