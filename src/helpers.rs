use chrono::prelude::{TimeZone, Utc};
use chrono::Duration;

// Formatação de datas
pub fn parse_date(s: &str) -> i64 {
    let mut s = s.to_string();
    let mut v = Vec::<i32>::new();

    let i = s.find('/').unwrap();
    v.push(s.get(0..i).unwrap().parse().unwrap());
    s = s.get(i+1..).unwrap().to_string();

    let u = s.find('/').unwrap();

    v.push(s.get(..u).unwrap().parse().unwrap());
    s = s.get(u+1..).unwrap().to_string();

    v.push(s.parse().unwrap());

    Duration::seconds(Utc.ymd(v[2], v[1] as u32, v[0] as u32).and_hms(0, 0, 0).timestamp()).num_days()
}

pub fn fill_size(s: &str, i: usize) -> Result<String, ()> {
    if s.len() > i {
        return Err(())
    }

    let mut f: String = s.chars().rev().collect();

    for _ in 0..(i - s.len()) {
        f.push('0')
    }

    Ok(f.chars().rev().collect())
}
