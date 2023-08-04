# Whale

Whale is data-oriented programming language for system management tasks such as
installing packages, managing disks and getting system information. Whale comes
with a command-line tool and an interactive shell that provides a live REPL with
syntax completion and history.

Whale is minimal, easy to read and easy to learn by example. Your code will
always do exactly what it looks like it's going to do.

A basic whale program:

```whale
output "Hello world!"
```

Whale can do two things at the same time:

```whale
async (
    'output "will this one finish first?"',
    'output "or will this one?"'
)
```

Wait for a file to change, then print the time:

```whale
watch "foo.whale";
output "The time is " + date::time;
```

## Usage

## The Whale Programming Language

Whale started as a hard fork of [evalexpr], which is a simple expression
language and unrelated project. Whale is still very simple but can manage
large, complex sets of data and perform complicated tasks through macros. It
should not take long for a new user to learn the language, especially with the
assistance of the shell.

### Variables

Variables have two parts: a key and a value. The key is always a text string.
The value can be any of the following data types:

- string
- integer
- floating point value
- boolean
- list
- map
- table
- function

Here are some examples of variables in whale.

```whale
x = 1;
y = "hello, it is " + now().time;
z = "42.42";

list = (3, 2, x);
big_list = (x, y, z, list);
```

### Macros

**Macros** are whale's built-in tools. Some of them can reconfigure your whole
system while others are do very little. They may accept different inputs, or
none at all. Functions in the `random` group can be run without input, but the
`integer` function can optionally take two numbers as in inclusive range.

```whale
coin_flip = random_boolean();
number = random_integer();
die_roll = random_integer(1, 6);
```

The **method operator `:`** offers an alternate syntax for passing variables to
macros and functions. Like calling methods in object-oriented languages, the
variable name on the left is passed as the first argument to the macro or
function on the left. You can pass more values using the syntax shown above.

```whale
message = "I hate whale";
message:replace("hate", "love");
```

### Lists

Lists are sequential collections of values. They can be built by grouping
values with parentheses and separating them with commas. Values can be indexed
by their position to access their contents. Lists are used to represent rows
in tables and most macros take a list as an argument.

```whale
list = (true, 42, "Ok");

assert_eq(list:get(0), true);
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

The macros `create_table` and `insert` make sure that all of the memory used to
hold the variables is allocated at once, so it is good practice to group your
rows together instead of using a call for each row.

```whale
animals.all:insert(
    ("eliza", "ostrich", 4),
    ("pat", "white rhino", 7),
    ("jim", "walrus", 9)
);

assert_eq(animals:length(), 6);

animals.by_name = animals:sort_by("name");
```

### Expressions, assignment and the yield operator

Whale has flexible syntax. The following block will print the same thing three
times by passing a raw string to a macro.

```whale
output "hiya";
output("hiya");
"hiya"::output;
```

This block assigns a variable and prints its value three times; first with
a simple macro call, then using method syntax and finally by using the  **yield
operator: `::`**.

```
message = "hiya";

output message;
message:output;
message::output;
```

Like a pipe in bash, zsh or fish, the yield operator evaluates the expression
on the left and passes it as input to the expression on the right. That input is
always assigned to the **`input` variable** for that context. These expressions
may simply contain a value or they can call a macro or function that returns
a value.

```whale
"https://api.sampleapis.com/futurama/characters"::download(input):from_json():get(4)
```

This can be useful when working on the command line but to make a script easier
to read, we can also declare variables.

```whale
json = endpoint:download();
data = json:from_json();
data:get(4)
```

You can use all of this together to write short, elegant scripts.

```whale
characters = download "https://api.sampleapis.com/futurama/characters";
characters:from_json():get(4)
```

### Functions

Functions are first-class values in whale, so they are assigned to variables
like any other value. The function body is wrapped in single parentheses. To
call a function, it's just like calling a macro: simply pass it an argument or
use an empty set of parentheses to pass an empty value.

The **`input` variable** represents whatever value is passed to the function when
called.

```whale
say_hi = 'output "hi"';
add_one = 'input + 1';

assert_eq(add_one(3), 4);
say_hi();
```

This function simply passes the input to the shell's standard output.

```whale
print = 'input:output';
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
