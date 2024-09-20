# Rust

## Data Type

### Scalar Type

- Integer Type
  - Signed-Integer (the number after 'i' is for the size (number) of bit use for store integer value.)
    - i8
    - i16
    - i32 (default when you just assign an integer value to variable without declare type.)
    - i64
    - i128
    - isize (depending on architecture)
  - Unsigned-Integer (the number after 'u' is for the size (number) of bit use for store integer value.)
    - u8
    - u16
    - u32
    - u64
    - u128
    - usize (depending on architecture)
- Floating-Point Type (the number after 'f' is for the size (number) of bit use for store integer value.)
  - f32
  - f64 (default when you just assign a floating number value to variable without declare type)
- Boolean Type (Using 8 Bits or 1 Byte to store boolean value)
  - bool
- Character Type (Using 32 Bits or 4 Bytes to store character value with represent of Unicode Scalar Value)
  - char

### Compound Type

- Tuple Type
  - (<type1>, <type2>, ..., <typeN>)
- Array Type
  - [<type of array element>; <length of array or number of array elements>]

## Ownership && Borrowing

### Ownership

- Ownership Rule
  - Each value in Rust has an owner.
  - There can be only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
