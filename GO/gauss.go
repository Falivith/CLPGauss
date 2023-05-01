package main

import (
    "bufio"
    "fmt"
    "os"
    "strconv"
    "strings"
)

func main() {
    matriz, err := lerMatriz("../matrix.txt")
  
    if err != nil {
        fmt.Println("Erro ao ler a matriz:", err)
        os.Exit(1)
    }
  
    fmt.Println(matriz)
    println(len(matriz))
  
  
    fmt.Println(matriz)
}



func lerMatriz(arquivo string) ([][]int, error) {
  
    file, err := os.Open(arquivo)
    if err != nil {
        return nil, err
    }
  
    defer file.Close()

    var matriz [][]int

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        linha := scanner.Text()
        numeros := strings.Split(linha, " ")
        var linhaMatriz []int
        for _, numero := range numeros {
            valor, err := strconv.Atoi(numero)
            if err != nil {
                return nil, err
            }
            linhaMatriz = append(linhaMatriz, valor)
        }
        matriz = append(matriz, linhaMatriz)
    }

    if err := scanner.Err(); err != nil {
        return nil, err
    }

    return matriz, nil
}

func imprimirMatriz(matriz [][]int, n int){
  
  print(matriz)
}