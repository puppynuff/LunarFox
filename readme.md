# Lunar Fox
## A new programming language

# Rust compilation

## Made in rust
This is a lfc -> rust compilation mode.

## Dependancies
Rust

## File extension
| File extension             |           Use         |
| -------------------------- | --------------------- |
| .lfc                       | Used for main files   |
| Cargo.toml (Generated)     | Used by cargo         |
| Cargo.lock (Generated)     | Used by cargo         |
| .gitignore (Generated)     | Used by github        |

This transpiles to rust. All rust code is also going to be valid, so feel free to look at their docs too!

[Rust's Website](https://www.rust-lang.org/)

___

## Current features
### C like variable definitions.
***Working types***
| Type keyword               |     Rust equivilant   |
| -------------------------- | --------------------- |
| ***Integers***             | ***Rust translation***|
| int8                       | i8                    |
| short                      | i16                   |
| int                        | i32                   |
| long                       | i64                   |
| int128                     | i128                  |
| arch                       | isize                 |
| ***Unsigned integers***    | ***Rust Translation***|
| uint8                      | u8                    |
| ushort                     | u16                   |
| uint                       | u32                   |
| ulong                      | u64                   |
| uint128                    | u128                  |
| uarch                      | usize                 |
|***Floating point numbers***| ***Rust Translation***|
| float                      | f32                   |
| double                     | f64                   |
| Booleans                   | Rust Translation      |
| bool                       | bool                  |
| ***Strings***              | ***Rust Translation***|
| string                     | String                |
| str                        | &str                  |
| char                       | char                  |

### Javascript like functions
***Working ways to make functions***

```rust
<span style="color:blue">function</span> () -> String {
    return "This works!".to_string();
}

fn () -> String {
    return "This works too!".to_string();
}
```

### All rust code is valid lsc code too!
If you really wanted to, you could rename any rust file to .lsc, put it in the source folder and it will work!

## Future features
***Check out my [Trello page](https:///trello.com/b/KRgmYTqb/lunarfox)***

___

# Contributing
## This is my first large rust project.
If you can, try to help me figure out the best / correct ways to do things since I still don't know much about rust.

Also try to keep indenting to a minimum.

## Warnings
If there is a warning, tell me and Ill try to fix it, or try to get rid of it before making a pull request

When making a update, please list what you changed / added too, and if you fixed any major bugs.

I will try to get your guy's updates into the project, just there is no guarentee. 

# Typescript transpilation

# NOT CURRENTLY IN DEVELOPMENT. CURRENT BRANCH IS RUST.

## Made in typescript
This version is a lfc -> typescript compilation mode.

## Dependancies
Node & Npm

## File extension
| File extension |         Use                      |
| -------------- | -------------------------------- |
| .lfc                        | Used for main files |
| package,json  (Generated)   | Used by node        |
| tsconfig.json (Generated)   | Used by typescript  |
| .gitignore    (Generated)   | Used by github      |