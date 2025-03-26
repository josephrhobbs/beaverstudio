# Contributing Guide

Thank you for contributing to the Beaver Studio project!

## Basics

At the heart of Beaver Studio is the `Artist` trait, the `Animate` trait, the `Animation` structure, and the `Shape` structure.

### `trait Artist`

Types that are `Artist` draw on video frames, creating shapes and other visual objects.

### `trait Animate`

Types that are `Animate` represent animations that occur in time.  Given a `progress` value between 0% and
100%, they create an `Artist` trait object that is capable of creating the animation in the given state.

Any animated data type must implement `Animate`.  This trait has three methods.

- `Animate::play(&self, progress: f64) -> Box<dyn Artist>` (_required_): given a progress value, create an `Artist`
trait object representing the current state of the animation.

- `Animate::clone_box(&self) -> Box<dyn Animate>` (_required_): clone the given trait object, allowing the `Animation`
structure to be passed to Python functions.

- `Animate::animate(&self) -> Animation` (_provided_): construct an `Animation` structure from this trait object.

Moreover, _every type that is `Animate`_ should implement the following three methods like so.  This code enables the
`MyAnimation` type to be converted into an `Animation`, which can be processed by Python.

```rust
struct MyAnimation;

impl MyAnimation {
    #[getter]
    pub fn get_display(&self) -> Animation;

    #[getter]
    pub fn get_trace(&self) -> Animation;

    #[getter]
    pub fn get_untrace(&self) -> Animation;
```

This enables `MyAnimation` to be traced, displayed, and untraced.

### `struct Animation`

The `Animation` structure contains an `Animate` trait object, and exists to safely interface with Python.  The user does
not created `Animation` structures directly, but rather creates them by means of the `.display`, `.trace`, and `.untrace`
attributes on user-facing data structures.

### `struct Shape`

The `Shape` structure represents a chain of Bezier curves.  Structures such as `Rectangle`, `Circle`, and `Polygon` inherit
the animation behaviors of `Shape`.  All other `Shape` objects should extend `Shape` using `#[pyclass(extends=Shape)]` in order
to ensure that all animation behaviors are inherited.  For more information, check out the implementations of `Rectangle`, `Circle`,
and `Polygon`.