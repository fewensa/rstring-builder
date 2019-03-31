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

#[test]
fn test_bool() {
  let string = StringBuilder::new()
    .append(false)
    .append(true)
    .string();
  println!("{:?}", string);
}

#[test]
fn test_num() {
  let string = StringBuilder::new()
    .append(9003 as usize)
    .append(2 as u8)
    .append(-1 as i8)
    .append(3 as i128)
    .append(0.3 as f32)
    .append(8.9 as f64)
    .string();
  println!("{:?}", string);
}
