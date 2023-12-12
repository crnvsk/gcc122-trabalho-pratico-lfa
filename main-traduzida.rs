// Este programa lê uma descrição de uma máquina de Turing a partir de um arquivo de
// configuração, inicializa uma fita com uma palavra de entrada, executa a máquina de 
// Turing e escreve o resultado em um arquivo de saída. O código está organizado em 
// funções para facilitar a compreensão e manutenção.

#![allow(dead_code)] // Permite códigos não utilizados sem emitir um aviso

// importa as bibliotecas
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

// Estrutura que representa uma transição da máquina de Turing
struct Transicao {
    estado_origem: String,
    simbolo_lido: char,
    estado_destino: String,
    simbolo_escrito: char,
    direcao_movimento: char,
}

// Estrutura que representa uma máquina de Turing
struct MaquinaDeTuring {
    estados: Vec<String>,
    alfabeto: Vec<String>,
    alfabeto_fita: Vec<String>,
    transicoes: Vec<Transicao>,
    estado_inicial: String,
    estados_aceitacao: Vec<String>,
}

// Função auxiliar para ler uma lista de uma linha e retornar um vetor de Strings
fn ler_lista_da_linha(linha: &str) -> Vec<String> {
    linha.trim_matches(|c| char::is_ascii_punctuation(&c)) // Limpa os caracteres de pontuacao como espaços e virgulas
        .split(',') // Divide a linha em substrings usando a virgula como delimitador 
        .filter(|s| !s.is_empty()) // Verifica se uma substring esta vazia e remove caso esteja
        .map(|s| s.to_string())  // Converte cada substring restante em uma String e cria um novo iterador contendo essas strings
        .collect() // Coleta os elementos do iterador em um vetor (Vec<String>)
}

// Função auxiliar que tem como objetivo processar uma linha de texto contendo informações 
// sobre uma transição em uma máquina de Turing e retornar uma instância da estrutura Transicao
fn ler_transicao_da_linha(linha: &str) -> Transicao {
    let partes: Vec<&str> = linha.split("->").collect(); // Divide a linha em duas partes usando a string "->" como delimitador. O resultado é um vetor de substrings contendo a parte antes e depois da seta "->"
    let simbolo_estado_origem: Vec<String> = ler_lista_da_linha(partes[0]); // Chama a função ler_lista_da_linha para processar a primeira parte da linha, que contém informações sobre o estado de origem e o símbolo lido
    let para_tripla: Vec<String> = ler_lista_da_linha(partes[1]); // Chama novamente a função ler_lista_da_linha para processar a segunda parte da linha, que contém informações sobre o estado de destino, o símbolo a ser escrito e a direção do movimento

    let estado_origem = &simbolo_estado_origem[0]; // Obtém a primeira string do vetor simbolo_estado_origem, que representa o estado de origem
    let simbolo_lido = simbolo_estado_origem[1].chars().next().unwrap(); // Obtém o segundo elemento do vetor simbolo_estado_origem como uma string, converte para um iterador de caracteres e pega o primeiro caractere. Este caractere representa o símbolo lido

    let estado_destino = &para_tripla[0]; //  Obtém a primeira string do vetor para_tripla, que representa o estado de destino
    let simbolo_escrito = para_tripla[1].chars().next().unwrap(); // Obtém o segundo elemento do vetor para_tripla como uma string, converte para um iterador de caracteres e pega o primeiro caractere. Este caractere representa o símbolo a ser escrito
    let direcao_movimento = para_tripla[2].chars().next().unwrap(); // Obtém o terceiro elemento do vetor para_tripla como uma string, converte para um iterador de caracteres e pega o primeiro caractere. Este caractere representa a direção do movimento da cabeça de leitura/escrita

    Transicao { //  Cria uma instância da estrutura Transicao utilizando as informações obtidas nos passos anteriores
        estado_origem: estado_origem.to_string(),
        simbolo_lido,
        estado_destino: estado_destino.to_string(),
        simbolo_escrito,
        direcao_movimento,
    }
}

