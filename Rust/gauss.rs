use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let arquivo_entrada = "matrix.txt";
    let (linhas, colunas, matriz) = ler_matriz(arquivo_entrada);

    // Imprimir matriz original
    println!("\nMatriz original:");
    imprimir(&matriz, linhas, colunas);

    gaussian_elimination(&mut matriz, linhas, colunas);

    // Liberar memória alocada para a matriz
    for i in 0..linhas {
        drop(matriz[i].as_mut());
    }
    drop(matriz);
}

fn ler_matriz(arquivo_entrada: &str) -> (usize, usize, Vec<Vec<i32>>) {
    let file = File::open(arquivo_entrada).unwrap();
    let reader = BufReader::new(file);
    let mut matriz = Vec::new();
    let (mut linhas, mut colunas) = (0, 0);

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matriz.push(values);
        linhas += 1;
        colunas = values.len();
    }

    (linhas, colunas, matriz)
}

fn gaussian_elimination(matriz: &mut Vec<Vec<i32>>, linhas: usize, colunas: usize) {
    let mut factor;

    // Eliminação
    for i in 0..linhas - 1 {
        // se o pivo é == 0 ele procura em uma linha != 0
        if matriz[i][i] == 0 {
            let mut flag = false;
            for j in i + 1..linhas {
                if matriz[j][i] != 0 {
                    matriz.swap(i, j);
                    flag = true;
                    break;
                }
            }
            if !flag {
                println!("Não é possível realizar a eliminação de Gauss nessa matriz.");
                return;
            }
        }

        // calculos para tornar zero a esquerda do pivo
        for j in i + 1..linhas {
            factor = matriz[j][i] as f64 / matriz[i][i] as f64;
            for k in i..colunas {
                // realiza as operacoes para zerar, de acordo com a linha a mais, imprimar para conseguir visualizar
                // println!("\n{} - ({:.2} * {}) = ", matriz[j][k], factor, matriz[i][k]);
                matriz[j][k] -= (factor * matriz[i][k] as f64) as i32;
                // println!("{}", matriz[j][k]);
            }
            // imprimir(matriz, linhas, colunas);
        }
    }

    // Checar se a matriz é consistente
    if matriz[linhas - 1][linhas - 1] == 0 {
        println!("Não é possível realizar a eliminação de Gauss nessa matriz.");
        return;
    }

    println!("Matriz para a realizacao dos calculos: ");
    imprimir(matriz, linhas, colunas);

        let mut x = vec![0.0; linhas];
    // realiza as operacoes do ultimo para o primeiro
    for i in (0..linhas).rev() {
        x[i] = matriz[i][colunas - 1] as f64;
        for j in i + 1..linhas {
            x[i] -= matriz[i][j] as f64 * x[j];
        }
        x[i] /= matriz[i][i] as f64;
    }

    // Imprime o vetor de soluções
    println!("\nVetor de solucoes:");
    for i in 0..linhas {
        println!("x{} = {}", i + 1, x[i]);
    }
}