pub fn can_serde_parse<T>(input: &T)
where
    T: std::fmt::Debug + std::cmp::PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    println!("input {:?}", input);
    let json = serde_json::to_string_pretty(&input).unwrap();
    println!("json {}", json);
    let parsed = serde_json::from_str::<T>(&json).unwrap();
    println!("parsed {:?}", parsed);

    assert_eq!(input, &parsed);
}

pub fn can_string_parse<T>(input: &T)
where
    T: std::fmt::Debug + std::cmp::PartialEq + ToString + std::str::FromStr,
{
    println!("input {:?}", input);
    let json = input.to_string();
    println!("string {}", json);
    let parsed = json
        .parse::<T>()
        .unwrap_or_else(|_| panic!("failed to parse"));
    println!("parsed {:?}", parsed);

    assert_eq!(input, &parsed);
}
