use crate::helpers::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use barcoders::generators::image::*;
use barcoders::sym::tf::*;

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

#[derive(Debug)]
pub struct Code {
    id: String,
    moeda: String,
    dg_verificador: Option<String>,
    fator_venci: String,
    valor: String,
    agencia: String,
    carteira: String,
    nosso_numero: String,
    conta: String,
    zero: String,
}

#[allow(clippy::too_many_arguments)]
impl Code {
    pub fn constructor(
        id: String,
        moeda: String,
        fator_venci: String,
        valor: String,
        agencia: String,
        carteira: String,
        nosso_numero: String,
        conta: String,
    ) -> Code {
        Code {
            id,
            moeda,
            dg_verificador: None,
            fator_venci,
            valor,
            agencia,
            carteira,
            nosso_numero,
            conta,
            zero: "0".to_string(),
        }
    }

    pub fn codify(&self) -> String {
        let op = match self.dg_verificador.clone() {
            Some(x) => x,
            None => "".to_string(),
        };

        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            self.id,
            self.moeda,
            op,
            self.fator_venci,
            self.valor,
            self.agencia,
            self.carteira,
            self.nosso_numero,
            self.conta,
            self.zero
        )
    }

    fn get_campo_livre(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.agencia, self.carteira, self.nosso_numero, self.conta, self.zero
        )
    }

    pub fn gen_digi_line(&self) -> String {
        let cl = self.get_campo_livre();

        let pri = format!("{}{}{}", self.id, self.moeda, cl.get(..5).unwrap());
        let sec = format!("{}", cl.get(5..15).unwrap());
        let ter = format!("{}", cl.get(15..25).unwrap());
        let qur = self.dg_verificador.clone().unwrap();
        let qui = format!("{}{}", self.fator_venci, self.valor);

        let mut dg: Vec<String> = Vec::new();

        dg.push(gen_dac(&pri).to_string());
        dg.push(gen_dac(&sec).to_string());
        dg.push(gen_dac(&ter).to_string());

        format!(
            "{}.{}{} {}.{}{} {}.{}{} {} {}",
            pri.get(..5).unwrap(),
            pri.get(5..).unwrap(),
            dg.get(0).unwrap(),
            sec.get(5..).unwrap(),
            sec.get(5..).unwrap(),
            dg.get(1).unwrap(),
            ter.get(5..).unwrap(),
            ter.get(5..).unwrap(),
            dg.get(2).unwrap(),
            qur,
            qui
        )
    }

    pub fn gen_ver_digit(&mut self) -> i32 {
        let c = self.codify();

        let mut mul: i32 = 2;
        let mut result: i32 = 0;

        for u in c.chars().rev() {
            let x: i32 = u.to_string().parse().unwrap();

            if mul > 9 {
                mul = 2;
            }

            result += x * mul;
            mul += 1;
        }

        let resto = 11 - (result % 11);

        if resto == 0 || resto == 1 || resto > 9 {
            self.dg_verificador = Some("1".to_string());
            1
        } else {
            self.dg_verificador = Some(resto.to_string());
            resto
        }
    }

    pub fn gen_barcode(&self, p: &str) {
        let bc = TF::interleaved(self.codify()).unwrap();
        let img = Image::png(80);
        let encoded = bc.encode();

        let bytes = img.generate(&encoded[..]).unwrap();

        let file = File::create(&Path::new(p)).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes[..]).unwrap();
    }
}
