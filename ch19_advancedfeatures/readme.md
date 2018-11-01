# Advanced Features

These features are rarely used, very specific

- Unsafe rust
- Advanced lifetimes
- Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, newtype pattern
- Advanced types: newtype pattern, type aliases, never type, dynamically sized types
- Advanced functions

## Unsafe rust

Memory safety is enforced by compiler.
Usually better to reject some valid than accept potentially invalid
Unsafe turns off checks and lets you interact with OS etc

Features include:
 - Dereference raw pointer
 - Call unsafe function/method
 - Access or modify mutable static variable
 - Implement unsafe trait

Usually encase unsafe code in safe abstraction

### Deref raw pointer

Raw pointers written as `*const T` or `*mut T`
* is part of the type name

Raw pointers:
 - Allowed to ignore borrowing rules with both immutable and mutable/many mutable to same location
 - Aren't guaranteed to point to valid memory
 - Allowed to be null
 - No auto cleanup

Can create outside unsafe

### Calling unsafe function/method

`unsafe fn name() {`
Marks fn body as unsafe
Example unsafe: split_at_mut
Splits mutable slice, rust only knows that code borrows mutably twice
Doesn't know they aren't overlapping

### Using `extern` external code

Extern always unsafe
`extern "C" {` follows the C Application Binary Interface (ABI)

#### Calling rust from other languages

```
#[no_mangle]
pub extern "C" fn call_from_c() {
    ...
```

### Accessing or modifying mutable static variable

Rust globals are called static
`static SCREAMING_SNAKE_CASE: type = value;`
Always uses the `'static` lifetime
Constants and immutable static are different, static has fixed memory address
Static can me mutable, though unsafe

### Implementing an unsafe trait

A trait is unsafe when at least one method has some invariant that compiler can't verify
`unsafe trait Foo {`
`unsafe impl Foo for i32 {`
Example, Sync and Send: If we use raw pointers have to use unsafe because compiler can't verify

## Advanced lifetimes

 - Lifetime subtyping: ensuring one lives longer than the other
 - Lifetime bounds: specifies lifetime for a ref to a generic type
 - Inheritance of trait object lifetimes: allows compiler to infer trait object lifetimes and when specified
 - Anonymous lifetime: making elision more obvious

### Ensuring one lifetime outlives another with lifetime subtyping

Example: Parser
Context struct holds ref to string being parsed
Error in parser returns ref to portion of string that's borked
Since functions can take ownership of containing struct and return a ref to inner data
Inner data may outlive outer struct, use `OuterStruct<'outer, 'inner: 'outer>` inner lives at least as long as outer

### Lifetime bounds on generic traits

RefCell returns Ref type, keeps track of borrowing rules at runtime
`struct Ref<'a, T>(&'a T);`
Doesn't know how long T will last
T could be ref that holds more refs
Instead:
`struct Ref<'a, T: 'a>(&'a T);`
Specifies T can be any type, if it contains refs they must live at east as long as 'a

Another solution:
`struct StaticRef<T: 'static>(&'static T);`
No refs is equivalent to a ref that lives forever ('static)

### Inheritance of trait object lifetimes

Trait objects are trait behind reference, dynamic dispatch

 - Default lifetime of Trait object is 'static
 - With &'a Trait or &'a mut Trait the default lifetime is 'a
 - With a single T: 'a clause the default lifetime is 'a
 - With multiple clauses like T: 'a there is no default lifetime, must be explicit

Can add lifetime bound on lifetime object (`Box<dyn Red>`) using syntax:
`Box<dyn Red + 'static>` or `Box<dyn Red + 'a>`

### The anonymous lifetime

Can use `'_` to reference the elided lifetime to reduce clutter
`fn foo<'a>(string: &'a str) -> StrWrap<'a> {`
is equivalent to
`fn foo(string: &str) -> StrWrap<'_> {`

## Advanced Traits

### Specifying placeholder types in trait defenitions with associated types

Associated Types are a type placeholder that can be used in trait method definitions
Can define trait that uses types without knowing exactly what types are

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Difference from generics is that generics must define type every time used, can define multiple types
Associated types specify the exact type and it doesn't need to be repeated

### Default generic type parameters, operator overloading

Can specify default concrete type when declaring generic type
`<PlaceholderType=ConcreteType>`

Rust doesn't allow creation of own operators, or overload arbitrary operators
Operators have traits assocaited with them

Definition of std::ops::Add:
```
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

RHS defaults to Self, can use Output to change

Default type parameters can be used:
 - To extend a type without breaking an existing class
 - Allowing customization in places where most users won't need

### Fully qualified syntax for methods with the same name

Can impl multiple traits with same name methods, or method on impl with same name as trait
Example cases:
 - Impl trait methods and method on struct all the same name
 - Impl trait with assoc. func and a type also with assoc func

Fully qualified syntax:
`<Type as Trait>::function(reciever_if_method, next_arg, ...);`

### Using supertraits to require one traits functionality within another trait

When a trait depends on another traits functionality it is a supertrait

Example: OutlinePrint that puts an outline around something that impl fmt::Display
```
trait OutlinePrint: fmt::Display {
    ...
```

### Using the newtype pattern to implement external traits on external types

Orphan rule: Only allowed to impl trait as long as trait or type are local

Example, want to impl Display on Vec<T>
Use tuple with only one item, thus type is local
If we want the newtype to impl all methods of wrapped type could impl deref to return inner

## Advanced types

 - Newtypes in general
 - Type aliases
 - ! type
 - Dynamically sized types

### Newtype pattern for safety and abstraction

Can be used to statically enforce that values aren't confused
Indicate units
New type can abstract (now private) inner type api
No abstraction overhead

### Creating type synonyms with type aliases

`type AliasName = i32`

Can add `AliasName + i32` without issue
Used to shorten names:
`type Thunk = Box<dyn Fn() + Send + 'static>`
In std::io:
`type Result<T> = Result<T, std::io::Error>;`

### The type that never returns

Aka the Never type
continue has the ! type so that:
```
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
}
```
Expressions of type ! can be coerced into any other type
In the Err case control flows back into the loop so guess is never assigned
Panic has value !
Unqualified loop has type !, never returns

### Dynamically sized types and Sized trait

aka DSTs or unsized types
str is a dynamically sized type
Need to know how much mem to allocate, all types take up same mem
Use &str cause slice always takes same amount of mem
Dynamically sized types are always behind a pointer with meta information
Can use any ref, such as `Box<str>` or `Rc<str>`

Sized trait auto-added to things whose type is known at compile time
`fn generic<T>(t: T)`
is treated as
`fn generic<T: Sized>(t: T)`

`fn generic<T: ?Sized>(t: &T)` relaxes restriction
Reads as T may or may not be Sized
Only available for Sized

## Advanced functions and closures

### Function pointers

Can pass closures to functions, can also pass regular functions to functions
Functions coerce to type fn, called a function pointer
Function pointers impl Fn, FnMut, and FnOnce
Thus can always pass function pointer to function expecting closure

Example of only wanting to accept fn and not closures:
C functions can accept functions as arguments but doesn't have closures

### Returning closures

Closures are represented as Traits, can't return directly
```
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```
Won't compile, doesn't have static size

Instead use trait object
```
fn returns_closure() -> Box<dyn Fn(i32) -> i32 {
    Box::new(|x| x + 1)
}
```
