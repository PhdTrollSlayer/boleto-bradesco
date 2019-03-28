mod helpers;

use helpers::*;

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
    let mut codigo: String = String::new();

    let id = "237".to_string();
    let moeda = "9".to_string();
    let zero = "0".to_string();

    let zero_time = Duration::seconds(Utc.ymd(1997, 10, 7).and_hms(0, 0, 0).timestamp()).num_days();
    let mut venci = String::new();

    let mut carteira = String::new();
    let mut conta = String::new();
    let mut valor = String::new();
    let mut agencia = String::new();
    let mut nosso_numero = String::new();

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
            .add_option(&["-n"], Store, "Nosso Numero (11 dígitos, opicional)");
        ap.refer(&mut venci)
            .add_option(&["-d"], Store, "Data de vecncimento (Formato: 13/10/2019)");

        ap.parse_args_or_exit();
    }

    // Analisa dados e os organiza no formato correto
    let venci = parse_date(&venci);

    valor = fill_size(&valor, 10).unwrap();
    agencia = fill_size(&agencia, 4).unwrap();
    nosso_numero = fill_size(&nosso_numero, 11).unwrap();
    conta = fill_size(&conta, 7).unwrap();
    carteira = fill_size(&carteira, 2).unwrap();

    let fator_venci = (venci - zero_time).to_string();

    // Cria codigo preliminar e gera o digito verificador
    codigo.push_str(&id.clone());
    codigo.push_str(&moeda.clone());
    codigo.push_str(&fator_venci.clone());
    codigo.push_str(&valor.clone());
    codigo.push_str(&agencia.clone());
    codigo.push_str(&carteira.clone());
    codigo.push_str(&nosso_numero.clone());
    codigo.push_str(&conta.clone());
    codigo.push_str(&zero.clone());

    let digito_verificador = gen_ver_digit(&codigo);

    // Cria codigo final e gera o código de barras
    let mut codigo = String::new();

    codigo.push_str(&id.clone());
    codigo.push_str(&moeda.clone());
    codigo.push_str(&digito_verificador.clone());
    codigo.push_str(&fator_venci.clone());
    codigo.push_str(&valor.clone());
    codigo.push_str(&agencia.clone());
    codigo.push_str(&carteira.clone());
    codigo.push_str(&nosso_numero.clone());
    codigo.push_str(&conta.clone());
    codigo.push_str(&zero.clone());

    println!("{}", codigo);

    gen_barcode(&codigo);

}
