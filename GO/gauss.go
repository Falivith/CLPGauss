package main

import (
    "bufio"
    "fmt"
    "os"
    "strconv"
    "strings"
    "time"
)

func main() {
    matriz, err := lerMatriz("../matrix_4x5.txt")

    if err != nil {
        fmt.Println("Erro ao ler a matriz:", err)
        os.Exit(1)
    }

  inicio := time.Now()
    gauss(matriz)
  duracao := time.Since(inicio)

 substituicao_reversa(matriz)
  
  fmt.Printf("Tempo de execução: %s\n", duracao)
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
        numeros := strings.Fields(linha)
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

    return matriz, nil
}

func trocarLinhas(matriz [][]float64, linha_1 int, linha_2 int) {
    matriz[linha_1], matriz[linha_2] = matriz[linha_2], matriz[linha_1]
}

func imprimirMatriz(matriz [][]float64) {
    nCols := len(matriz[0])
    nLins := len(matriz)

    // Encontrar o maior número de dígitos na matriz
    maxDigits := 0
    for i := 0; i < nLins; i++ {
        for j := 0; j < nCols; j++ {
            digits := len(fmt.Sprintf("%.3f", matriz[i][j]))
            if digits > maxDigits {
                maxDigits = digits
            }
        }
    }

    // Imprimir a matriz formatada
    for i := 0; i < nLins; i++ {
        for j := 0; j < nCols; j++ {
            element := fmt.Sprintf("%.3f", matriz[i][j])
            fmt.Printf("%s", element)
            for k := 0; k < maxDigits+1-len(element); k++ {
                fmt.Printf(" ")
            }
        }
        fmt.Println()
    }
  fmt.Println()
}

func gauss(matriz [][]float64)(escalonada [][]float64){
  nCols := len(matriz[0])
  nLins := len(matriz)
  // Percorrendo colunas
  for j := 0; j < nCols - 1; j++ {
    // Percorrendo a coluna J
    for i := j; i < nLins - 1; i++{
      if(matriz[i][j] != 0){
        if(i != j){
          trocarLinhas(matriz, i, j)
        }
        // O Pivô (elemento ij de E) é não nulo    
        // Operações em linha
        for m := j+1; m < nLins; m++ {
          element := -matriz[m][j] / matriz[j][j]
          for n := j; n < nCols; n++ {
            matriz[m][n] += element * matriz[j][n]
          }
        }
      break;
      } else {
        if(i == nLins - 1){
          fmt.Println("Esse sistema não possui solução.")
        }
      }
    }
  }
  return matriz
}

func substituicao_reversa(matriz [][]float64) []float64 {
    nLins := len(matriz)
    resposta := make([]float64, nLins)

    for i := 0; i < nLins; i++ {
        d := nLins - 1 - i
        b := matriz[d][nLins]
        for j := d + 1; j < nLins; j++ {
            b -= matriz[d][j] * resposta[j]
        }
        xd := b / matriz[d][d]
        resposta[d] = xd
        fmt.Printf("x%d = %.3f\n", d+1, xd)
    }

    return resposta
}