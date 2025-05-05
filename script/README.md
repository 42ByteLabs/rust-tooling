# Rust Script

Run Rust scripts in GitHub Actions or in a container.

## Usage

### GitHub Actions

**In-line script:**

```yaml
- name: Run a Rust script
  uses: 42ByteLabs/rust-tooling/script@v0.1.1
  with:
    script: |
      // cargo-deps: chrono="0.4"
      fn main() {
          println!("Hello, world!");
      }
```

**Script in a file:**

```yaml
- name: Run a Rust script
  uses: 42ByteLabs/rust-tooling/script@v0.1.1
  with:
    path: script.rs
```

### Container

```sh
docker run --rm -v "$(pwd)":/code -w /code 42bytelabs/rust-tooling-script:latest script.rs
```



