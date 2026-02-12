<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="logo.png">
    <source media="(prefers-color-scheme: light)" srcset="logo.png">
    <img height="200" alt="Rust-to-datapack Logo" src="logo.png">
  </picture>
</p>

<h1 align="center">🦀 Rust-to-datapack</h1>
<p align="center">
  <em>Write Minecraft datapacks in Rust – fast, safe, and expressive.</em>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/rust-1.70%2B-blue?style=flat-square&logo=rust" alt="Rust version">
  </a>
  <a href="https://minecraft.net/">
    <img src="https://img.shields.io/badge/Minecraft-1.20%2B-success?style=flat-square&logo=minecraft" alt="Minecraft">
  </a>
</p>

---

## 📦 Overview

**Rust-to-datapack** is a pioneering tool that lets you generate full-featured Minecraft datapacks using the Rust programming language. No more wrestling with thousands of JSON and `.mcfunction` files – write clean, modular Rust code and compile it into a ready-to-play datapack.

> 💡 **Why?**  
> Datapacks often require repetitive, error-prone text files. With Rust, you get strong typing, code reuse, and blazing‑fast iteration. Build complex mechanics with a fraction of the effort.

---

## ✨ Features

- 🧱 **Intuitive API** – create functions, recipes, advancements, loot tables and more with simple Rust constructs.
- 🧩 **Modular** – split your datapack logic into Rust modules; reuse code across projects.
- ⚡ **Compile‑time checks** – catch command errors before you even launch Minecraft.
- 🔧 **Zero runtime overhead** – outputs pure `.mcfunction` and JSON files; no extra dependencies in the datapack.
- 🚀 **Fast iteration** – edit Rust, run `cargo run`, replace datapack, reload – all in seconds.
- 🧪 **Active development** – new Minecraft features are added regularly.

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or newer)
- Minecraft Java Edition (1.20+ recommended)

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/rust-to-datapack.git
cd rust-to-datapack
cargo build --release
```

Tip: You can also use this as a template for your own datapack project. Fork or copy the structure and start coding!

---

📝 Usage

1. Write your datapack logic in src/main.rs

```rust
mod main_commands;
use main_commands::*;

fn radius(radius: i32) {
    for x in -radius..=radius {
        for y in -radius..=radius {
            for z in -radius..=radius {
                let command2 = format!(
                    "execute at @a if block ~{} ~{} ~{} minecraft:carved_pumpkin if block ~{} ~{} ~{} minecraft:soul_sand run fill ~{} ~{} ~{} ~{} ~{} ~{} minecraft:air replace",
                    x, y, z, x, y-1, z, x, y-1, z, x, y, z
                );
                let command = format!(
                    "execute at @a if block ~{} ~{} ~{} minecraft:carved_pumpkin if block ~{} ~{} ~{} minecraft:soul_sand run summon minecraft:wither_skeleton ~{} ~{} ~{}",
                    x, y, z, x, y-1, z, x, y, z
                );
                tick(&command);
                tick(&command2);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    restart_files();

    // Create a function called "welcome"
    create_fn("welcome");
    wfn("welcome", "title @a subtitle {\"text\":\"lol\",\"color\":\"red\"}");

    // Add commands to load.mcfunction
    load("title @a title {\"text\":\"hello world!\",\"bold\":true,\"color\":\"dark_red\"}");
    load("function main:welcome");

    // Generate wither-skeleton‑golem behaviour in a 4‑block radius
    radius(4);

    // Create a shaped recipe: one book -> one knowledge book (renamed)
    recipe_template(
        "big_book",
        vec!["S"],
        vec![("S", "minecraft:book")],
        "minecraft:knowledge_book",
    );

    // Give the player a custom player head upon unlocking the recipe
    connect_advancement(
        "big_book_advancement",
        "big_book",
        "give_big_book_func",
        "minecraft:player_head{display:{Name:'{\"text\":\"book\",\"color\":\"gold\",\"underlined\":true,\"bold\":true,\"italic\":false}',Lore:['{\"text\":\"ID головы: 1426\",\"color\":\"gray\",\"italic\":false}','{\"text\":\"\",\"color\":\"blue\",\"italic\":false}']},SkullOwner:{Id:[I;2080793942,-524468218,-1541115779,1949756395],Properties:{textures:[{Value:\"eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvNmRlNGFiMTI5ZTEzN2Y5ZjRjYmY3MDYwMzE4ZWUxNzQ4ZGMzOWRhOWI1ZDEyOWE4ZGEwZTYxNGUyMzM3NjkzIn19fQ==\"}]}}} 1",
    );

    Ok(())
}
```

2. Build the datapack

Simply run:

```bash
cargo run
```

After execution, a new folder named data will appear in your project root. This is your generated datapack.

3. Install the datapack

· Create a pack.mcmeta file (see Minecraft Wiki) in the root of your generated folder.
· Place the entire folder (containing data and pack.mcmeta) into the datapacks folder of your Minecraft world.
· Run /reload in-game or restart the world.

That’s it! Your custom Minecraft mechanics are now live.

---

🧠 What does the example do?

The example above demonstrates two powerful features:

1. Golem‑like summoning –
      When a player places a carved pumpkin on top of soul sand, a wither skeleton is spawned and the soul sand is removed. This works in a 4‑block radius around every player.
2. Custom recipe & reward –
      A simple recipe (one book → one knowledge book) is created. Unlocking this recipe grants the player a custom player head (a “big book” with special lore and texture).
3. A tiny greeting –
      When the datapack loads, a colourful “hello world!” title appears and a function called welcome is executed.

All this is achieved with clean, iterative Rust code – no manual JSON editing required.

---

📚 API Overview (preview)

Function Description
restart_files() Clears previous output and prepares a fresh datapack structure.
create_fn(name) Creates a new .mcfunction file.
wfn(name, command) Appends a command to the given function.
load(command) Adds a command to the load.mcfunction (runs when datapack loads).
tick(command) Adds a command to the tick.mcfunction (runs every game tick).
recipe_template(...) Generates a shaped crafting recipe.
connect_advancement() Links a recipe to an advancement that rewards an item upon unlocking.

More utilities (loot tables, predicates, tags) are coming soon – contributions are welcome!

---

🤝 Contributing

We’d love your help to make Rust-to-datapack even better!

· 🐛 Found a bug? Open an issue.
· 💡 Have an idea? Start a discussion or submit a pull request.
· 📖 Improve docs? The README and in‑code docs are always a good place to start.

Please read our CONTRIBUTING.md for more details (you can add this file later).

---


❤️ Acknowledgements

· Inspired by the endless creativity of the Minecraft datapack community.
· Built with Rust – a language that empowers fearless concurrency and reliability.

---

<p align="center">
  <i>Happy datapacking – with 🦀!</i>
</p>