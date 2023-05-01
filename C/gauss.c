#include <stdio.h>
#include <stdlib.h>

void gaussian_elimination(double **matriz, double *result, int tamMatriz);

int main()
{
    
    int i, j, tamMatriz, repeat;
    double **matriz, *result;
  repeat = 0;
  do{
    printf("Entre com o tamanho da matriz: ");
    scanf("%d", &tamMatriz);
    if(tamMatriz <1){
      repeat = 1;
    }
  tamMatriz--;
  }while(repeat);
    
    matriz = (double **)malloc(tamMatriz * sizeof(double *));

    for (i = 0; i < tamMatriz; i++)
    {
        matriz[i] = (double *)malloc((tamMatriz + 1) * sizeof(double));
    }
    result = (double *)malloc(tamMatriz * sizeof(double));

    printf("Entre com os valores da matriz: \n");

    for (i = 0; i < tamMatriz; i++)
    {
        for (j = 0; j < tamMatriz + 1; j++)
        {
            printf("[%d][%d]: ", i, j);
            scanf("%lf", &matriz[i][j]);
        }
    }

    gaussian_elimination(matriz, result, tamMatriz);

    printf("O resultado e: \n");
    for (i = 0; i < tamMatriz; i++)
    {
        printf("x%d = %lf\n", i + 1, result[i]);
    }

    // libera a memória alocada para a matriz e o vetor de solução
    for (i = 0; i < tamMatriz; i++)
    {
        free(matriz[i]);
    }
    free(matriz);
    free(result);

    return 0;
}

void gaussian_elimination(double **matriz, double *result, int tamMatriz)
{
    int i, j, k;
    double temp;

    for (i = 0; i < tamMatriz - 1; i++)
    {
        for (j = i + 1; j < tamMatriz; j++)
        {
            temp = matriz[j][i] / matriz[i][i];
            for (k = i + 1; k < tamMatriz + 1; k++)
            {
                matriz[j][k] -= temp * matriz[i][k];
            }
        }
    }

    for (i = tamMatriz - 1; i >= 0; i--)
    {
        result[i] = matriz[i][tamMatriz];
        for (j = i + 1; j < tamMatriz; j++)
        {
            result[i] -= matriz[i][j] * result[j];
        }
        result[i] /= matriz[i][i];
    }
}
