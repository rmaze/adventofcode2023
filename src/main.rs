/*
Advent of Code 2023 - Day 1 - Part 1.
https://adventofcode.com/2023/day/1

A estratégia utilizada abaixo é percorrer cada linha uma vez,
identificar o primeiro e o último dígito nessa única passagem,
e calcular o valor sem criar estruturas adicionais.

Iteração Única por Linha: Percorri cada linha uma vez,
identificando o primeiro e o último dígito durante esta iteração.

Minimização de Alocações: Evitei criar estruturas adicionais,
reduzindo o uso de memória e o tempo de alocação/desalocação.

Cálculo Direto: Assim que encontro os dígitos necessários,
realizo o cálculo e adiciono ao total.
*/

use std::fs;

fn main() {
    // lendo o arquivo esperando q nd de errado
    let file_path: &str = "src\\day_1.txt";
    match fs::read_to_string(file_path) {
        Ok(calibration_document) => {
            // chamando a função q soma
            let sum: u32 = sum_calibration_values(&calibration_document);
            println!("Sum of all calibration values: {}", sum);
        },
        Err(e) => {
            // n consegui ler o arquivo. :(
            println!("Error reading file: {}", e);
        }
    }
}

// func que soma os valores de calibração de maneira eficiente.
fn sum_calibration_values(calibration_document: &str) -> u32 {
    let mut total: u32 = 0; // começando do zero

    for line in calibration_document.lines() {
        // procurando digitos
        let (mut first_digit, mut last_digit) = (None, None);
        for c in line.chars() {
            if c.is_digit(10) {
                // achei o primeiro
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                // atualizando o ultimo digito
                last_digit = Some(c);
            }
        }

        // se ambos dígitos foram encontrados, faço a soma
        if let (Some(fd), Some(ld)) = (first_digit, last_digit) {
            // calculo o valor
            let value: u32 = fd.to_digit(10).unwrap() * 10 + ld.to_digit(10).unwrap();
            total += value; // adc
        }
    }

    total // soma total
}
