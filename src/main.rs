mod equacao_diofantina;

use std::io::stdin;
use equacao_diofantina::EquacaoDiofantina;

fn main() {
    /* Pedindo os valores de A, B e C */
    let a: usize = match pedir_valor("A") { Ok(valor) => valor, Err(erro) => { println!("{}", erro); return ; } };
    let b: usize = match pedir_valor("B") { Ok(valor) => valor, Err(erro) => { println!("{}", erro); return ; } };
    let c: usize = match pedir_valor("C") { Ok(valor) => valor, Err(erro) => { println!("{}", erro); return ; } };
    
    /* Pergunta ao usuário se deseja apenas soluções positivas */
    let somente_solucoes_positivas = match perguntar_sobre_solucoes_negativas() {
        Ok(valor) => valor, 
        Err(erro) => { println!("{}", erro); return ; }
    };

    /* Montando equação e obtendo resposta */
    let equacao  = EquacaoDiofantina{a, b, c};
    let resposta = match equacao.resolver(somente_solucoes_positivas) {
        Ok(valor) => valor,
        Err(erro) => { println!("{}", erro); return ; }
    };
    
    /* Exibindo resposta ao usuário */
    println!("{}", resposta);
}

/*
 * Pede pelo input de valor numérico para uma variável.
 * Devolve uma mensagem de erro caso valor digitado seja inválido.
 */
fn pedir_valor(nome_variavel: &str) -> Result<usize, &str> {
    let mut x = String::new();
    println!("Informe o valor de {}: ", nome_variavel);
    stdin().read_line(&mut x).expect("Erro de IO");
    match x.trim().parse() {
        Err(_) => Err("Por favor, informe apenas valores inteiros positivos."),
        Ok(num) => Ok(num)
    }
}

/*
 * Pergunta ao usuário se deseja apenas soluções positivas.
 * Devolve uma mensagem de erro caso valor digitado seja inválido.
 * Se for válido, devolve booleano indicando se deseja somente soluções positivas.
 */
fn perguntar_sobre_solucoes_negativas() -> Result<bool, &'static str> {
    let mut x = String::new();
    println!("Sua resposta pode conter valores negativos? (S/N)");
    stdin().read_line(&mut x).expect("Erro de IO");
    if x.trim() == "s" || x.trim() == "S" {
        return Ok(false);
    } else if x.trim() == "n" || x.trim() == "N" {
        return Ok(true);
    }
    Err("Por favor, responda esta pergunta com S ou N.")
}