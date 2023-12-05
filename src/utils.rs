use std::str::FromStr;

pub fn parse_spaced_string<T: FromStr>(string: &str) -> impl Iterator<Item = T> + '_ {
    string.split_whitespace().filter_map(|s|s.parse().ok())
}