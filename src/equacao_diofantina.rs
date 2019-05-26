use std::fmt;
use std::cmp::min;

pub struct Solucao<'a> {
    pub x: i32,
    pub y: i32,
    eq: &'a EquacaoDiofantina
}

impl<'a> fmt::Display for Solucao<'a> {

    /*
     * Permitindo imprimir Solucao em formato A.x + B.y = C
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x >= 0 && self.y >= 0 {
            write!(f, "\nx = {}\ny = {}\n{}.{} + {}.{} = {}", self.x, self.y, self.eq.a, self.x, self.eq.b, self.y, self.eq.c)
        } else if self.x >= 0 && self.y < 0{
            write!(f, "\nx = {}\ny = {}\n{}.{} + {}.({}) = {}", self.x, self.y, self.eq.a, self.x, self.eq.b, self.y, self.eq.c)
        } else if self.x < 0 && self.y >= 0{
            write!(f, "\nx = {}\ny = {}\n{}.({}) + {}.{} = {}", self.x, self.y, self.eq.a, self.x, self.eq.b, self.y, self.eq.c)
        } else {
            write!(f, "\nx = {}\ny = {}\n{}.({}) + {}.({}) = {}", self.x, self.y, self.eq.a, self.x, self.eq.b, self.y, self.eq.c)
        }
    }
    
}

#[derive(Debug)]
pub struct EquacaoDiofantina {
    pub a: i32,
    pub b: i32,
    pub c: i32
}

impl EquacaoDiofantina {
    
    /*
     * Encontra a solução de uma equação diofantina
     * Para isto, utiliza a seguinte lógica:
     * Multiplica A por diversos valores de x iniciando em 0, 1, -|A| ou -|B|.
     * Caso C - Ax seja divisível por B, este será o valor de y.
     * Devolve uma mensagem de erro caso não seja encontrada nenhuma solução.
     */
    pub fn resolver(&self, somente_solucoes_positivas: bool) -> Result<Solucao, &str> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let mut x: i32;
        
        /* Definindo valor inicial de x */
        if !somente_solucoes_positivas {
            if a > 0 {
                x = min(-1 * a.abs(), -1 * b.abs());
            } else {
                x = 0
            }
        } else {
            x = 1;
        }
        
        /* Iterando por todos os valores possíveis de x e verificando se 
           algum satisfaz a equação */
        while x * a <= c.abs() {
            let temp = c - (a * x);
            if temp % b == 0 && (temp / b > 0 || !somente_solucoes_positivas) {
                let y:i32 = temp / b;
                return Ok(Solucao{x, y, eq: &self});
            }
            x += 1;
        }
        
        return Err("Nenhuma solução encontrada");
    }
      
}