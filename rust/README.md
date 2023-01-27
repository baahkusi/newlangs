# Mastering Rust in A day
Main sources
- https://doc.rust-lang.org/book/title-page.html
- https://doc.rust-lang.org/nomicon/intro.html
- https://doc.rust-lang.org/std/index.html
## Variables
- Variables are immutable by default.
- You have to explicitly state that you want a variable to be mutable.
- You can redeclare a variable and even have a new type for the redeclaration.
- Constants.
### Refs
- https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

## Data Types
- Scalar and Compound Data Types
- Scaler Data Types (integer, floating-point, char, boolean)
- Compound Data Types (Tuples, Arrays, Strings)
### Refs
- https://doc.rust-lang.org/book/ch03-02-data-types.html

## Control Flow
- `if` statements of course.
- `if` statement is an expression so can be assigned to a variable.
- No `switch` but there is `match` statement.
- `loop`, `while` and `for` statements.
- `loop` statements are expressions and can be assigned to a variable.
- `loop` statements have labels so you can break out of an outer loop
  from and inner loop directly using the label; cool right?
- `for` and `while` as usual bro...
### Refs
- https://doc.rust-lang.org/book/ch03-05-control-flow.html

## In-built Data Structures
- Beware of Ownership, References & Borrowing
- Common Data Structures (Vec, String, HashMap)
### Refs
- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- https://doc.rust-lang.org/book/ch04-03-slices.html
- https://doc.rust-lang.org/book/ch08-01-vectors.html
- https://doc.rust-lang.org/book/ch08-02-strings.html
- https://doc.rust-lang.org/book/ch08-03-hash-maps.html

## How to create and invoke functions
- Functions are just normal like most programming languages, nothing notorious
- Closures are same, nothing notorious, except for me the notation is a bit wastefull, doesn't use anything similar to function notation
- Iterators are cool
### Refs
- https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
- https://doc.rust-lang.org/book/ch13-01-closures.html
- https://doc.rust-lang.org/book/ch13-02-iterators.html

## Familiarise yourself with some in-built functions.
- Has some helpful functions under various categories
- i/o, env variables, file, os, memory, etc ...
### Refs
- https://doc.rust-lang.org/std/all.html#functions

## How to create your own objects, data structures, classes.
- Structs and Method Implementations (Imple)
- Enums and Match syntax
- Rust can be used in object oriented style to a large degre
- This is possible with Traits
- Traits and Trait Bounds
- Lifetimes is an interesting concept
### Refs
- https://doc.rust-lang.org/book/ch05-01-defining-structs.html
- https://doc.rust-lang.org/book/ch05-02-example-structs.html
- https://doc.rust-lang.org/book/ch05-03-method-syntax.html
- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
- https://doc.rust-lang.org/book/ch06-02-match.html
- https://doc.rust-lang.org/book/ch06-03-if-let.html
- https://doc.rust-lang.org/book/ch17-01-what-is-oo.html
- https://doc.rust-lang.org/book/ch17-02-trait-objects.html
- https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
- https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions
- https://doc.rust-lang.org/book/ch10-02-traits.html
- https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
## Familiarize yourself with some in-built objects.
- Has some helpful objects/structs for various functionalities just like functions.
- i/0, network, file, os, memory, 
### Refs
- https://doc.rust-lang.org/std/io/index.html#structs
## Now time to understand how to put code projects together, ie. packages, imports, exports, libraries.
- As is usual with programming languages, lots of rules for modules, etc ...
- It's kind of straght forwared
- Cargo
### Refs
- https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
- https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
- https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
- https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
- https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
- https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
## Familiarize yourself with some in-built packages or libraries.
- Has some helpful modules for various things.
- i/0, network, file, os, memory, 
### Refs
- https://doc.rust-lang.org/std/index.html#modules
## Error handling.
- You either Panic or return Result
- Panic stops the execution of the program completely though, it can't be handled
### Refs
- https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
- https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
- https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
## Understand good and bad practices, coding conventions, gotchas.
- Rust does have a strict compiler, type system and also formating tool.
- These makes it such that best practices and conventions are enforced from the tooling level.
- When it comes to object oriented programming rust has its own approach but there is some flexibility.
### Refs
- https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html