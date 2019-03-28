# Criador de Boletos do Bradesco

![](codigo_de_barras.png)

#### Alunos: Rodrigo, Gustavo

### Compilação

```
git clone https://github.com/PhdTrollSlayer/boleto-bradesco.git
cd boleto-bradesco
cargo build --release
cd target/release
./boleto -h
```

```
-h,--help             Show this help message and exit
-r                    Carteira (2 dígitos)
-c                    Conta (7 dígitos)
-v                    Valor (10 dígitos)
-a                    Agência (4 dígitos)
-n                    Nosso Numero (11 dígitos, opcional)
-d                    Data de vecncimento (Formato: dd/mm/aaaa)
-o                    Arquivo a ser criado para código de barras (Padrão:
                      codigo_de_barras.png)
```

##### Exemplo de uso

```
./boleto -r 10 -c 3218 -v 200 -a 4521 -d 13/12/2019 -o tst.png
```

###### Saída:
```
Dados: Code {
    id: "237",
    moeda: "9",
    dg_verificador: Some(
        "1"
    ),
    fator_venci: "8102",
    valor: "0000000200",
    agencia: "4521",
    carteira: "10",
    nosso_numero: "00000000000",
    conta: "0003218",
    zero: "0"
}
Codigo final: 23791810200000002004521100000000000000032180
Código de barras gerado
```
