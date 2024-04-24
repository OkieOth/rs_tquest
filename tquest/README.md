![WIP](https://img.shields.io/badge/work%20in%20progress-red)

# tquest
Rust library crate to implement terminal based questionnaires


# Idea

Buid a structure that guides through a set of cascaded questions,
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