// Função para construir uma máquina de Turing a partir de um arquivo de configuração
fn construir_maquina_de_turing(arquivo_de_configuracao: String) -> MaquinaDeTuring {
    // Abre o arquivo de configuração
    let arquivo = File::open(arquivo_de_configuracao).expect("Não foi possível abrir o arquivo"); // Abre o arquivo de configuração especificado (arquivo_de_configuracao). Caso ocorra um erro ao abrir o arquivo, a função expect irá encerrar o programa e exibir uma mensagem de erro
    let leitor = BufReader::new(arquivo); // Cria um leitor de buffer (BufReader) para o arquivo aberto, o que otimiza a leitura do arquivo

    let mut contador_de_linha = 1; //  Inicializa um contador para acompanhar o número da linha que está sendo processada

    // Declarações de variáveis para armazenar informações sobre estados, alfabeto, alfabeto da fita, transições, estado inicial e estados de aceitação
    let mut estados = Vec::new();
    let mut alfabeto = Vec::new();
    let mut alfabeto_fita = Vec::new();
    let mut transicoes = Vec::new();
    let mut estado_inicial = String::new();
    let mut estados_aceitacao = Vec::new();

    // Itera sobre as linhas do arquivo
    for linha in leitor.lines() { // Itera sobre as linhas do arquivo usando o leitor de buffer
        let linha = linha.expect("Não foi possível ler a linha"); // Obtém o conteúdo da linha atual. Se houver um erro ao ler a linha, encerra o programa com uma mensagem de erro
        let linha = linha.trim(); // Remove espaços em branco no início e no final da linha
        let em_transicoes = linha.starts_with('(') && contador_de_linha != 1; // Verifica se a linha representa uma transição, ou seja, se começa com "(" e não é a primeira linha

        match contador_de_linha { // Um bloco match é usado para processar diferentes partes do arquivo de configuração com base no número da linha
            2 => estados = ler_lista_da_linha(linha), // Se a linha é a terceira do arquivo (segundo contador), processa a lista de estados e atribui o resultado à variável estados
            3 => alfabeto = ler_lista_da_linha(linha), //  Se a linha é a quarta do arquivo (terceiro contador), processa a lista do alfabeto e atribui o resultado à variável alfabeto
            4 => alfabeto_fita = ler_lista_da_linha(linha), // Se a linha é a quinta do arquivo (quarto contador), processa a lista do alfabeto da fita e atribui o resultado à variável alfabeto_fita
            _ if em_transicoes => transicoes.push(ler_transicao_da_linha(linha)), // Se a linha está em transições (iniciando com "(") e não é a primeira linha, processa a transição chamando a função ler_transicao_da_linha e adiciona a transição ao vetor transicoes.
            _ => {} // chamando a função ler_transicao_da_linha e adiciona a transição ao vetor transicoes.
        }

        if linha.starts_with('q') { // Se a linha começa com "q", ela representa o estado inicial. Remove possíveis vírgulas e armazena o estado inicial
            estado_inicial = linha.trim_matches(',').to_string(); 
        } else if linha.starts_with('{') && contador_de_linha > 4 { // Se a linha começa com "{" e o contador de linha é maior que 4, a linha representa os estados de aceitação. Chama a função ler_lista_da_linha e armazena os estados de aceitação
            estados_aceitacao = ler_lista_da_linha(linha);
        }

        contador_de_linha += 1; //  Incrementa o contador de linha para processar a próxima linha.
    }

    // Retorna a máquina de Turing construída
    MaquinaDeTuring { // Cria e retorna uma instância da estrutura MaquinaDeTuring com as informações obtidas durante o processamento do arquivo
        estados,
        alfabeto,
        alfabeto_fita,
        transicoes,
        estado_inicial,
        estados_aceitacao,
    }
}

