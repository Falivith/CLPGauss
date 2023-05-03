use std::fs::File;
use std::io::{BufRead, BufReader};

fn ler_matriz(arquivo_entrada: &str) -> (Vec<Vec<i32>>, usize, usize) {
    let file = File::open(arquivo_entrada).unwrap();
    let reader = BufReader::new(file);
    let mut linhas = 0;
    let mut colunas = 0;
    let mut matriz: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let row: Vec<i32> = line.split(' ').map(|s| s.parse().unwrap()).collect();

        if colunas == 0 {
            colunas = row.len();
        }
        matriz.push(row);
        linhas += 1;
    }
    (matriz, linhas, colunas)
}

fn gaussian_elimination(matriz: &mut Vec<Vec<i32>>) {
    let linhas = matriz.len();
    let colunas = matriz[0].len();
    let mut factor: i32;

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
            factor = matriz[j][i] / matriz[i][i];
            for k in i..colunas {
                matriz[j][k] -= factor * matriz[i][k];
            }
        }
    }

    // Checar se a matriz é consistente
    if matriz[linhas - 1][linhas - 1] == 0 {
        println!("Não é possível realizar a eliminação de Gauss nessa matriz.");
        return;
    }

    println!("Matriz para a realizacao dos calculos:");
    imprimir(&matriz);

    let mut x: Vec<i32> = vec![0; linhas];
    // realiza as operacoes do ultimo para o primeiro

    for i in (0..linhas).rev() {
        x[i] = matriz[i][colunas - 1];
        for j in i + 1..linhas {
            x[i] -= matriz[i][j] * x[j];
        }
        x[i] /= matriz[i][i];
    }

    println!("Solução do sistema: {:?}", x);
}

fn imprimir(matriz: &Vec<Vec<i32>>) {
    for i in 0..matriz.len() {
        println!("");
        for j in 0..matriz[i].len() {
            print!("{:?} ", matriz[i][j]);
        }
    }
    println!("");
}

fn main() {
    let arquivo_entrada = "../matrix_3.txt";

    let (mut matriz, linhas, colunas) = ler_matriz(arquivo_entrada);
    println!("\nA matriz tem dimensões {}x{}\n", linhas, colunas);
    // Imprimir matriz original
    println!("\nMatriz original:");
    imprimir(&matriz);

    gaussian_elimination(&mut matriz);

    // Liberar memória alocada para a matriz (não é necessário em Rust, pois o
    // coletor de lixo cuida disso automaticamente)
}
