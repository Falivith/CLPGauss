#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int **lerMatriz(char *arquivoEntrada, int *linhas, int *colunas);

void gaussian_elimination(int **matriz, int linhas, int colunas);

void imprimir(int **matriz, int linhas, int colunas);

int main()
{  
    time_t start = time(NULL);
    int tempo = 0;
    
    char *arquivoEntrada = "../matrix_300x301.txt";

    int i, linhas = 0, colunas = 0;
    int **matriz;

    matriz = lerMatriz(arquivoEntrada, &linhas, &colunas);

    // Imprimir matriz original
    //printf("\nMatriz original:");
    //imprimir(matriz, linhas, colunas);
  
    tempo = clock();
    gaussian_elimination(matriz, linhas, colunas);
    tempo = clock() - tempo;
  
    printf("\nTempo Gasto: %lf \n", ((double)(tempo) / CLOCKS_PER_SEC));
    // Liberar memória alocada para a matriz
    for (i = 0; i < linhas; i++)
    {
        free(matriz[i]);
    }
    free(matriz);

    return 0;
}

int **lerMatriz(char *arquivoEntrada, int *linhas, int *colunas)
{
    FILE *fp;
    char ch;
    int i, j;

    // abrir arquivo
    fp = fopen(arquivoEntrada, "r");
    if (fp == NULL)
    {
        printf("Não foi possível abrir o arquivo.\n");
        exit(1);
    }
    // determinar o número de linhas e colunas
    while ((ch = fgetc(fp)) != EOF)
    {
        if (ch == '\n')
        {
            (*linhas)++;
        }
        else if (*linhas == 0 && ch == ' ')
        {
            (*colunas)++;
        }
    }
    (*colunas)++;
    (*linhas)++;

    // voltar para o início do arquivo
    rewind(fp);

    // alocar matriz
    int **matriz = (int **)malloc((*linhas) * sizeof(int *));
    for (i = 0; i < (*colunas); i++)
    {
        matriz[i] = (int *)malloc((*colunas) * sizeof(int));
    }

    // preencher matriz com valores do arquivo
    for (i = 0; i < (*linhas); i++)
    {
        for (j = 0; j < (*colunas); j++)
        {
            int ret = fscanf(fp, "%d", &matriz[i][j]);
            if (ret != 1)
            {
                printf("Erro ao ler o valor do arquivo\n");
                fclose(fp);
                exit(1);
            }

            if (j < (*colunas) - 1)
            {
                // descartar tabulação
                fgetc(fp);
            }
        }
    }

    // fechar arquivo
    fclose(fp);

    return matriz;
}

void gaussian_elimination(int **matriz, int linhas, int colunas)
{
    int i, j, k;
    double factor;

    // Eliminação
    for (i = 0; i < linhas - 1; i++)
    {
        // se o pivo é == 0 ele procura em uma linha != 0
        if (matriz[i][i] == 0)
        {
            int flag = 0;
            for (j = i + 1; j < linhas; j++)
            {
                if (matriz[j][i] != 0)
                {
                    int *temp = matriz[i];
                    matriz[i] = matriz[j];
                    matriz[j] = temp;
                    flag = 1;
                    break;
                }
            }
            if (flag == 0)
            {
                printf("Não é possível realizar a eliminação de Gauss nessa matriz.\n");
                return;
            }
        }

        // calculos para tornar zero a esquerda do pivo
        for (j = i + 1; j < linhas; j++)
        {
            factor = matriz[j][i] / matriz[i][i];
            for (k = i; k < colunas; k++)
            {
                // realiza as operacoes para zerar, de acordo com a linha a mais, imprimar para conseguir visualizar
                // printf("\n%d - (%.2lf * %d) = ",matriz[j][k], factor, matriz[i][k]);
                matriz[j][k] -= factor * matriz[i][k];
                // printf("%d\n",matriz[j][k]);
            }
            // imprimir(matriz, linhas, colunas);
        }
    }

    // Checar se a matriz é consistente
    if (matriz[linhas - 1][linhas - 1] == 0)
    {
        printf("Não é possível realizar a eliminação de Gauss nessa matriz.\n");
        return;
    }

    //printf("Matriz para a realizacao dos calculos: ");
    //imprimir(matriz, linhas, colunas);

    double x[linhas];
    // realiza as operacoes do ultimo para o primeiro

    for (i = linhas - 1; i >= 0; i--)
    {
        x[i] = matriz[i][colunas - 1];
        for (j = i + 1; j < linhas; j++)
        {
            // printf("\n%.2lf - (%d * %.2lf) = ",x[i], matriz[i][j], x[j]);
            x[i] -= matriz[i][j] * x[j];
            // printf("%.2lf\n", x[i]);
        }
        // printf("%.2lf / %d = ",x[i], matriz[i][i]);
        x[i] /= matriz[i][i];
        // printf("%.2lf\n", x[i]);
    }

    // Imprimir solução
    printf("\nSolucao:\n");
    for (i = 0; i < linhas; i++)
    {
        printf("x%d = %2.lf\n", i + 1, x[i]);
    }
}

void imprimir(int **matriz, int linhas, int colunas)
{
    printf("\n\n");
    for (int i = 0; i < linhas; i++)
    {
        for (int j = 0; j < colunas; j++)
        {
            printf("%d ", matriz[i][j]);
        }
        printf("\n");
    }
    printf("\n");
}
