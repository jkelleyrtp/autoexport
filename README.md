# Autoexport

<div>
  <a href="https://crates.io/crates/autoexport">
    <img src="https://img.shields.io/crates/v/autoexport.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/autoexport">
    <img src="https://img.shields.io/crates/d/autoexport.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/autoexport">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

Automatically export a series of modules from a top-level module.

The modules must be defined within the body of the macro - autoexport does not know about modules inside files. This means
you need to define your module structure from a top-level file, like lib.rs.

Rust's module system can be verbose at times, especially for large apps. This helps make your programs cleaner.

## Usage

It's common to define a series of routes, components, or useful utilities in a top-level lib.rs that you want exported to other parts of your app. Just take on `#[autoexport::autoexport]` and voila, no need to write `pub use item::*;`

Autoexport will use the visibility defined at the site of the module

```rust
#[autoexport::autoexport]
pub mod components {
    pub mod footer;
    pub mod header;
}

// now all of footer/header are in scope

run_function_thats_defined_in_footer();

run_function_thats_defined_in_header();
```


## License

MIT or Apache  at your discretion
