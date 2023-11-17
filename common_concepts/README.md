# RUST
- statical typed language
- that means it has to know the types of all variables at compile time
- Usually the compiler can infer which type we want to use based on the value
- If there are more options possible as type we have to define the type of the value


## Variables and Mutability
- Variable are defined by the _let_ keyword
- By default variables are immutable
- If program does not compile the program is simply not safe enough
- Add _mut_ designate future readers of the code that this variable will be changed on a later place
- Constants are defined by using _const_ keyword
- Constants are not allowed to be mutable, they are always immutable
- the type of the value must be annotated
- Constants may be set only to a constant expression, not the result of a value that could be computed at runtime
- __Shadowing__ is when a new variable is declared with the same name as a previous variable
- Shawdowing is different from marking a variable as _mut_ because we get a compile time error if we try to reassign to this variable without using _let_
- Shawdowing allows to change the type of the value but reuse the same name of the variable
- Variables cannot change their type

## Data Types

- Every value is of a specific data type
- _scalar_ type and _compound_ type

### Scalar Types
- Integer
- Floating-point
- Numbers
- Booleans

#### Integer Types
- number without fractional component
- unasigned intergers: _u8, u16, u32, u64, u128, usize_ can be only be positive
- unasigned integers can store numbers from _-(2n - 1) to 2n - 1 - 1_
- asigned integers: _i8, i16, i32, i64, i128, isize_ can be both negative and positive
- Possible ways to write integer literals: decimal, hex, octal, byte (u8 only), binary
- integer default is __i32__
- __Integer Overflow__: When you assign a u8 annotated value a value > 256 then you get an integer overflow. This results in two behaviours:
    - panic in debug mode
    - complement wrapping in release mode
- the standard libary provides a bunch of methodes to handle the possibility of overflow

#### Floating-point Types
- numbers with decimal points
- two primitive types: _f32_ and _f64_ (32 bits, 64 bits)
- all floating-point types are signed

#### Boolean Type
- _bool_: _true_ and _false_
- 1 Byte size

#### Character Type
- _char_ literals are defined within single quotes
- Attention: _string_ literals are defined in double quotes
- 4 Byte size
- represents Unicode Scalar Value, that means it can represent a lot more than just ASCII (8 bits) (Unicode is a superset of ASCII and is stored in byte sequences as UTF-32 or UTF-8)

### Compound Types
- Tuple type
- Array type

#### Tuple Type
- number of values with a variety of types
- once declared they cannot grow or shrink in size
- e.g: _let tup: (i32, f64, u8) = (500, 6.4, 1)_
- accesing tuple element by index: _tup.0_
- __Unit__ type is a tuple without any values -> ()
- expressions implicitly return the unit value if they don't return any other value

#### Array Type
- arrays have elements of the same type
- arrays have a fixed length
- e.g.: _let a = [3,3,3,3,3];_ or _let a: [i32; 5] = [3,3,3,3,3]_ or _let a: [3;5]
- arrays are __allocated on the stack__ not the heap
- access by indexing: _a[0];_
- If there is an index error it results in a runtime error, the program panics (=ends with an error message)
- Example for memory safety because rust safes you from accessing invalid memory (like in other low level languages) and does not continue instead exits with an error



### Vector Type
- provided by the standard libary which is like an array but can growth or shrink in size