use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let matriz = ler_matriz("../matrix_4x5.txt")?;

    let inicio = Instant::now();
    let matriz_escalonada = gauss(matriz);
    let duracao = inicio.elapsed();

    substituicao_reversa(matriz_escalonada);

    println!("Tempo de execução: {:?}", duracao);

    Ok(())
}

fn ler_matriz(arquivo: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let file = File::open(arquivo)?;
    let reader = BufReader::new(file);

    let mut matriz = Vec::new();

    for line in reader.lines() {
        let linha = line?;
        let numeros: Vec<f64> = linha
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matriz.push(numeros);
    }

    Ok(matriz)
}

fn trocar_linhas(matriz: &mut Vec<Vec<f64>>, linha_1: usize, linha_2: usize) {
    matriz.swap(linha_1, linha_2);
}

fn imprimir_matriz(matriz: &Vec<Vec<f64>>) {
    let n_cols = matriz[0].len();
    let n_lins = matriz.len();

    // Encontrar o maior número de dígitos na matriz
    let mut max_digits = 0;
    for i in 0..n_lins {
        for j in 0..n_cols {
            let digits = format!("{:.3}", matriz[i][j]).len();
            if digits > max_digits {
                max_digits = digits;
            }
        }
    }

    // Imprimir a matriz formatada
    for i in 0..n_lins {
        for j in 0..n_cols {
            let element = format!("{:.3}", matriz[i][j]);
            print!("{}", element);
            for _k in 0..max_digits + 1 - element.len() {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn gauss(mut matriz: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n_cols = matriz[0].len();
    let n_lins = matriz.len();

    // Percorrendo colunas
    for j in 0..n_cols - 1 {
        // Percorrendo a coluna J
        for i in j..n_lins - 1 {
            if matriz[i][j] != 0.0 {
                if i != j {
                    trocar_linhas(&mut matriz, i, j);
                }
                // O Pivô (elemento ij de E) é não nulo
                // Operações em linha
                for m in j + 1..n_lins {
                    let element = -matriz[m][j] / matriz[j][j];
                    for n in j..n_cols {
                        matriz[m][n] += element * matriz[j][n];
                    }
                }
                break;
            } else {
                if i == n_lins - 1 {
                    println!("Esse sistema não possui solução.");
                }
            }
        }
    }

    matriz
}

fn substituicao_reversa(matriz: Vec<Vec<f64>>) -> Vec<f64> {
    let n_lins = matriz.len();
    let mut resposta = vec![0.0; n_lins];

    for i in 0..n_lins {
        let d = n_lins - 1 - i;
        let mut b = matriz[d][n_lins];
        for j in d + 1..n_lins {
            b -= matriz[d][j] * resposta[j];
        }
        let xd = b / matriz[d][d];
        resposta[d] = xd;
        println!("x{} = {:.3}", d + 1, xd);
    }

    resposta
}