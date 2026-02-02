# CONTRIBUTING.md

Thank you for your interest in contributing to **tree-sitter-yang**! This project aims to build a robust, RFC-compliant YANG parser library based on [tree-sitter](https://tree-sitter.github.io/tree-sitter/), and we welcome contributions of all kinds—from code fixes and feature enhancements to documentation improvements and bug reports. 

Below is a comprehensive guide to help you get started, align with our project standards, and ensure a smooth collaboration experience.


## Table of Contents
1. [Code of Conduct](#code-of-conduct)
2. [How to Contribute](#how-to-contribute)
   - [Report a Bug](#1-report-a-bug)
   - [Request a Feature](#2-request-a-feature)
   - [Submit a Pull Request](#3-submit-a-pull-request)
3. [Development Setup](#development-setup)
   - [Prerequisites](#prerequisites)
   - [Local Installation](#local-installation)
4. [Development Guidelines](#development-guidelines)
   - [Branch Naming Conventions](#branch-naming-conventions)
   - [Commit Message Standards](#commit-message-standards)
   - [Code Style](#code-style)
   - [Testing Requirements](#testing-requirements)
5. [Pull Request Process](#pull-request-process)
6. [RFC Compliance Notes](#rfc-compliance-notes)
7. [Community & Support](#community--support)
8. [License](#license)


## Code of Conduct
To foster an open, inclusive, and respectful community, we adhere to a **Code of Conduct** (inspired by the [Contributor Covenant](https://www.contributor-covenant.org/)):
- Treat all contributors with kindness, empathy, and respect—regardless of background, experience, or identity.
- Avoid discriminatory, harassing, or dismissive language/behaviors.
- Collaborate constructively: focus on ideas, not individuals; provide actionable feedback.
- Be mindful of your impact on others: prioritize clarity and patience in discussions.

Violations may result in temporary or permanent exclusion from the project. If you witness or experience misconduct, please reach out to the maintainers via GitHub Discussions.


## How to Contribute
There are multiple ways to support the project—even if you’re new to Tree-sitter or YANG!


### 1. Report a Bug
If you encounter unexpected behavior (e.g., parsing errors for valid YANG files, crashes, or incorrect AST output), follow these steps to file a useful bug report:
1. **Check for Duplicates**: First, search the [Issues](https://github.com/trislu/tree-sitter-yang/issues) tab to ensure the bug hasn’t already been reported.
2. **Open a New Issue**: Use the "Bug Report" template (if available) and include:
   - A clear, descriptive title (e.g., "RFC 7950 `leaf-list` with `units` fails to parse").
   - Steps to reproduce (minimal YANG snippet that triggers the bug).
   - Expected vs. actual behavior (e.g., "Expected AST to include `units` node; actual output omits it").
   - Environment details (OS, Rust version, Node.js version—if testing bindings).
   - Any relevant logs/error messages.


### 2. Request a Feature
Want to add support for a YANG feature (e.g., Customized Extension Parsing) or improve tooling? Follow these steps:
1. **Search Existing Requests**: Check [Issues](https://github.com/trislu/tree-sitter-yang/issues) to avoid duplicates.
2. **Open a Feature Request**: Use the template (if available) and include:
   - A clear use case (e.g., "Add support for `zig` binding").
   - Any technical suggestions (e.g., "Extend `lib.rs` to handle syntatical validation.").


### 3. Submit a Pull Request
Ready to write code or fix documentation? Follow this workflow to ensure your PR is reviewed quickly:
1. **Fork the Repository**: Click the "Fork" button at the top of the [repo](https://github.com/trislu/tree-sitter-yang) to create your own copy.
2. **Clone Your Fork**: 
   ```bash
   git clone https://github.com/your-username/tree-sitter-yang.git
   cd tree-sitter-yang
   ```
3. **Create a Feature/Bugfix Branch**: Use our [branch naming convention](#branch-naming-conventions) (e.g., `fix/rfc7950-leaf-list-units`).
4. **Make Changes**: Implement your fix/feature, following our [code style](#code-style) and [testing requirements](#testing-requirements).
5. **Push to Your Fork**: 
   ```bash
   git push origin your-branch-name
   ```
6. **Open a PR**: Go to the original repo’s [Pull Requests](https://github.com/trislu/tree-sitter-yang/pulls) tab and click "New Pull Request". Link to any related issues (e.g., "Fixes #123").


## Development Setup
To work on `tree-sitter-yang` locally, ensure you have the following tools installed, then follow the installation steps.


### Prerequisites
The project uses multiple languages (Rust, JavaScript, C) and tools—install these first:
| Tool               | Purpose                                  | Installation Link                          |
|---------------------|------------------------------------------|--------------------------------------------|
| Rust (1.90+)        | Core parser logic & bindings             | [rustup.rs](https://rustup.rs/)            |
| tree-sitter-cli (0.26+)       | Tree-sitter grammar development          | `cargo install tree-sitter-cli` or `npm install tree-sitter-cli`          |
| C Compiler (C11) | Build Rust bindings & Tree-sitter core    | `sudo apt install gcc` (Linux) / Xcode (macOS) |
| Git                 | Version control                          | [git-scm.com](https://git-scm.com/)        |


### Local Installation
1. **Install Dependencies**:
   - Rust: Run `rustup install 1.90` to use the suitable Rust version.
   - JavaScript: Run `npm install` to install Tree-sitter CLI and JS dependencies.

2. **Build the Parser**:
   ```bash
   # Generate Tree-sitter parser (from grammar.js)
   tree-sitter generate --abi=14

   # Build the Rust binding (uses Cargo)
   cargo build # add '--feature dev' to regenerate parser
   ```

3. **Verify Local Changes**:
   - For parser's test(if any): `tree-sitter test`.
   - For Rust bindings: Use `cargo test --verbose` to test the Rust API.


## Development Guidelines
To keep the codebase consistent and maintainable, follow these standards:


### Branch Naming Conventions
Use a prefix to indicate the branch’s purpose, followed by a short, descriptive name:
| Prefix       | Purpose                                  | Example                                  |
|--------------|------------------------------------------|------------------------------------------|
| `fix/`       | Bug fixes                                | `fix/rfc7950-boolean-node-parsing`       |
| `feat/`      | New features (e.g., RFC support)         | `feat/rfc9890-extension-parsing`         |
| `docs/`      | Documentation updates                    | `docs/add-contributing-guide`             |
| `refactor/`  | Code refactoring (no functional change)  | `refactor/simplify-grammar-rules`        |
| `test/`      | Adding/updating tests                    | `test/add-leaf-list-unit-tests`          |


### Commit Message Standards
Follow the [Conventional Commits](https://www.conventionalcommits.org/) format to make history readable and enable automated release notes:
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```
- **Type**: Use `fix`, `feat`, `docs`, `test`, `refactor`, `chore` (e.g., dependency updates).
- **Scope** (optional): Specify the area (e.g., `grammar`, `rust-bindings`, `tests`).
- **Description**: Short (≤72 chars) summary of the change.
- **Body** (optional): Explain *why* the change is needed (if non-obvious).
- **Footer** (optional): Link to issues (e.g., `Fixes #123` or `Closes #456`).

**Examples**:
- `fix(grammar): correct RFC 7950 boolean node exposure`
- `feat(rust-bindings): add AST traversal helper methods`
- `docs: update RFC links to include 9890`


### Code Style
Consistency helps reviewers focus on logic, not formatting. Use these tools to auto-enforce style:
1. **Rust**:
   - Use `rustfmt` (run `cargo fmt` to format code).
   - Use `clippy` for linting (run `cargo clippy --all-targets --all-features` to catch issues).
   - Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for bindings.

2. **JavaScript (Grammar)**:
   - Use editors that supports `.editorconfig`.
   - Follow Tree-sitter’s [grammar best practices](https://tree-sitter.github.io/tree-sitter/creating-parsers#grammar-rules) (e.g., prioritize clarity over brevity).

3. **C (External Scanner)**:
   - Follow [`LLVM Coding Standards`](https://llvm.org/docs/CodingStandards.html) style, see `src/.clang-format`.

4. **Documentation**:
   - Use Markdown consistently (headings, lists, code blocks).
   - Keep comments concise: Explain *why* (not *what*) for non-obvious code.


### Testing Requirements
All changes (bug fixes, features) must include tests to prevent regressions. Follow these rules:
1. **Test Types**:
   - **Grammar Tests**: Add YANG snippets to the `tests/corpus/` directory (Tree-sitter’s corpus test format) to validate parsing. Example:
     ```yaml
     # tests/corpus/leaf-list-units.txt
     ---
     name: Parse leaf-list with units (RFC 7950)
     corpus: |
       leaf-list temperature {
         type int32;
         units "celsius";
       }
     expectations:
       - [leaf-list, name: temperature, units: "celsius"]
     ```
   - **Rust Tests**: Add unit/integration tests to `src/lib.rs` or `tests/` (e.g., test Rust API methods).
   - **Binding Tests**: Add tests for JS/Python/Go bindings in their respective `bindings/<lang>/test/` directories.

2. **Test Coverage**:
   - Aim for ≥80% code coverage for new Rust code (run `cargo tarpaulin` to check).
   - Ensure all grammar rules have corresponding corpus tests (no untested YANG constructs).


## Pull Request Process
To ensure your PR is merged smoothly, follow these steps:
1. **Link to Issues**: If your PR fixes a bug or implements a feature request, link to the issue in the PR description (e.g., "Fixes #123").
2. **Pass CI Checks**: Ensure all GitHub Actions workflows pass (Rust CI, Cargo Publish, JS tests). If checks fail, fix the issues (e.g., linting errors, test failures).
3. **Request Reviews**: Assign 1–2 maintainers (e.g., `trislu`) for review. Address all feedback promptly—be open to discussion!
4. **Squash Commits (If Needed)**: If your PR has multiple "work-in-progress" commits, squash them into a single, clean commit (use `git rebase -i HEAD~N` where `N` is the number of commits).
5. **Merge**: Once approved and CI passes, a maintainer will merge your PR. If you have write access, use "Squash and Merge" to keep history clean.


## RFC Compliance Notes
`tree-sitter-yang` is designed to be fully compliant with YANG RFCs. When contributing:
- **Prioritize RFCs**: The project supports 2 core RFCs—ensure changes align with them:
  1. [RFC 6020](https://www.rfc-editor.org/rfc/rfc6020) (YANG 1.0)
  2. [RFC 7950](https://www.rfc-editor.org/rfc/rfc7950) (YANG 1.1)
- **Flag Non-Compliance**: If your change deviates from an RFC (e.g., for backward compatibility), explicitly note this in the PR description and discuss with maintainers.
- **Update RFC Docs**: If you add support for a new RFC feature, update the `README.md`’s "Supported RFCs" section.


## Community & Support
Join the conversation to ask questions, share ideas, or get help:
- **GitHub Discussions**: Use the [Discussions](https://github.com/trislu/tree-sitter-yang/discussions) tab for general questions (e.g., "How do I add a new grammar rule?").
- **Slack/Discord** (Future): If the community grows, we’ll add a chat channel—watch the repo for updates.
- **Maintainer Contact**: Reach out to `trislu` via GitHub if you need urgent help with a contribution.


## License
By contributing to `tree-sitter-yang`, you agree that your contributions will be licensed under the project’s [MIT LICENSE](https://github.com/trislu/tree-sitter-yang/blob/main/LICENSE). Ensure you have the right to contribute your code (no third-party copyright issues).


Thank you again for contributing! Your work helps make `tree-sitter-yang` a better tool for the YANG and Tree-sitter communities. 🚀