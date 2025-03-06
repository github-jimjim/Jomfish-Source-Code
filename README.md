# Jomfish - Chess Engine Source Code

This repository contains the source code for Jomfish, my Rust-based chess engine. This is the **first version** of my successful chess bot, which currently boasts an impressive **3300 Elo**.

## Introduction

Jomfish is a Rust-based chess engine designed for high performance and adaptability. This repository hosts the **early versions** of the Jomfish Bot, with significant improvements in later releases. The goal is to continually evolve the engine towards becoming even stronger, with the ultimate aim of integrating NNUE for further enhancement.

## Versions

Here are the **older versions** of Jomfish that have been released with source code:

- **Version 2.0**: The second version of Jomfish, which has only simple commands that are needed to work.
- **Version 4.0**: Has a better evaluation and is slightly better than sunfish and Walleye

## Features

- **Version 2.0**: 
  - Basic UCI support with default configurations.
  - Focuses on performance without specialized tuning for threads or overhead.
  - Can be compiled and used in chess arenas of your choice.
  - Performance improvements compared to previous versions.
  - 2 Versions available: compact and none. The none is a little bit faster
 
- **Version 4.0**: 
  - Basic UCI support with default configurations.
  - Focuses on performance without specialized tuning for threads.
  - Can be compiled and used in chess arenas of your choice.
  - Performance improvements compared to previous versions.
  - It has a small improvement of about 150 elo from 3.0 and now it has **2200 elo**

## Collaboration

I would greatly appreciate any efforts to train an NNUE (Neural Network Unified Evaluation) model to integrate into **Jomfish 10**. If your NNUE trainer shows significant improvement, I will share the **Jomfish 10 source code** with you, including the trainer.

## Usage

- **Version 2.0** doesn't support any special UCI commands, only the basic ones without configuration options like Overhead or Threads.
- **Version 4.0** support Overhead. 
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

## Performance of Jomfish 4.0

| Engine          | Winrate % | Wins     | Looses   | Draws    |
|-----------------|-----------|----------|----------|----------|
| Jomfish 2       | 0%        | 0        | 3        | 1        |
| Jomfish 3       | 25%       | 1        | 2        | 1        |
| Jomfish 4       | 100%      | 4        | 0        | 0        |

Jomfish 4.0 has a estimated Elo of **2000** with a boost of **150** Elo from the 3.0. It finally has Move Overhead and plays normal in the endgame unlike the 3.0 or 2.0 version. You may have noticed it but after that it is unlikely that I post the other versions. If you are lucky maybe I will post the 6.0 but never the other verisons.


## When to Expect New Versions

You may have noticed it but after that it is unlikely that I post the other versions. If you are lucky maybe I will post the 6.0 but never the other verisons. I am planning not to post Jomfish until it reaches a NNUE.

## Pynfish and Jomfish Links

- [Arena](https://github.com/github-jimjim/Arenmy)
- [Pynfish](https://github.com/github-jimjim/Pynfish)
- [Jomfish 10](https://github.com/github-jimjim/jomfish)
