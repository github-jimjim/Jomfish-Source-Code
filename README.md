# Jomfish - Chess Engine Source Code

This repository contains the source code for Jomfish, my Rust-based chess engine. This is the **first version** of my successful chess bot, which currently boasts an impressive **3300 Elo**.

## Introduction

Jomfish is a Rust-based chess engine designed for high performance and adaptability. This repository hosts the **early versions** of the Jomfish Bot, with significant improvements in later releases. The goal is to continually evolve the engine towards becoming even stronger, with the ultimate aim of integrating NNUE for further enhancement.

## Versions

Here are the **older versions** of Jomfish that have been released with source code:

- **Version 2.0**: The second version of Jomfish, which has gained popularity for its performance and ability to compete against strong engines.

**Note**: The latest version (Jomfish 10) is not included in this repository. Only the earlier versions are available.

## Features

- **Version 2.0**: 
  - Basic UCI support with default configurations.
  - Focuses on performance without specialized tuning for threads or overhead.
  - Can be compiled and used in chess arenas of your choice.
  - Performance improvements compared to previous versions.
  - 2 Versions available: compact and none. The none is a little bit faster

## Collaboration

I would greatly appreciate any efforts to train an NNUE (Neural Network Unified Evaluation) model to integrate into **Jomfish 10**. If your NNUE trainer shows significant improvement, I will share the **Jomfish 10 source code** with you, including the trainer.

## Usage

- **Version 2.0** doesn't support any special UCI commands, only the basic ones without configuration options like Overhead or Threads. 
- Compile it and use it in any arena of your choice.

### Compilation Process

To compile Jomfish, run:
```bash
cargo build --release
```
If you don't want to compile it yourself, you can go to the Jomfish 10 repository or download it from the release section.

## Performance of Jomfish 2.0

While Jomfish 2.0 is not as strong as Stockfish 1 or 2, it has a significant improvement from the earlier Jomfish versions. The difference in Elo between Stockfish 1 (2800 Elo) and Stockfish 10 (3300 Elo) is relatively small (about 500 Elo). However, Jomfish 2.0 has made a substantial leap from Jomfish 1 (1000 Elo) to Jomfish 10 (3300 Elo).

Jomfish 2.0 can easily defeat Pynfish 2, even though Pynfish 2 uses Stockfish 12 NNUE. However, Jomfish 2.0 has an approximate Elo of **1700** and sometimes makes errors, especially in winning positions (such as drawing through threefold repetition). Jomfish 3 improved on these weaknesses with a performance boost and additional UCI commands.

## When to Expect New Versions

I will try to release new versions when I have time, but it may take months. However, I am not planning to release the latest version (Jomfish 10) here, only the older versions. Stay tuned and have fun!

## Pynfish and Jomfish Links

- [Arena](https://github.com/github-jimjim/Arenmy)
- [Pynfish](https://github.com/github-jimjim/Pynfish)
- [Jomfish 10](https://github.com/github-jimjim/jomfish)
