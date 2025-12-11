# 👾 Git-Gotchi

**Git-Gotchi** is a CLI-based productivity pet that lives in your git repository. It rewards consistency and "good" commit habits while punishing neglect or bad practices.

## 🚀 Concept

Git-Gotchi isn't just a toy; it's a **Coach**.
- **Hunger (Time Decay)**: If you don't commit for 24h, your pet gets hungry. 48h? It gets sick. A week? It dies.
- **Eating (Commits)**: 
    - **Snack** (1-20 lines): Good!
    - **Lunch** (20-300 lines): Perfect! Best XP.
    - **Banquet** (300+ lines): Too much! Your pet gets **BLOATED** and earns less XP.
- **Evolution**: As you gain XP, your pet evolves from a simple Egg into a Digital God.

## 📦 Installation

Prerequisites: [Rust & Cargo](https://rustup.rs/).

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/git-gotchi.git
   cd git-gotchi
   ```
2. Build and install:
   ```bash
   cargo install --path .
   ```
   *Alternatively, just run with `cargo run --` for development.*

## 🎮 Usage

### 1. Initialize
Run this inside any git repository you want to gamify:
```bash
git-gotchi init
```
This creates a `.gitgotchi.json` save file and installs a `post-commit` hook in your `.git` folder.

### 2. Commit Code
Just use git as normal!
```bash
git add .
git commit -m "feat: implemented cool stuff"
```
The hook will automatically feed your pet based on the size of your commit.

### 3. Check Status
See how your buddy is doing:
```bash
git-gotchi status
```

## 🐉 Evolution Stages

| Level | Stage | Description |
|-------|-------|-------------|
| 1 | **Egg** | A mysterious byte. |
| 2-4 | **Baby Byte** | Cute and round. |
| 5-9 | **Script Kid** | Getting stronger. |
| 10-19 | **Code Bot** | A machine of productivity. |
| 20+ | **GIT GOD** | Digital ascension. |

## 🛠️ Tech Stack
- **Language**: Rust 🦀
- **CLI**: Clap
- **Serialization**: Serde JSON
- **Colors**: Colored crate

## 📝 Licence
MIT
