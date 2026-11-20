# FlatBuffers schema evolution example

This example demonstrates what happens when code generated from one version
of a FlatBuffers schema reads data written with another version.

- [`schemas/v1`](schemas/v1/battery_state.fbs) contains the fields `voltage`,
  `current` and `charge_percent` from 0 to 100.
- [`schemas/v2`](schemas/v2/battery_state.fbs) deprecates `charge_percent`, adds
  `charge_state` with values ranging 0.0 to 1.0, and adds an optional
  `temperature` field.

The v2 definition follows the
[FlatBuffers schema evolution rules](https://flatbuffers.dev/evolution/).
This is checked by `flatc --conform` in `nix flake check`.

## Usage

```sh
nix flake check  # build, test, lint.
nix run          # build and run the example
```

Or from the dev shell, run `cargo run`. You will see the following output:

```text
Backwards compatible: v2 code reads v1 data.

  field           written by v1   read by v2
  voltage         24.1            24.1
  current         1.5             1.5
  charge_percent  87.0            no accessor
  charge_state    absent          0.0 (default)
  temperature     absent          None (default)

Forwards compatible: v1 code reads v2 data.

  field           written by v2   read by v1
  voltage         24.1            24.1
  current         1.5             1.5
  charge_percent  absent          0.0 (default)
  charge_state    0.87            unknown to v1
  temperature     35.0            unknown to v1
```

## Conclusions

- Schema evolution does not require the schema definitions at runtime. Each
  program only uses the accessors generated from its own schema version. The
  buffer itself records which fields were written.
- Use `null` defaults in scalar field definitions so readers can differentiate
  between absent values and real ones. For example, reading v2 data with v1 code
  returns `charge_percent` as `0.0`, and reading v1 data with v2 code returns
  `charge_state` as `0.0`. Both look like an empty battery. `temperature` uses a
  `null` default, so v2 code reading v1 data gets `None` instead.
