# Raytracer

A simple raytracer, built following [The Raytracer Challenge](http://raytracerchallenge.com/).

Implemented from scratch with zero external dependencies (except for test helpers).

Current progress: _it can render a simple scene with spheres and cast shadows_.

![rendered sphere](https://raw.githubusercontent.com/dallagi/raytracer/main/examples/out/scene.png)

## Run the examples

This project requires rust nightly.

Run an example with:

```bash
cargo run --release --example scene
```

The rendered image will be saved in the `examples/out/` directory in the PPM format.

## Implementation notes

### Differences from the book

- There is no concept of tuple, points and vector have their own types
- Shading supports multiple sources of light

### Other notes
- Matrices and matrix operations are implemented via const generics (where possible)
