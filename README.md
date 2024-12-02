# Advent of Code 2024 - Rust Solutions

Welcome to my repository for **Advent of Code 2024**, where I solve each day's challenges using the Rust programming language.

---

## About Advent of Code
[Advent of Code](https://adventofcode.com/2024) is an annual coding event with daily programming puzzles released throughout December. It’s a great way to enhance your problem-solving skills and learn new programming concepts.

---

## Repository Structure
Here's how this repository is organized:

```
advent-of-code-2024/
├── README.md          # You're here!
├── Cargo.toml         # Rust project configuration
├── src/               # Source files for solutions
│   ├── main.rs        # Entry point of the project
│   ├── days/          # Solution files for each day
│   │   ├── day01.rs   # Example: Day 1 solutions (both parts)
│   │   ├── day02.rs
│   │   └── ...
├── input/             # Puzzle input files
│   ├── day01.txt      # Example: Input for Day 1
│   ├── day02.txt
│   └── ...
```

---

## How to Run

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/Divakar-2508/advent-of-code-2024.git
   cd advent-of-code-2024
   ```

2. **Run a Specific Day:**

   Use the following command to run solutions for a specific day:

   ```bash
   cargo run -- <day>
   ```

   Example:

   ```bash
   cargo run -- 1
   ```
---

## Why Rust?

Rust is a systems programming language known for its speed, safety, and expressiveness. It's a perfect fit for Advent of Code as it helps write efficient and elegant solutions.

---

## Notes

- Inputs are stored in the `input/` directory.
- Solutions for both parts of a day are in the corresponding `dayXX.rs` file under `src/days/`.

---

## License

Feel free to use and share these solutions as per the [MIT License](LICENSE).
