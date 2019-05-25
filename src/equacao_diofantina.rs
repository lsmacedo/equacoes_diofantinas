use std::fmt;

pub struct Resposta<'a> {
    pub x: i32,
    pub y: i32,
    eq: &'a EquacaoDiofantina
}

impl<'a> fmt::Display for Resposta<'a> {

    /*
     * Permitindo imprimir resposta em formato AxX + BxY = C
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{} + {}x{} = {}", self.eq.a, self.x, self.eq.b, self.y, self.eq.c)
    }
    
}

#[derive(Debug)]
pub struct EquacaoDiofantina {
    pub a: usize,
    pub b: usize,
    pub c: usize
}

impl EquacaoDiofantina {
    
    /*
     * Encontra a solução de uma equação diofantina
     * Para isto, utiliza a seguinte lógica:
     * Multiplica A por diversos valores de x iniciando em 0 ou -A.
     * Caso o resto da divisão C - Ax por B for 0, ela será o valor de y.
     * Devolve uma mensagem de erro caso não seja encontrada nenhuma solução.
     * 
     */
    pub fn resolver(&self, somente_solucoes_positivas: bool) -> Result<Resposta, &str> {
        let a: i32 = self.a as i32;
        let b: i32 = self.b as i32;
        let c: i32 = self.c as i32;
        let mut _x: i32 = 0;
        
        if !somente_solucoes_positivas {
            _x = a * -1;
        }
        
        while _x * a <= c {
            let temp = c - (a * _x);
            if temp % b == 0 {
                let x:i32 = _x;
                let y:i32 = temp / b;
                return Ok(Resposta{x, y, eq: &self});
            }
            _x += 1;
        }
        
        return Err("Nenhuma solução encontrada");
    }
      
}