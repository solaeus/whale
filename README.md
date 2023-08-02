# Whale

Whale is data-oriented programming language for system management tasks such as
installing packages, managing disks and getting system information. Whale comes
with a command-line tool and an interactive shell that provides a live REPL with
syntax completion and history.

A basic whale program:

```whale
output "Hello world!"
```

Whale can do two things at the same time:

```whale
async ( '"will this one finish first?"', '"or will this one?"' )
```

Wait for a file to change, then print the time:

```whale
watch "foo.whale";
output "The time is " + date::time;
```

## Usage

### Variables

Variables have two parts: a key and a value. The value can be a string, integer,
floating point value, boolean, list or map. The key is always a string. Here are
some examples of variables in whale.

```whale
x = 1;
y = "hello, it is " + now().time;
z = "42.42";

list = (3, 2, x);
big_list = (x, y, z, list);
```

### Macros

Macros are whale's built-in tools. Some of them can reconfigure your whole
system while others are do very little. They may accept different inputs, or
none at all. Functions in the `random` group can be run without input, but the
`integer` function can optionally take two numbers as in inclusive range.

```whale
coin_flip = random_boolean();
number = random_integer();
die_roll = random_integer(1, 6);
```

### Lists

Lists are sequential collections of values. They can be built by grouping
values with parentheses and separating them with commas. They can be indexed
by their position to access their contents. Lists are used to represent rows
in tables and most macros take a list as an argument.

```whale
list = (true, 42, "Ok");

assert_eq(list.0, true);
```

### Maps

Maps are flexible collections with arbitrary key-value pairs, similar to JSON
objects.

```whale
info.message = "FOOBAR";
info.time = now().timestamp;

info:to_json:write "info.txt";
```

### Tables

Tables are strict collections, each row must have a value for each column. Empty
cells must be explicitly set to an empty value. Querying a table is similar to
SQL.

```whale
animals.all = create_table (
  ("name", "species", "age"),
  (
    ("rover", "cat", 14),
    ("spot", "snake", 9),
    ("bob", "giraffe", 2),
  )
);
```

Adding new rows to a table is easy with the `+=` syntax. But when working with
large amounts of data, it's best to include the rows in the `create_table` or
`insert` statements. These macros make sure that all of the memory used to hold
the variables is allocated at once.

```whale
animals.all += ("jeremy", "mouse", 1);

animals:insert(("eliza", "ostrich", 2), ("pat", "white rhino", 7));

animals.by_name = animals:sort_by "name";
animals.oldest = animals:select_where("species", 'age > 5');
```

### Expressions, assignment and pipes

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

```whale
"https://api.sampleapis.com/futurama/characters":download:from_json:get 4
```

This can be useful when working on the command line but to make a script easier
to read, we can also declare variables.

```whale
endpoint = "https://api.sampleapis.com/futurama/characters";
json = endpoint:download;
data = json:from_json;
data:get 4
```

You can use all of this together to write short, elegant scripts.

```whale
characters = download "https://api.sampleapis.com/futurama/characters";
characters:from_json:get 4
```

### Functions

Functions are first-class values in whale, so they are assigned to variables
like any other value. The function body is wrapped in single parentheses. To
call a function, it's just like calling a macro: simply pass it an argument or
use an empty set of parentheses to pass an empty value.

The `input` variable represents whatever value is passed to the function when
called.

```whale
say_hi = 'output "hi"';
add_one = 'input + 1';

assert_eq(add_one(3), 4);
say_hi();
```

This function simply prints its input.

```whale
print = 'output input';
print "foobar";
```

Because functions are stored in variables, we can use collections like maps to
organize them.

```whale
math.add = 'input.0 + input.1';
math.subtract = 'input.0 - input.1';

assert_eq(math.add(2, 2), 4);
assert_eq(math.subtract(100, 1), 99);
```

[evalexpr]: https://github.com/ISibboI/evalexpr
[toolbox]: https://containertoolbx.org
[fish]: https://fishshell.com
