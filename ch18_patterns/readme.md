# Chapter 18: Patterns and Matching

Match against the structure of types
Patterns consist of:

- Literals
- Destructured arrays, enums, structs
- Variables
- Wildcards
- Placeholders

If pattern matches we use named pieces of the code
Cover:

- valid pattern places
- refutable vs. irrefutable patterns
- different kinds of pattern syntax

## Places patterns can be used:

### match arms

```
match VALUE {
    PATTERN => EXPRESSION,
    ...
}
```

Match must be exhaustive
`_` is catchall pattern, never binds

### conditional `if let` expressions

`if let`, `else if`, `else if let` can all be mixed
Bound variables must be used inside new scope, the following not valid:
`if let Ok(age) = age && age > 30`
if let not exhaustively checked

### `while let`

`while let Ok(v) = stack.pop() {`

### `for` loops

`for (a, b) in v.iter().enumerate() {` destructure

### `let` Statements

`let PATTERN = EXPRESSION`

`let (x, y, z) = (1, 2, 3);`

### Function parameters

`fn name(&(x, y): &(i32, i32)) {`

## Refutability, when patterns fail to match

Patterns may be refutable, irrefutable.
Irrefutable patterns match any value

`let x = 5` x matches anything

`let Some(x) = value` matches only Some

Function parameters, let, for can only accept irrefutable patterns
If let, while let must be refutable to act on branch
Not-last match arms must be refutable
Last match arm should be irrefutable

## Pattern Syntax

### Literals

```
match x {
    1 => ...
```

### Named variables

`let x = 5`
Note: named variables shadow outer scope

### Multiple patterns

Use `|` for or-like patterns

```
match x {
    1 | 2 => ...
```

### Ranges

```
match x {
    1 ... 5 =>
```

Matches inclusively
Only allowed with numeric or char
Compiler checks if range is empty

### Destructuring

#### Structs

`let Struct { field: alias, ... } = value;`
alias can be omitted

can destructure with literals:

```
match struct {
    Struct { v1, v2: 0 } => ...
```

#### Enums

Can destructure data in enum, depends on shape

```
enum Name {
    NoData,
    NamedFields { f: Type },
    SingleField(Type),
    Tuple(Type1, Type2),
}

match value {
    Name::NoData => ...,
    Name::NamedFields { f } => ...,
    Name::SingleField(a) => ...,
    Name::Tuple(a, b) => ...,
}
```

#### Nested structs and funcs

Can match enum inside enum
`Enum1(Enum2(value)) =>`

#### References

Can use & to get variable holding value ref points at
Ex: iterate over values when have a list of refs
`.map(|&Point { x, y }| ...)`

#### Structs and Tuples

`let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });`

## Ignoring values in pattern

Capture value into `_`
Ignore value `Some(_)`
Use name starting with `_`
Use `..` to ignore remaining

Note: `_name` binds and `_` doesn't
Has ownership implications

### Match Guard

Extra conditions on match arms

```
match num {
    Enum(var) if var = condition => ...,
```

The `|` applies to all match guards
`a | b | c if z` a, b, c all check z

### @ Bindings

@ is used to bind to a value at the same time it's being tested

```
match msg {
    Message::Hello { id: id_variable @ 3..7 } => ...
```

## Legacy patterns: `ref` and `ref mut`

Use `ref` to explicitly request binding to be a reference
Rather than moving value out of the struct
