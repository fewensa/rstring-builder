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
