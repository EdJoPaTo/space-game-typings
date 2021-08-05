pub fn can_serde_parse<T>(input: &T)
where
    T: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned + std::cmp::PartialEq,
{
    println!("input {:?}", input);
    let json = serde_json::to_string_pretty(&input).unwrap();
    println!("json {}", json);
    let parsed = serde_json::from_str::<T>(&json).unwrap();
    println!("parsed {:?}", parsed);

    assert_eq!(input, &parsed);
}
