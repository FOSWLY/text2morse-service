# Text2Morse Service

⚡ Blazing fast and memory safety 🦀 text-to-morse online service!

Our service strives to be very secure and transparent, so you must log in with a Yandex 💛 account to use it!

We don't share your data with anyone, or even store it, because we can afford it 🛸

## How to run

Don't forget to fill in the env variables in `.env` file

To run your own instance:

### With Docker

1. Install [Docker](https://www.docker.com/)
2. Build the image

```bash
docker build -t "text2morse" .
```

3. Run container

```bash
docker run -p 7674:7674 text2morse
```

### Manually

1. Install [Rust 1.75+](https://www.rust-lang.org/learn/get-started)

2. (Optional) Run for developing:

   2.1. Install cargo watch:

   ```bash
   cargo install cargo-watch
   ```

   2.2. Run live server:

   ```bash
   cargo watch -x run
   ```

3. Run for Production:

   3.1. Build:

   ```bash
   cargo build --release
   ```

   3.2. Run a text2morse file:

   ```bash
   ./target/release/text2morse
   ```

idk what else to write, so here's a random pic.

<details>
  <summary>Click to see a pic</summary>

  <img src="https://github.com/user-attachments/assets/cd3101a9-3768-4076-ac21-04f66614e19b" alt="pic">
</details>

Written with ❤️ & 🦀
