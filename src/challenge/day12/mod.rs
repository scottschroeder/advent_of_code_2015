use anyhow::Result;
use serde_json::Value;

fn count_json(value: Value, ignore_red: bool) -> Result<i64> {
    let mut stack = vec![value];
    let mut total = 0;
    let red = Value::String("red".to_string());
    while let Some(v) = stack.pop() {
        match v {
            Value::Array(arr) => stack.extend(arr.into_iter()),
            Value::Object(map) => {
                if ignore_red && map.values().any(|v| v == &red) {
                    continue;
                }
                stack.extend(map.into_iter().map(|(_, map_v)| map_v));
            }
            Value::Number(x) => {
                x.as_i64()
                    .ok_or_else(|| anyhow::anyhow!("{:?} is not an integer", x))
                    .map(|i| total += i)?;
            }
            _ => {}
        }
    }
    Ok(total)
}

pub fn part1(input: &str) -> Result<String> {
    let input_json: Value = serde_json::from_str(input)?;
    log::debug!("{:#?}", input_json);
    let total = count_json(input_json, false)?;
    Ok(format!("{}", total))
}

pub fn part2(input: &str) -> Result<String> {
    let input_json: Value = serde_json::from_str(input)?;
    log::debug!("{:#?}", input_json);
    let total = count_json(input_json, true)?;
    Ok(format!("{}", total))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day12");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "156366")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "96852")
    }

    #[test]
    fn verify_p1_ex0() {
        assert_eq!(part1("[1,2,3]").unwrap().as_str(), "6")
    }
    #[test]
    fn verify_p1_ex1() {
        assert_eq!(part1(r##"{"a":2,"b":4}"##).unwrap().as_str(), "6")
    }
    #[test]
    fn verify_p1_ex2() {
        assert_eq!(part1(r##"[[[3]]]"##).unwrap().as_str(), "3")
    }
    #[test]
    fn verify_p1_ex3() {
        assert_eq!(part1(r##"{"a":{"b":4},"c":-1}"##).unwrap().as_str(), "3")
    }
    #[test]
    fn verify_p1_ex4() {
        assert_eq!(part1(r##"{"a":[-1,1]}"##).unwrap().as_str(), "0")
    }
    #[test]
    fn verify_p1_ex5() {
        assert_eq!(part1(r##"[-1,{"a":1}]"##).unwrap().as_str(), "0")
    }
    #[test]
    fn verify_p1_ex6() {
        assert_eq!(part1(r##"[]"##).unwrap().as_str(), "0")
    }
    #[test]
    fn verify_p1_ex7() {
        assert_eq!(part1(r##"{}"##).unwrap().as_str(), "0")
    }
    #[test]
    fn verify_p2_ex0() {
        assert_eq!(part2(r##"[1,2,3]"##).unwrap().as_str(), "6")
    }
    #[test]
    fn verify_p2_ex1() {
        assert_eq!(part2(r##"[1,{"c":"red","b":2},3]"##).unwrap().as_str(), "4")
    }
    #[test]
    fn verify_p2_ex2() {
        assert_eq!(
            part2(r##"{"d":"red","e":[1,2,3,4],"f":5}"##)
                .unwrap()
                .as_str(),
            "0"
        )
    }
    #[test]
    fn verify_p2_ex3() {
        assert_eq!(part2(r##"[1,"red",5]"##).unwrap().as_str(), "6")
    }
}
