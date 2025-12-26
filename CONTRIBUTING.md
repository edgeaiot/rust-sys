### 🦀 CONTRIBUTING

````markdown
# Contributing to This Rust Project

Welcome! 🎉  
Thanks for your interest in contributing. Whether you found a typo, want to fix a bug, or add a new feature — your help is appreciated.

---

## 🧰 Prerequisites

Before contributing, please make sure you have:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [cargo](https://doc.rust-lang.org/cargo/) installed
- A GitHub account

---

## 💡 How to Contribute

### 1. Fork and Clone
```bash
git clone https://github.com/<your-username>/<project-name>.git
cd <project-name>
````

### 2. Create a New Branch

Please name your branch clearly:

```bash
git checkout -b fix/issue-description
# or
git checkout -b feature/short-feature-name
```

### 3. Make Your Changes

Follow Rust best practices:

* Keep your code clean and readable
* Use `rustfmt` for formatting:

  ```bash
  cargo fmt
  ```
* Run `cargo clippy` to catch common mistakes
* Add or update tests if necessary:

  ```bash
  cargo test
  ```

### 4. Commit Changes

Use descriptive commit messages:

```bash
git add .
git commit -m "Fix panic when parsing empty input"
```

### 5. Push and Open a Pull Request

Push your branch and open a PR to the `main` branch:

```bash
git push origin feature/short-feature-name
```

Then go to GitHub → open a Pull Request → describe what you changed and why.

---

## 🧪 Running Tests

Make sure all tests pass locally before opening a PR:

```bash
cargo test
```

You can also run with verbose output:

```bash
cargo test -- --nocapture
```

---

## 🧱 Code Style

Please follow Rust conventions:

* Use [Rustfmt](https://github.com/rust-lang/rustfmt)
* Avoid unsafe code unless absolutely required
* Prefer `Result<T, E>` over panicking (`unwrap`, `expect`)
* Document public functions with `///` comments

---

## 🗣️ Communication

* Use [GitHub Issues](../../issues) for bugs, suggestions, and discussions.
* Be respectful and constructive.
* PR reviews aim to improve the code, not criticize contributors.

---

## 🪄 Example Commands

```bash
cargo run
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

---

Thank you for helping make this project better! ❤️
Happy coding 🦀

```