// Função para formatar a fita da máquina de Turing para exibição
fn formatar_fita(fita: &Vec<char>, posicao_cabeca: usize, estado_atual: &String) -> String {
    fita.iter() // Cria um iterador sobre os elementos da fita, e o método enumerate é usado para obter tuplas contendo o índice (i) e o valor (&simbolo) de cada elemento
        .enumerate()
        .map(|(i, &simbolo)| { // Para cada elemento da fita, aplica a lógica definida no bloco de código entre chaves
            if i == posicao_cabeca { //  Verifica se o índice i é igual à posição da cabeça de leitura/escrita (posicao_cabeca). Se for verdadeiro, formata a string com o estado atual entre chaves e o símbolo atual  
                format!("{{{}}}{}", estado_atual, simbolo)
            } else if posicao_cabeca == fita.len() && i == fita.len() - 1 { // Verifica se a posição da cabeça de leitura/escrita é no final da fita e se o índice i é o último elemento da fita. Se for verdadeiro, formata a string com o símbolo atual seguido pelo estado atual entre chaves
                format!("{}{{{}}}", simbolo, estado_atual)
            } else {
                simbolo.to_string() // Se nenhuma das condições anteriores for atendida, simplesmente converte o símbolo atual para uma string
            }
        })
        .collect::<String>() // Coleta os resultados do mapeamento em um único valor, que é uma string que representa a fita formatada
}

// Função para inicializar a fita da máquina de Turing com a palavra de entrada
fn inicializar_fita(palavra_de_entrada: &str) -> Vec<char> {
    let mut fita: Vec<char> = vec!['B']; //  Inicializa um vetor de caracteres chamado fita com um caractere 'B' representando o símbolo branco, que é utilizado para indicar o início/começo da fita
    fita.extend(palavra_de_entrada.chars()); //  Extende o vetor fita com os caracteres da palavra de entrada. Isso é feito convertendo a palavra_de_entrada em um iterador de caracteres (chars()) e estendendo o vetor com esses caracteres
    fita.push('B'); // Adiciona mais um caractere 'B' no final da fita, indicando o final da entrada
    fita // Retorna o vetor fita inicializado.
}

// Função para escrever na saída
fn escrever_na_saida( 
    buffer_de_saida: &mut BufWriter<File>,
    fita: &Vec<char>,
    posicao_cabeca: usize,
    estado_atual: &String,
) {
    writeln!( //  Utiliza a macro writeln! para escrever uma linha formatada no arquivo de saída (buffer_de_saida). O conteúdo da linha é gerado chamando a função formatar_fita com os parâmetros fita, posicao_cabeca e estado_atual
        buffer_de_saida,
        "{}",
        formatar_fita(&fita, posicao_cabeca, &estado_atual)
    )
    .expect("Falha ao escrever no arquivo de saída"); // Utiliza o método expect para tratar potenciais erros durante a escrita no arquivo de saída. Se ocorrer um erro, a mensagem entre aspas será exibida indicando a falha
}

