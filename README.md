# REBAR

Python interpreter, written in Rust. Incredibly slow and inefficient (for now?).

## WORKING

### scanner.rs

* mutable borrow of self.history
* ensure no recompile of regex patterns
* think about where to store unit tests
* how to raise syntax errors//how they should be typed
   * particularly whitespace
* attempt a rewrite of pattern matching

### main.rs

* more efficient passing of scanner, state structs down parser chain (rewrite as struct and pass &self?)
* think about handling nested scopes with State hashmap
* implement Exit, List, Clear keywords
* Update parse_newline() to handle multiline statements
* update parse_var to raise NameError on var not found
* add sqrt and negative numbers to parse_factor
