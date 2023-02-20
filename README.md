# Raytracer

A simple raytracer, built following [The Raytracer Challenge](http://raytracerchallenge.com/).

Implemented from scratch with zero external dependencies (except for test helpers).

Current progress: _it renders a sphere_.

![rendered sphere](https://raw.githubusercontent.com/dallagi/raytracer/main/examples/out/sphere.png)

## Run the examples

This project requires rust nightly.

Run an example with:

```bash
cargo run --release --example sphere
```

The rendered image will be saved in the `examples/out/` directory in the PPM format.
