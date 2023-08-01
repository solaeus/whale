# Whale

Whale is data-oriented programming language for system management tasks such as
installing packages, managing disks and getting system information. Whale comes
with a command-line tool and an interactive shell that provides a live REPL with
syntax completion and history.

The most basic whale program:

```rust
output "Hello world!"
```

Run two files at the same time:

```rust
whale::async ("foo.whale", "bar.whale")
```

Wait for a file to change, then print the time:

```rust
wait watch "foo.whale";
output "The time is " + date::time;
```

## Usage

### Variables

Variables have two parts: a key and a value. The value can be a string, integer,
floating point value, boolean, list or map. The key is always a string. Here are
some examples of variables in whale.

```ruby
x = 1;
y = "hello, it is " + now().time;
z = "42.42";

list = (3, 2, x);
big_list = (x, y, z, list);
```

### Macros

Macros are whale's built-in tools. Some of them can reconfigure your whole
system while others are do very little. They may accept different inputs, or
none at all. Functions in the `random` module can all run without input, but the
`integer` function can optionally take two numbers as a range or a single number
as the highest allowable result.

```whale
coin_flip = random_boolean();
number = random_integer();
die_roll = random_integer(1, 6);
```

### Expressions, declarations and pipes

Whale has flexible syntax. The following block will print the same thing five
times.

```whale
output "hiya";
output("hiya");
"hiya":output;

message = "hiya";
output message;
message:output;
```

Whale supports pipe-like syntax through expression evaluation.

```
"https://api.sampleapis.com/futurama/characters":download:from_json:get 4
```

This can be useful when working on the command line but to make a script easier
to read, we can also declare variables.

```
endpoint = "https://api.sampleapis.com/futurama/characters";
json = endpoint:download;
data = json:from_json;
data:get 4
```

You can use all of this together to write short, elegant scripts.

```
characters = download "https://api.sampleapis.com/futurama/characters";
characters:from_json:get 4
```

### Maps

Maps are flexible collections with arbitrary key-value pairs, similar to JSON
objects.

```
info.message = "FOOBAR";
info.time = now().timestamp;

info:write "info.txt";
```

### Tables

Tables are strict collections, each row must have a value for each column. Empty
cells must be explicitly set to an empty value. Querying a table is similar to
SQL.

```
animals.all = create_table (
  ("name", "species", "age"),
  (
    ("rover", "cat", 14),
    ("spot", "snake", 9),
    ("bob", "giraffe", 2),
  )
);

animals.by_name = animals:sort_by "name";
animals.oldest = animals:select_where 'species > 5';
```


[evalexpr]: https://github.com/ISibboI/evalexpr
[toolbox]: https://containertoolbx.org
[fish]: https://fishshell.com
