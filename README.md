# Derive Deref Macro

Crate `derive-deref-rs` is a macro library used to easily defined the Derive and DrerfMut trait of the core library.

## How to use `#[derive(Deref)]`

- Structure with only one field:
> ```rust
> #[derive(Deref)]
> struct MyStruct {
>     a: String
> }
> ```
> this will implement the Deref and DerefMut trait with the field a.

> ```rust
> #[derive(Deref)]
> struct MyStruct(String);
> ```
> This will also work since it's have only one field.

- Structure with multiple fields:
> ```rust
> struct MyStruct {
>     #[deref]
>     a: String,
>     b: String
> }
> ```
> When a structure has multiple field, we need to tell which field will be used to dereference the structure. To do that, simply add `#[deref]` on the field you want to use. You can only use this attribute once, otherwise a compile error will occur.


## Limitation of the macro.
- Does not support Enum and Union
- Only support tuple struct with only one field.
- Does not work on struct without any field.
- Can't dereference a structure with 2 fields with `#[deref]` attribute.

