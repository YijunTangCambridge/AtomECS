# AtomECS
> Simulate laser cooling with rust.

[![build](https://github.com/TeamAtomECS/AtomECS/actions/workflows/build.yml/badge.svg)](https://github.com/TeamAtomECS/AtomECS/actions/workflows/build.yml) [![unit_tests](https://github.com/TeamAtomECS/AtomECS/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/TeamAtomECS/AtomECS/actions/workflows/unit-tests.yml)

The `atomecs` crate simulates the laser-cooling of atoms by optical scattering forces, and supports numerous features:
* Doppler forces on atoms that scatter light, including (optionally) the random fluctuations that give rise to the Doppler temperature limit.
* Magnetic fields, implemented on a grid or through simple analytical models.
* Atoms generated by an oven.
* Atoms generated on the surface of a simulation volume (eg, a chamber).
* Cooling light beams, defined by their detuning and gaussian intensity profiles.
* Volumes that define bounds for the simulation.
* File output in binary or text format.
* Thorough unit testing to ensure simulation results are correct.
* Good parallel performance on modern multi-core CPUs
* Simulations can be wrapped using python/matlab, as shown in the [source_optimisation_example](https://github.com/TeamAtomECS/source_optimisation_example).

## Getting Started

If you would like to get started, try some examples with `cargo run --release --example 1d_mot`, then use the scripts in the Matlab directory to plot the results.
You can also use `cargo doc` to explore the documentation, which has more detail on the structure of the program.

**Important note:** If you receive the error 'panicked while panicking' then see [this issue](https://github.com/TeamAtomECS/AtomECS/issues/2) - you may need to use an earlier toolchain.


## ECS

`atomecs` follows the data-oriented Entity-Component-System (ECS) pattern, which is implemented using [specs](https://github.com/slide-rs/specs).
ECS is well suited to high-performance simulations, and is flexible enough to accomodate changing design goals.
For these reasons, ECS has become established in the video game industry, and since 2018 Unity (one of the market leaders) has embraced the pattern.

_If you are unfamiliar with this pattern, and come from an object-oriented background, it is thoroughly recommended that you read about it before diving into the code._
Some useful resources:
* Although written for Unity/C#, the concepts in the [Unity Entities Package Documentation](https://docs.unity3d.com/Packages/com.unity.entities@0.14/manual/ecs_core.html) are very useful to understand.
* For the advantages of the pattern, see Mike Acton's [GDC talk](https://www.youtube.com/watch?v=p65Yt20pw0g)

### Current Limitations

* atom-atom interactions are not implemented. Most of our work deals with atom sources, which have low steady-state number densities, so we haven't implemented this. Results for steady-state 3D MOTs should be interpreted carefully.

## Credits

* [Xuhui Chen](https://github.com/Pi-sun), Oxford

* [Elliot Bentine](https://github.com/ElliotB256), Oxford

* [Maurice Zeuner](https://github.com/MauriceZeuner), Cambridge

* [Tiffany Harte](https://github.com/tiffanyharte), Cambridge
