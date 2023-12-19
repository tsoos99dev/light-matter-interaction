# FDTD based electromagnetic simulations

This is part of a personal study on light-matter interaction.
The simulation is currently limited to 1D and just a few sources and materials. It's mostly based on [this](https://eecs.wsu.edu/~schneidj/ufdtd/) material. The computations are implemented in Rust. Visualisations are done via Matlab.

## Setup

Install Rust:
https://www.rust-lang.org/tools/install

## Building

Once the prerequisites have been installed

```
cargo build --release
```

It's all single threaded right now, but it should take too long to run with reasonable parameters. This creates a csv output of sampled field values.

## Configuration

A new script can be created based on the example in main.rs. There's no CLI interface available for the time being. The capabilities of the simulation currently include the following:

**Sources:**

- Gaussian
- Harmonic

All sources are implemented using the TSFS method, so they can only move to the right.

**Materials:**

- Lossless dielectric
- Lossy dielectric (finite conductivity)
- Lorentz

## Boundaries

There is only a simple first and second order differential equation based boundary available.

## Probes

Measurements can be made by defining probes. The following probes are available:

- EPoint
- EField

## Exporting

Measurements can be exported into a csv file using one of the exporters.

## Visualisation

There's only a Matlab script right now that reads the exported csv from
the simulation. I'll add a Python script later.
