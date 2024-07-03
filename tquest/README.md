[![ci](https://github.com/OkieOth/rs_tquest/actions/workflows/rust.yml/badge.svg)](https://github.com/OkieOth/rs_tquest/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/tquest.svg)](https://crates.io/crates/tquest)



# tquest
Rust library crate to implement terminal based questionnaires


# Idea

Build a structure that guides through a set of cascaded questions,
collect the related answers and use the bundled structe in the end
to some further processing

## Assumed Questions

1. Do you want to do this questionaire? (y/N)
2. What's your name? - string result, max length ...
3. What's your day of birth? - string result, regexp check
4. Do you have one or more siblings? (y/N)
    4.1 What's the name of your sibling?
    4.2 What's the date of birth of your sibling
    4.3 Do you have further Siblings
5. Do you worked already in a job? (y/N)
   5.1. Where was that job?
   5.2. What company you worked for?
...

# Usage

```shell
cargo test -- --ignored
```