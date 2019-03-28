mod helpers;
mod code;

use helpers::*;
use code::*;

use chrono::prelude::{TimeZone, Utc};
use chrono::Duration;

use argparse::{ArgumentParser, Store};

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

fn main() {
    // Inicializa valores padrões
    let id = "237".to_string();
    let moeda = "9".to_string();

    let zero_time = Duration::seconds(Utc.ymd(1997, 10, 7).and_hms(0, 0, 0).timestamp()).num_days();
    let mut venci = String::new();

    let mut carteira = String::new();
    let mut conta = String::new();
    let mut valor = String::new();
    let mut agencia = String::new();
    let mut nosso_numero = String::new();

    let mut output = String::from("codigo_de_barras.png");

    // Recupera argumentos e valores da linha de comando
    {
        let mut ap = ArgumentParser::new();

        ap.set_description("Criador de boletos do Banco Bradesco");
        ap.refer(&mut carteira)
            .add_option(&["-r"], Store, "Carteira (2 dígitos)");
        ap.refer(&mut conta)
            .add_option(&["-c"], Store, "Conta (7 dígitos)");
        ap.refer(&mut valor)
            .add_option(&["-v"], Store, "Valor (10 dígitos)");
        ap.refer(&mut agencia)
            .add_option(&["-a"], Store, "Agência (4 dígitos)");
        ap.refer(&mut nosso_numero)
            .add_option(&["-n"], Store, "Nosso Numero (11 dígitos, opcional)");
        ap.refer(&mut venci)
            .add_option(&["-d"], Store, "Data de vecncimento (Formato: dd/mm/aaaa)");
        ap.refer(&mut output)
            .add_option(&["-o"], Store, "Arquivo a ser criado para código de barras (Padrão: codigo_de_barras.png)");

        ap.parse_args_or_exit();
    }

    // Analisa dados e os organiza no formato correto
    let venci = parse_date(&venci);

    if valor.len() < 1 || agencia.len() < 1 || conta.len() < 1 || carteira.len() < 1 {
        panic!("Erro: Algum valor não foi inserido!")
    }

    valor = fill_size(&valor, 10).unwrap();
    agencia = fill_size(&agencia, 4).unwrap();
    nosso_numero = fill_size(&nosso_numero, 11).unwrap();
    conta = fill_size(&conta, 7).unwrap();
    carteira = fill_size(&carteira, 2).unwrap();

    let fator_venci = (venci - zero_time).to_string();

    // -------------------------------------------------------------
    let mut codigo = Code::constructor(id, moeda, fator_venci, valor, agencia, carteira, nosso_numero, conta);

    codigo.gen_ver_digit();
    println!("Dados: {:#?}", codigo);
    println!("Codigo final: {}", codigo.codify());

    codigo.gen_barcode(&output);
    println!("Código de barras gerado");


    // -------------------------------------------------------------
}
