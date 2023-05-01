use std::io::{stdin, stdout, Write};
use std::mem::size_of;
use std::process::exit;

fn gaussian_elimination(matriz: &mut Vec<Vec<f64>>, result: &mut Vec<f64>, tam_matriz: usize) {
    for i in 0..tam_matriz - 1 {
        for j in i + 1..tam_matriz {
            let temp = matriz[j][i] / matriz[i][i];
            for k in i + 1..tam_matriz + 1 {
                matriz[j][k] -= temp * matriz[i][k];
            }
        }
    }

    for i in (0..tam_matriz).rev() {
        result[i] = matriz[i][tam_matriz];
        for j in i + 1..tam_matriz {
            result[i] -= matriz[i][j] * result[j];
        }
        result[i] /= matriz[i][i];
    }
}

fn ler_arquivo(arquivo_entrada: &str, num_enderecos: usize) -> Vec<u32> {
    use std::fs::File;
    use std::io::Read;

    let mut file = match File::open(arquivo_entrada) {
        Ok(file) => file,
        Err(_) => {
            println!("Erro ao abrir o arquivo.");
            exit(EXIT_FAILURE);
        }
    };

    let mut buf = vec![0u8; size_of::<u32>() * num_enderecos];
    match file.read_exact(&mut buf) {
        Ok(_) => (),
        Err(_) => {
            println!("Erro ao ler o arquivo.");
            exit(EXIT_FAILURE);
        }
    };

    let mut result = vec![0u32; num_enderecos];
    result.copy_from_slice(&buf);

    result
}

fn main() {
    let mut tam_matriz = 0;
    while tam_matriz < 1 {
        print!("Entre com o tamanho da matriz: ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        tam_matriz = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida, tente novamente.");
                continue;
            }
        };

        if tam_matriz < 1 {
            println!("O tamanho da matriz deve ser maior ou igual a 1.");
        }
    }

    let mut matriz = vec![vec![0f64; tam_matriz + 1]; tam_matriz];
    let mut result = vec![0f64; tam_matriz];

    println!("Entre com os valores da matriz:");
    for i in 0..tam_matriz {
        for j in 0..tam_matriz + 1 {
            print!("[{}][{}]: ", i, j);
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let val = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Entrada inválida, tente novamente.");
                    j -= 1;
                    continue;
                }
            };

            matriz[i][j] = val;
        }
    }

    gaussian_elimination(&mut matriz, &mut result, tam_matriz);

    println!("O resultado é:");
    for i in 0..tam_matriz {
        println!("x{} = {}", i + 1, result[i]);
    }

    // limpa a memória alocada para