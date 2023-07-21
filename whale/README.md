# Whale

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

## Variables

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

## Functions

Whale has lots of functions. Some of them can reconfigure your whole system
while others are simple tools. They may accept different inputs, or none at all.
Functions in the `random` module can all run without input, but the `integer`
function can take two numbers as a range or a single number as the highest
allowable result.

```ruby
coin_flip = random::boolean;
number = random::integer;
die_roll = random::integer(1, 6);
```

[toolbox]: https://containertoolbx.org
