use quote::quote;

use crate::derive_deref_inner;

#[test]
fn test_deref_when_struct_with_one_field() {
    let input = quote! {
        struct Test {
            a: i32,
        }
    };
    let output = derive_deref_inner(input).unwrap();
    let expected = quote! {
        impl core::ops::Deref for Test {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.a
            }
        }

        impl core::ops::DerefMut for Test {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.a
            }
        }
    };
    assert_eq!(expected.to_string(), output.to_string());
}

#[test]
fn test_deref_when_tuple_struct_with_one_field() {
    let input = quote! {
        struct Test (i32);
    };
    let output = derive_deref_inner(input).unwrap();
    let expected = quote! {
        impl core::ops::Deref for Test {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for Test {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    assert_eq!(expected.to_string(), output.to_string());
}

#[test]
fn test_deref_when_struct_with_multiple_field_and_one_deref_attr() {
    let input = quote! {
        struct Test {
            #[deref]
            a: i32,
            b: i32
        }
    };
    let output = derive_deref_inner(input).unwrap();
    let expected = quote! {
        impl core::ops::Deref for Test {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.a
            }
        }

        impl core::ops::DerefMut for Test {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.a
            }
        }
    };
    assert_eq!(expected.to_string(), output.to_string());
}

#[test]
fn test_deref_with_generics() {
    let input = quote! {
        struct Test<T: Debug> {
            a: T
        }
    };
    let output = derive_deref_inner(input).unwrap();
    let expected = quote! {
        impl<T: Debug> core::ops::Deref for Test<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.a
            }
        }

        impl<T: Debug> core::ops::DerefMut for Test<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.a
            }
        }
    };
    assert_eq!(expected.to_string(), output.to_string());
}

#[test]
fn test_deref_with_generics_with_where() {
    let input = quote! {
        struct Test<T> where T: Debug {
            a: T
        }
    };
    let output = derive_deref_inner(input).unwrap();
    let expected = quote! {
        impl<T> core::ops::Deref for Test<T> where T: Debug {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.a
            }
        }

        impl<T> core::ops::DerefMut for Test<T> where T: Debug {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.a
            }
        }
    };
    assert_eq!(expected.to_string(), output.to_string());
}

#[test]
fn test_error_when_struct_with_multiple_field_and_no_deref_attr() {
    let input = quote! {
        struct Test {
            a: i32,
            b: i32
        }
    };
    let output = derive_deref_inner(input);
    let output = output.unwrap_err().to_string();
    assert_eq!(
        "Struct with multiple field need to have the #[deref] attribute on one field.",
        output
    );
}

#[test]
fn test_error_when_struct_with_multiple_deref_attr() {
    let input = quote! {
        struct Test {
            #[deref]
            a: i32,
            #[deref]
            b: i32
        }
    };
    let output = derive_deref_inner(input);
    let output = output.unwrap_err().to_string();
    assert_eq!("Only one field can have the attribute #[deref]", output);
}

#[test]
fn test_error_when_struct_is_empty() {
    let input = quote! {
        struct Test;
    };
    let output = derive_deref_inner(input);
    let output = output.unwrap_err().to_string();
    assert_eq!("Derive Deref cannot deref empty struct.", output);
}

#[test]
fn test_error_when_enum() {
    let input = quote! {
        enum Test {}
    };
    let output = derive_deref_inner(input);
    let output = output.unwrap_err().to_string();
    assert_eq!("Derive Deref does not support Enum.", output);
}

#[test]
fn test_error_when_union() {
    let input = quote! {
        union Test {}
    };
    let output = derive_deref_inner(input);
    let output = output.unwrap_err().to_string();
    assert_eq!("Derive Deref does not support Union.", output);
}
