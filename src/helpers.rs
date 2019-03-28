use barcoders::sym::tf::*;
use barcoders::generators::image::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

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

pub fn gen_ver_digit(c: &str) -> String {
    let mut mul = 2;
    let mut result: i32 = 0;

    for u in c.chars().rev() {
        let x: i8 = u.to_string().parse().unwrap();

        if mul > 9 {
            mul = 2;
        }

        result += x as i32 * mul as i32;
        mul += 1;
    }

    let resto = 11 - (result % 11);

    if resto == 0 || resto == 1 || resto > 9 {
        return "1".to_string()
    } else {
        return resto.to_string()
    }

}

pub fn gen_barcode(code: &str) {
    let bc = TF::interleaved(code).unwrap();
    let img = Image::png(80);
    let encoded = bc.encode();

    let bytes = img.generate(&encoded[..]).unwrap();

    let file = File::create(&Path::new("codigo_de_barras.png")).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
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
