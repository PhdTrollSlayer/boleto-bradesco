use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

use barcoders::sym::tf::*;
use barcoders::generators::image::*;

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
        Code{
            id,
            moeda,
            dg_verificador: None,
            fator_venci,
            valor,
            agencia,
            carteira,
            nosso_numero,
            conta,
            zero: "0".to_string()
        }
    }

    pub fn codify(&self) -> String {
        let op = match self.dg_verificador.clone() {
            Some(x) => {x},
            None => {"".to_string()},
        };

        format!("{}{}{}{}{}{}{}{}{}{}", self.id,
                                        self.moeda,
                                        op,
                                        self.fator_venci,
                                        self.valor,
                                        self.agencia,
                                        self.carteira,
                                        self.nosso_numero,
                                        self.conta,
                                        self.zero)
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
        writer.write(&bytes[..]).unwrap();
    }
}
