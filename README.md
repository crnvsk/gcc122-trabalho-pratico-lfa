# GCC122 - Linguagens Formais e Autômatos - Trabalho Prático

## Discentes:

- Nicolas Lopes Tostes Barbosa
- Arthur Rafael Silva Nunes
- João Pedro Alves Carneiro Valadão
- Augusto Inácio Silva Mariano


## Descrição do Trabalho
O objetivo deste trabalho prático é desenvolver um aplicativo que simule uma Máquina de Turing Determinística (MTD) para reconhecimento de linguagens. A MTD é um modelo abstrato de computação que consiste em uma fita infinita dividida em células, onde uma cabeça de leitura/escrita percorre a fita, alterando seu estado de acordo com as transições definidas.

## Funcionalidades
O aplicativo deve ser implementado na linguagem de programação de sua preferência e deve oferecer as seguintes funcionalidades:

### Entrada de Dados:

Receber como entrada um arquivo de texto contendo a descrição da Máquina de Turing (MT).
Receber como entrada a palavra a ser verificada pela MT.

### Saída de Dados:

Gerar um arquivo de texto com todas as configurações da MT até a aceitação ou rejeição da palavra.

### Chamada por Linha de Comando:

O aplicativo deve ser executado por linha de comando, seguindo o formato:

```
> ./mt desc mt1.txt aaabbb saida.txt
```

### Formato do Arquivo de Configuração:

O arquivo de configuração deve formalizar a Máquina de Turing em uma sêxtupla (Q, Σ, Γ, δ, q0, F):
Q: Conjunto de estados (padrão recomendado: q[0-9]*).
Σ: Alfabeto de entrada (padrão recomendado: [a-z]).
Γ: Alfabeto da fita (padrão recomendado: [A-Z], sendo B o símbolo branco).
δ: Função de transição, definida como um conjunto de transições no formato:

```
(qi, a) -> (qj, b, M)
```

Onde qi é o estado atual, a é o símbolo lido, qj é o próximo estado, b é o símbolo a ser escrito, e M é o movimento da cabeça (L para a esquerda, R para a direita).

q0: Estado inicial.
F: Estado(s) final(is).

### Exemplo de Uso
Para exemplificar a execução do aplicativo, considere o seguinte comando:

```
> ./mt desc mt1.txt aaabbb saida.txt
```

Este comando usa o arquivo de descrição mt1.txt para configurar a Máquina de Turing e verifica se a palavra aaabbb é aceita pela máquina. O resultado é registrado no arquivo de saída saida.txt com todas as configurações da MT durante o processo.

### Observações
Certifique-se de seguir os requisitos obrigatórios mencionados acima para garantir o correto funcionamento do aplicativo.