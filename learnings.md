# Learnings

- Rustaceans // Rust Enthusiasts

- rustc <filename.rs>
- ./main
  OR
- rustc <filename.rs> -o <output_filename>
- ./<output_filename>
  OR
- cargo run
  Cargo is Rust's build system & package manager. cargo new <project>, cargo init, cargo check (skips execution, only checks for compilarion errors), cargo build, cargo run

- {} -> placeholders, can also be used as {<variable_name>}
- no semicolon for inline type functions.
- Basically rust treats same strings as one entity and multiple constants containing same strings are pointed to the same add;\*

- Variables are immutable by default;
  -- let a:&str="abc";
  -- &a and a as result in same output? Quite Intresting

# Structs

- &self is same as this impl
- is a keyword, that implements/links a function to a struct, like some js classes contain prototype functions.
- any function that doesn;t use self is considered a static function and can be called by the struct name followed by :: and the static function name
- Ex: Rectangle::test();
