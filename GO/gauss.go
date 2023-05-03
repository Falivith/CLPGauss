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
    trocarLinhas(matriz, 1, 2)
    fmt.Println(matriz)
    somarLinhas(matriz, 1, 2, 3)
    fmt.Println(matriz)
    multiplicarLinha(matriz, 1, 3)
    fmt.Println(matriz)
}

func lerMatriz(arquivo string) ([][]float64, error) {

    file, err := os.Open(arquivo)
    if err != nil {
        return nil, err
    }

    defer file.Close()

    var matriz [][]float64

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        linha := scanner.Text()
        numeros := strings.Split(linha, " ")
        var linhaMatriz []float64
        for _, numero := range numeros {
            valor, err := strconv.ParseFloat(numero, 64)
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

func trocarLinhas(matriz [][]float64, linha_1 int, linha_2 int) {
    matriz[linha_1], matriz[linha_2] = matriz[linha_2], matriz[linha_1]
}

func somarLinhas(matriz [][]float64, linha_1 int, linha_2 int, linha_destino int) {
    nCols := len(matriz[0])
    for j := 0; j < nCols; j++ {
        matriz[linha_destino][j] = matriz[linha_1][j] + matriz[linha_2][j]
    }
}

func multiplicarLinha(matriz [][]float64, linha int, escalar float64) {
    nCols := len(matriz[0])
    for j := 0; j < nCols; j++ {
        matriz[linha][j] *= escalar
    }
}

func gauss(matriz [][]float64)(escalonada [][]float64){

  
  return escalonada
}