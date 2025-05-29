use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;

pub fn load_from_json<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monsters::Monster;
    use crate::utilities::MONSTERS_JSON_PATH;

    #[test]
    fn loads_from_json() {
        let monsters = load_from_json::<Vec<Monster>>(MONSTERS_JSON_PATH);

        assert_eq!(monsters.first().unwrap().name, "Aboleth");
    }
}
