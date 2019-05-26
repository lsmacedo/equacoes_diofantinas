mod equacao_diofantina;
mod testes;

use std::io::stdin;
use equacao_diofantina::EquacaoDiofantina;
use testes::rodar_testes;

fn main() {
    /* Pedindo os valores de A, B e C */
    let a: i32 = match pedir_valor("A") { Ok(valor) => valor, Err(erro) => { println!("{}", erro); return ; } };
    let b: i32 = match pedir_valor("B") { Ok(valor) => valor, Err(erro) => { println!("{}", erro); return ; } };
    let c: i32 = match pedir_valor("C") { Ok(valor) => valor, Err(erro) => { println!("{}", erro); return ; } };
    
    /* Pergunta ao usuário se deseja apenas soluções positivas */
    let somente_solucoes_positivas = match perguntar_sobre_solucoes_positivas() {
        Ok(valor) => valor, 
        Err(erro) => { println!("{}", erro); return ; }
    };

    /* Montando equação e obtendo solução */
    let equacao  = EquacaoDiofantina{a, b, c};
    let solucao  = match equacao.resolver(somente_solucoes_positivas) {
        Ok(valor) => valor,
        Err(erro) => { println!("{}", erro); return ; }
    };
    
    /* Exibindo solução ao usuário */
    println!("{}", solucao);
}

/*
 * Pede pelo input de valor numérico para uma variável.
 * Devolve uma mensagem de erro caso valor digitado seja inválido.
 */
fn pedir_valor(nome_variavel: &str) -> Result<i32, &str> {
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
fn perguntar_sobre_solucoes_positivas() -> Result<bool, &'static str> {
    let mut x = String::new();
    println!("Sua solução deve conter somente valores positivos? (S/N)");
    stdin().read_line(&mut x).expect("Erro de IO");
    if x.trim() == "s" || x.trim() == "S" {
        return Ok(true);
    } else if x.trim() == "n" || x.trim() == "N" {
        return Ok(false);
    }
    Err("Por favor, responda esta pergunta com S ou N.")
}