pub fn get_system_id(number: i64) -> String {
  let sum = number + 10;
  let res: String = match sum {
    0..=99 => {
      format!("{}{}", 0, sum)
    },
    _ => format!("{}", sum)
  };
  return res;
}