rstring-builder
===

[![Build Status](https://drone.0u0.me/api/badges/fewensa/rstring-builder/status.svg)](https://drone.0u0.me/fewensa/rstring-builder)

This create is a string builder type. If you want support append your type, you can impl `Vcharsable` and then `append` your struct.

`rstring-builder` is designed to be character built, so `Vcharsable` will return `Vec<char>` and `rstring-bulder` is actually maintaining this array.

Now support `&str` `String` `OsStr` and `char`.

# Usage

```toml
[dependencies]
rstring-builder = "0.1"
```

# Example

```rust
use rstring_builder::StringBuilder;

#[test]
fn test_string_builder() {
  let mut builder = StringBuilder::new();
  builder.append("a")
    .append('b')
    .append("c".to_string())
    .append("\ndef");

  assert_eq!("bc\ndef".to_string(), builder.delete_at(0).string());
  assert_eq!("bdef".to_string(), builder.delete(1, 3).string());
  assert_eq!(4, builder.len());
  assert_eq!("b".to_string(), builder.delete(1, builder.len()).string());
  assert_eq!("".to_string(), builder.clear().to_string());
}
```

# License
  
MIT

