// Este programa lê uma descrição de uma máquina de Turing a partir de um arquivo de
// configuração, inicializa uma fita com uma palavra de entrada, executa a máquina de 
// Turing e escreve o resultado em um arquivo de saída. O código está organizado em 
// funções para facilitar a compreensão e manutenção.

#![allow(dead_code)] // Permite códigos não utilizados sem emitir um aviso

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

// Estrutura que representa uma transição da máquina de Turing
struct Transition {
    from_state: String,
    read_symbol: char,
    to_state: String,
    write_symbol: char,
    move_direction: char,
}

// Estrutura que representa uma máquina de Turing
struct TuringMachine {
    states: Vec<String>,
    alphabet: Vec<String>,
    tape_alphabet: Vec<String>,
    transitions: Vec<Transition>,
    initial_state: String,
    accept_states: Vec<String>,
}

// Função auxiliar para ler uma lista de uma linha
fn read_list_from_line(line: &str) -> Vec<String> {
    line.trim_matches(|c| char::is_ascii_punctuation(&c))
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

// Função auxiliar para ler uma transição de uma linha
fn read_transition_from_line(line: &str) -> Transition {
    let parts: Vec<&str> = line.split("->").collect();
    let from_state_symbol: Vec<String> = read_list_from_line(parts[0]);
    let to_triple: Vec<String> = read_list_from_line(parts[1]);

    let from_state = &from_state_symbol[0];
    let read_symbol = from_state_symbol[1].chars().next().unwrap();

    let to_state = &to_triple[0];
    let write_symbol = to_triple[1].chars().next().unwrap();
    let move_direction = to_triple[2].chars().next().unwrap();

    Transition {
        from_state: from_state.to_string(),
        read_symbol,
        to_state: to_state.to_string(),
        write_symbol,
        move_direction,
    }
}

// Função para construir uma máquina de Turing a partir de um arquivo de configuração
fn build_turing_machine(config_file: String) -> TuringMachine {
    // Abre o arquivo de configuração
    let file = File::open(config_file).expect("Unable to open the file");
    let reader = BufReader::new(file);

    let mut line_counter = 1;

    let mut states = Vec::new();
    let mut alphabet = Vec::new();
    let mut tape_alphabet = Vec::new();
    let mut transitions = Vec::new();
    let mut initial_state = String::new();
    let mut accept_states = Vec::new();

    // Itera sobre as linhas do arquivo
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let line = line.trim();
        let in_transitions = line.starts_with('(') && line_counter != 1;

        match line_counter {
            2 => states = read_list_from_line(line),
            3 => alphabet = read_list_from_line(line),
            4 => tape_alphabet = read_list_from_line(line),
            _ if in_transitions => transitions.push(read_transition_from_line(line)),
            _ => {}
        }

        if line.starts_with('q') {
            initial_state = line.trim_matches(',').to_string();
        } else if line.starts_with('{') && line_counter > 4 {
            accept_states = read_list_from_line(line);
        }

        line_counter += 1;
    }

    // Retorna a máquina de Turing construída
    TuringMachine {
        states,
        alphabet,
        tape_alphabet,
        transitions,
        initial_state,
        accept_states,
    }
}

// Função para formatar a fita da máquina de Turing para exibição
fn format_tape(tape: &Vec<char>, head_position: usize, current_state: &String) -> String {
    tape.iter()
        .enumerate()
        .map(|(i, &symbol)| {
            if i == head_position {
                format!("{{{}}}{}", current_state, symbol)
            } else if head_position == tape.len() && i == tape.len() - 1 {
                format!("{}{{{}}}", symbol, current_state)
            } else {
                symbol.to_string()
            }
        })
        .collect::<String>()
}

// Função para inicializar a fita da máquina de Turing com a palavra de entrada
fn initialize_tape(input_word: &str) -> Vec<char> {
    let mut tape: Vec<char> = vec!['B'];
    tape.extend(input_word.chars());
    tape.push('B');
    tape
}

// Função para escrever na saída
fn write_to_output(
    output_buffer: &mut BufWriter<File>,
    tape: &Vec<char>,
    head_position: usize,
    current_state: &String,
) {
    writeln!(
        output_buffer,
        "{}",
        format_tape(&tape, head_position, &current_state)
    )
    .expect("Failed to write to output file");
}

// Função principal para executar a máquina de Turing
fn run_turing_machine(tm: TuringMachine, input_word: String, output_file: String) {
    let mut tape = initialize_tape(&input_word);
    let mut current_state = tm.initial_state.clone();
    let mut head_position = 0;

    let mut output_buffer =
        BufWriter::new(File::create(output_file).expect("Failed to create output file"));

    write_to_output(&mut output_buffer, &tape, head_position, &current_state);

    // Loop principal da execução da máquina de Turing
    loop {
        let current_symbol = tape[head_position];

        // Procura pela transição correspondente
        let transition = match tm.transitions.iter().find(|t| {
            t.from_state == current_state && t.read_symbol == current_symbol
        }) {
            Some(transition) => transition,
            None => {
                // Se não houver transição, rejeita a entrada
                writeln!(
                    &mut output_buffer,
                    "rejeita"
                )
                .expect("Failed to write to output file");
                break;
            }
        };

        // Atualiza a fita e o estado
        tape[head_position] = transition.write_symbol;
        current_state = transition.to_state.clone();

        // Move a cabeça de leitura/escrita
        match transition.move_direction {
            'D' => head_position += 1,
            'E' => head_position -= 1,
            _ => panic!("Invalid move direction"),
        }

        // Escreve o estado atual da fita na saída
        write_to_output(&mut output_buffer, &tape, head_position, &current_state);

        // Verifica se o estado atual é um estado de aceitação
        if tm.accept_states.contains(&current_state) {
            writeln!(
                &mut output_buffer,
                "aceita"
            )
            .expect("Failed to write to output file");
            break;
        }
    }
}

// Função principal
fn main() {
    // Obtém os argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifica se o número de argumentos é válido
    if args.len() != 4 {
        eprintln!("Usage: cargo run description_file.txt input_word output_file.txt");
        std::process::exit(1);
    }

    // Obtém os nomes dos arquivos de entrada e saída
    let machine_file = args[1].to_string();
    let input_word = args[2].to_string();
    let output_file = args[3].to_string();

    // Constrói a máquina de Turing a partir do arquivo de configuração
    let turing_machine = build_turing_machine(machine_file);

    // Executa a máquina de Turing com a palavra de entrada e escreve o resultado no arquivo de saída
    run_turing_machine(turing_machine, input_word, output_file);
}
