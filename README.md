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

## Feautures

- Data-oriented language based on expressions and variable assignment
- Declarative syntax
- Small and easy to learn
- Manage immutable operating systems
- Extensive features for containerized workflows

## Motivation

Using the command line is like speaking a language: you learn idioms and develop
preferences based on how you use it. The hard part is knowing *how* to say
what you want. Let's say we want to partition a hard drive. We need to know
the device path, label type, partition name, filesystem type and byte range.
With whale, we simply describe the desired state in a file and let one of the
language's functions do the rest.

```rust
path = "/dev/sda";
label = "gpt";
name = "partition1";
filesystem = "ext4";
range = ("1Mib", "8MiB");
```

Then run the function with the name of the file.

```rust
disk::partition "my_partition.whale"
```

Here's the same code using a conventional shell with GNU's parted.

```sh
parted /dev/sda mklabel gpt mkpart partition1 ext4 1MiB 8MiB 
```

It fits nicely on one line, but after a few months you may want to make a change
and you will wish it was more explicit. The traditional Unix command has only
values, but whale has key-value pairs. The names of the keys are easier to
remember than how to order the information.

## Implementation

Whale serves its use cases in the easiest and most obvious way possible. It
implements solutions with the same philosophy: the `disk::partition` function
uses GNU's parted, acting as a simple API layer when possible. Whale always uses
[fish] as a shell when running commands. This is because other shells do not
support user-modified commands in non-interactive mode.

Whale is highly focused on serving users of immutable operating systems and
containerized workflows. While it works an any system, it has advanced features
such as layering packages on the base OS and package management inside the
container.

Whale is a hard fork of [evalexpr]. Thanks to the creator and contributors of
that project.

## Usage

Whale has variables that hold data and functions that use data. If we declare
a [toolbox] container as a variable, whale can use the data to create the
container from scratch. Maintaining the container is as easy as maintaining that
file. We can also write this in the whale shell instead of a file.

```rust
name = "toolbox:" + time::date;
image = "fedora-toolbox:38";
copr = ("varlad/helix");
packages = ("fzf", "helix", "ripgrep");
```

Whale is based on **expressions**, so `"toolbox:" + time::date` will evaluate to
`toolbox:<today's date>`. When passed to the `output` function, the above file
looks like this.

```txt
(
  copr = "varlad/helix";
  image = "fedora-toolbox:38"; 
  name = "toolbox:2023-07-03"; 
  packages = ("fzf", "helix", "ripgrep");
)
```

When passed to the `toolbox::build` function, whale will create a new toolbox
container based on our declaration. Whale is easier to read, write and maintain
than a shell script. Unlike a scripting language, the language itself handles
most of the business logic while our code focuses on the data.

### Variables

Variables have two parts: a key and a value. The value can be a string, integer,
floating point value, boolean, list or map. The key is always a string. Here are
some examples of variables in whale.

```ruby
x = 1;
y = "hello";
z = "42.42";

list = (3, 2, x);
big_list = (x, y, z, list);

map.text = "nested value"
map.list = big_list;
nested.maps.work.too = "!"
```

### Functions

Whale has lots of functions. Some of them can reconfigure your whole system
while others are simple tools. They may accept different inputs, or none at all.
Functions in the `random` module can all run without input, but the `integer`
function can optionally take two numbers as a range or a single number as the
highest allowable result.

```ruby
coin_flip = random::boolean;
number = random::integer;
die_roll = random::integer(1, 6);
```

[evalexpr]: https://github.com/ISibboI/evalexpr
[toolbox]: https://containertoolbx.org
[fish]: https://fishshell.com