// Função principal para executar a máquina de Turing
fn executar_maquina_de_turing(mt: MaquinaDeTuring, palavra_de_entrada: String, arquivo_de_saida: String) {
    let mut fita = inicializar_fita(&palavra_de_entrada); //  Inicializa a fita da máquina de Turing com a palavra de entrada fornecida
    let mut estado_atual = mt.estado_inicial.clone(); //  Inicializa o estado atual da máquina de Turing com o estado inicial fornecido pela máquina de Turing
    let mut posicao_cabeca = 0; // Inicializa a posição da cabeça de leitura/escrita no início da fita

    let mut buffer_de_saida =
        BufWriter::new(File::create(arquivo_de_saida).expect("Falha ao criar o arquivo de saída")); //  Inicializa um buffer de saída para escrever no arquivo de saída. Se ocorrer um erro ao criar o arquivo, a função expect encerrará o programa com uma mensagem de erro

    escrever_na_saida(&mut buffer_de_saida, &fita, posicao_cabeca, &estado_atual); //  Escreve a configuração inicial da fita no arquivo de saída

    // Loop principal da execução da máquina de Turing
    loop { // Inicia um loop que representa a execução da máquina de Turing. O loop será encerrado quando a máquina de Turing aceitar ou rejeitar a entrada
        let simbolo_atual = fita[posicao_cabeca]; // Obtém o símbolo atual na posição da cabeça de leitura/escrita

        // Procura pela transição correspondente
        let transicao = match mt.transicoes.iter().find(|t| { // Procura pela transição correspondente na máquina de Turing com base no estado atual e no símbolo lido. Se não houver transição, a máquina rejeita a entrada
            t.estado_origem == estado_atual && t.simbolo_lido == simbolo_atual
        }) {
            Some(transicao) => transicao,
            None => {
                // Se não houver transição, rejeita a entrada
                writeln!(
                    &mut buffer_de_saida,
                    "rejeita"
                )
                .expect("Falha ao escrever no arquivo de saída");
                break;
            }
        };

        // Atualiza a fita e o estado
        fita[posicao_cabeca] = transicao.simbolo_escrito; // Atualiza o símbolo na posição da cabeça de leitura/escrita com o símbolo escrito pela transição
        estado_atual = transicao.estado_destino.clone(); // Atualiza o estado atual com o estado de destino da transição

        // Move a cabeça de leitura/escrita
        match transicao.direcao_movimento { // Move a cabeça de leitura/escrita com base na direção indicada pela transição
            'D' => posicao_cabeca += 1,
            'E' => posicao_cabeca -= 1,
            _ => panic!("Direção de movimento inválida"),
        }

        // Escreve o estado atual da fita na saída
        escrever_na_saida(&mut buffer_de_saida, &fita, posicao_cabeca, &estado_atual); // Escreve a configuração atual da fita no arquivo de saída

        // Verifica se o estado atual é um estado de aceitação
        if mt.estados_aceitacao.contains(&estado_atual) { // Verifica se o estado atual é um estado de aceitação. Se for, a máquina de Turing aceita a entrada e o loop é encerrado
            writeln!( // A função continua executando até que a máquina de Turing aceite ou rejeite a entrada, escrevendo a configuração da fita em cada passo
                &mut buffer_de_saida,
                "aceita"
            )
            .expect("Falha ao escrever no arquivo de saída");
            break;
        }
    }
}

// Função principal
fn main() {
    // Obtém os argumentos da linha de comando
    let args: Vec<String> = env::args().collect(); // Obtém os argumentos da linha de comando como uma coleção de strings (Vec<String>). O primeiro argumento (args[0]) é o nome do programa

    // Verifica se o número de argumentos é válido
    if args.len() != 4 { // Verifica se o número de argumentos é diferente de 4. Se não for, exibe uma mensagem de uso e encerra o programa com código de saída 1
        eprintln!("Uso: cargo run arquivo_de_descricao.txt palavra_de_entrada arquivo_de_saida.txt"); // Exibe uma mensagem de uso na saída de erro padrão. A função eprintln! é usada para imprimir mensagens na saída de erro
        std::process::exit(1); // Encerra o programa com código de saída 1, indicando uma terminação anormal
    }

    // Obtém os nomes dos arquivos de entrada e saída
    let arquivo_da_maquina = args[1].to_string(); //  Obtém o nome do arquivo de descrição da máquina de Turing a partir dos argumentos
    let palavra_de_entrada = args[2].to_string(); // Obtém a palavra de entrada a ser processada pela máquina de Turing a partir dos argumentos
    let arquivo_de_saida = args[3].to_string(); // Obtém o nome do arquivo de saída a partir dos argumentos

    // Constrói a máquina de Turing a partir do arquivo de configuração
    let maquina_de_turing = construir_maquina_de_turing(arquivo_da_maquina); //  Constrói a máquina de Turing a partir do arquivo de descrição usando a função construir_maquina_de_turing

    // Executa a máquina de Turing com a palavra de entrada e escreve o resultado no arquivo de saída
    executar_maquina_de_turing(maquina_de_turing, palavra_de_entrada, arquivo_de_saida); // Executa a máquina de Turing com a palavra de entrada e escreve o resultado no arquivo de saída usando a função executar_maquina_de_turing
}
