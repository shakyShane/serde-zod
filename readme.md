# serde-zod

Generate [`zod`](https://github.com/colinhacks/zod) definitions from your JSON-serializable types in Rust.

## Why

This library was created whilst building a [Tauri](https://github.com/tauri-apps/tauri) App where the architecture encourages heavy use of JSON serializable messaging.

Having those message structures described in Rust structs/enums was fantastic, but losing the type information on the frontend was a shame - so I built this attribute macro to solve that problem 💪😎

## Features

- [x] structs -> `z.object()`
- [x] Optimized enum representation
  - [x] defer to `z.enum(["A", "B")` when a Rust enum contains only `unit` variants (no sub-fields)
  - [x] use `z.discriminatedUnion("tag", ...)` when attribute `serde(tag = "kind")` is used
  - [x] fall back to `z.union` if fields are mixed
- [x] array subtype via `Vec<T>`
- [x] optional types `Option<T>`
- [ ] HashMap/BTreeMap
- [ ] Set/BTreeSet
- [ ] serde rename_all
- [ ] serde rename_field
- [ ] document all available output types

| rust                                   | zod                              |
|----------------------------------------|----------------------------------|
| enum with only unit variants           | z.enum([...])                    |
| enum with "tagged" variants            | z.discriminatedUnion("tag", ...) |
| enum with mixed variants               | z.union([...])                   |
| String                                 | z.string()                       |
| usize\|u8\|u16\|f32\|f64 etc (numbers) | z.number()                    |
| Option<String>                         | z.string().optional()            |
| Struct/Enum fields                     | z.object({ ... })                |

See the [tests](https://github.com/shakyShane/serde-zod/blob/main/app/src/main.rs) for more examples, or the [Typescript output](example/types.ts) to see what it generates.



## Basic Usage

Add the `#[serde_zod::codegen]` attribute *above* any existing Rust struct or enum where you
 already have `#[derive(serde::Serialize)]` or `#[derive(serde::Deserialize)]`

```rust
#[serde_zod::codegen]
#[derive(serde::Serialize)]
pub struct Person {
  age: u8,
}
```

With that, you can then create a binary application (alongside your lib, for example) to output the zod definitions

```rust
fn main() {
    let lines = vec![
        Person::print_imports(), // ⬅️ only needed once
        Person::codegen(),
    ];
    fs::write("./app/types.ts", lines.join("\n")).expect("hooray!");
}
```

**output**
```ts
import z from "zod"

export const Person =
  z.object({
    age: z.number()
  })

// ✅ usage
const person = Person.parse({age: 21})
```

--- 

# Output Types

## `z.enum()`

- [zod-enums](https://github.com/colinhacks/zod#zod-enums)

When you have a Rust enum with only `unit` variants - meaning all variants are 'bare' or 'without nested fields', then `serde-zod` will print a simple `z.enum(["A", "B"])`

**input**
```rust
#[serde_zod::codegen]
#[derive(Debug, Clone, serde::Serialize)]
pub enum UnitOnlyEnum {
    Stop,
    Toggle,
}

#[serde_zod::codegen]
#[derive(serde::Serialize)]
pub struct State {
    control: UnitOnlyEnum,
}
```

**output**
```ts
import z from "zod"

export const UnitOnlyEnum =
  z.enum([
    "Stop",
    "Toggle",
  ])

export const State =
  z.object({
    control: UnitOnlyEnum,
  })

// usage

// ❌ Invalid enum value. Expected 'Stop' | 'Toggle', received 'oops'
const msg = State.parse({control: "oops"})

// ✅ Both valid
const msg1 = State.parse({control: "Stop"})
const msg2 = State.parse({control: "Toggle"})
```

## `z.discriminatedUnion`

- [zod-discriminated-unions](https://github.com/colinhacks/zod#discriminated-unions)

When you use `#[serde(tag="<tag>")]` on a Rust enum, it can be represented by Typescript as a 'discriminated union'. So,
when `serde-zod` notices `tag=<value>`, it will generate the optimized zod definitions. These offer the best-in-class
type inference when used in Typescript

**input**
```rust
#[serde_zod::codegen]
#[derive(serde::Serialize)]
#[serde(tag = "kind")]
pub enum Control {
    Start { time: u32 },
    Stop,
    Toggle,
}
```

**output**

```ts
import z from "zod"

export const Control =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Start"),
      time: z.number(),
    }),
    z.object({
      kind: z.literal("Stop"),
    }),
    z.object({
      kind: z.literal("Toggle"),
    }),
  ])

// this usage show the type narrowing in action
const message = Control.parse({ kind: "Start", time: 10 });
if (message.kind === "Start") {
  console.log(message.time) // ✅ 😍 Type-safe property access here on `time`
}
```

## `z.union()`

- [zod-unions](https://github.com/colinhacks/zod#zod-unions)

This is the most flexible type, but also gives the least type information and produces the most unrelated errors (you get an errorr for each unmatched enum variant)

**input**
```rust
#[serde_zod::codegen]
#[derive(serde::Serialize)]
pub enum MixedEnum {
    One,
    Two(String),
    Three { temp: usize },
}
```

**output**
```ts
import z from "zod"

export const MixedEnum =
  z.union([
    z.literal("One"),
    z.object({
      Two: z.string(),
    }),
    z.object({
      Three: z.object({
        temp: z.number(),
      }),
    }),
  ])

// ✅ all of these are valid, but don't product great type information
MixedEnum.parse("One")
MixedEnum.parse({Two: "hello...."})
MixedEnum.parse({Three: { temp: 3 }})
```
