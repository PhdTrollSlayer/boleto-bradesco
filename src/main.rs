use chrono::prelude::{TimeZone, Utc};
use chrono::Duration;

use barcoders::sym::tf::*;
use barcoders::generators::image::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

// TODO: agrparse, refactoring


fn main() {
    let zero_time = Duration::seconds(Utc.ymd(1997, 10, 7).and_hms(0, 0, 0).timestamp()).num_days();
    let venci = Duration::seconds(Utc.ymd(2019, 6, 16).and_hms(0, 0, 0).timestamp()).num_days();

    /*
    0: Id banco - 3 - 000 para cobrança interna
    1: cod moeda - 1 - 9 para real
    2: digito verificador? - 1 - 0 identifica que não existe
    3: fator vecncimento - 4
    4: valor - 10
    5: agencia do bene - 4
    6: carteira - 2
    7: nmr do nosso numero - 11
    8: conta do bene - 7
    9: zero - 1
    */

    let mut codigo: Vec<String> = Vec::new();

    let id = "237".to_string();
    let moeda = "9".to_string();
    let fator_venci = (venci - zero_time).to_string();
    let valor = fill_size("520", 10);
    let agencia = fill_size("1229", 4);
    let carteira = "25".to_string();
    let nosso_numero = fill_size("", 11);
    let conta = "1229469".to_string();
    let zero = "0".to_string();

    codigo.push(id.clone());
    codigo.push(moeda.clone());
    codigo.push(fator_venci.clone());
    codigo.push(valor.clone());
    codigo.push(agencia.clone());
    codigo.push(carteira.clone());
    codigo.push(nosso_numero.clone());
    codigo.push(conta.clone());
    codigo.push(zero.clone());

    let mut fl: String = String::new();
    for s in codigo {
        fl.push_str(&s);
    }

    let digito_verificador = gen_ver_digit(&fl);
    let mut codigo: Vec<String> = Vec::new();

    codigo.push(id.clone());
    codigo.push(moeda.clone());
    codigo.push(digito_verificador.clone());
    codigo.push(fator_venci.clone());
    codigo.push(valor.clone());
    codigo.push(agencia.clone());
    codigo.push(carteira.clone());
    codigo.push(nosso_numero.clone());
    codigo.push(conta.clone());
    codigo.push(zero.clone());

    let mut fl: String = String::new();
    for s in codigo {
        fl.push_str(&s);
    }

    println!("{}", fl);

    gen_barcode(&fl);

}

fn gen_ver_digit(c: &str) -> String {
    let mut mul = 2;
    let mut result: i32 = 0;

    for u in c.chars().rev() {
        let x: i8 = u.to_string().parse().unwrap();

        if mul > 9 {
            mul = 2;
        }

        result += x as i32* mul as i32;
        mul += 1;
    }

    let resto = result % 11;

    if resto == 0 || resto == 1 || resto > 9 {
        return "1".to_string()
    } else {
        return resto.to_string()
    }

}

fn gen_barcode(code: &str) {
    let bc = TF::interleaved(code).unwrap();
    let img = Image::png(80);
    let encoded = bc.encode();

    let bytes = img.generate(&encoded[..]).unwrap();

    let file = File::create(&Path::new("codigo_de_barras.png")).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

fn fill_size(s: &str, i: usize) -> String {
    let mut f: String = s.chars().rev().collect();

    for _ in 0..(i - s.len()) {
        f.push('0')
    }

    f.chars().rev().collect()
}
