use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_Expr<TOKENS: IntoIterator<Item=Tok>>(
    tokens: TOKENS)
    -> Result<(Option<Tok>, i32), Option<Tok>>
{
    let mut tokens = tokens.into_iter();
    let lookahead = tokens.next();
    match try!(__parse__Expr::__state0(lookahead, &mut tokens)) {
        (lookahead, __parse__Expr::__Nonterminal::__Expr(nt)) => Ok((lookahead, nt)),
        _ => unreachable!(),
    }
}

mod __parse__Expr {
    #![allow(non_snake_case, unused_mut, unused_variables)]

    use util::tok::Tok;

    pub enum __Nonterminal {
        __Expr(i32),
        Term(i32),
        Expr(i32),
        Factor(i32),
    }

    // State 0
    //   Expr = (*) Expr "+" Factor [EOF]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [EOF]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [EOF]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "(" -> Shift(S5)
    //   "Num" -> Shift(S3)
    //
    //   Term -> S1
    //   Expr -> S4
    //   Factor -> S2
    pub fn __state0<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym0 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym0));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym0 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state3(lookahead, tokens, sym0));
            }
            _ => {
                return Err(lookahead);
            }
        }
        loop {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state1(lookahead, tokens, sym0));
                }
                __Nonterminal::Expr(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state4(lookahead, tokens, sym0));
                }
                __Nonterminal::Factor(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state2(lookahead, tokens, sym0));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
    }

    // State 1
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //   EOF -> Reduce(Factor = Term => ActionFn(6);)
    //
    pub fn __state1<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 2
    //   Expr = Factor (*) [EOF]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S7)
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "/" -> Shift(S6)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //
    pub fn __state2<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Times(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state7(lookahead, tokens, sym0, sym1));
            }
            Some(tok @ Tok::Div(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state6(lookahead, tokens, sym0, sym1));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 3
    //   Term = "Num" (*) [EOF]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "Num" => ActionFn(7);)
    //   EOF -> Reduce(Term = "Num" => ActionFn(7);)
    //   "-" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(7);)
    //
    pub fn __state3<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 4
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   "+" -> Shift(S8)
    //   EOF -> Reduce(__Expr = Expr => ActionFn(0);)
    //   "-" -> Shift(S9)
    //
    pub fn __state4<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Plus(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state8(lookahead, tokens, sym0, sym1));
            }
            Some(tok @ Tok::Minus(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state9(lookahead, tokens, sym0, sym1));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action0(sym0);
                return Ok((lookahead, __Nonterminal::__Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 5
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [EOF]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S14)
    //   "(" -> Shift(S13)
    //
    //   Factor -> S10
    //   Expr -> S12
    //   Term -> S11
    pub fn __state5<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state14(lookahead, tokens, sym1));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Factor(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state10(lookahead, tokens, sym1));
                }
                __Nonterminal::Expr(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state12(lookahead, tokens, sym0, sym1));
                }
                __Nonterminal::Term(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state11(lookahead, tokens, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 6
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S3)
    //   "(" -> Shift(S5)
    //
    //   Term -> S15
    pub fn __state6<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state3(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state15(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 7
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S5)
    //   "Num" -> Shift(S3)
    //
    //   Term -> S16
    pub fn __state7<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state3(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state16(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 8
    //   Expr = Expr "+" (*) Factor [EOF]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S3)
    //   "(" -> Shift(S5)
    //
    //   Factor -> S17
    //   Term -> S1
    pub fn __state8<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state3(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state17(lookahead, tokens, sym0, sym1, sym2));
                }
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state1(lookahead, tokens, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 9
    //   Expr = Expr "-" (*) Factor [EOF]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S5)
    //   "Num" -> Shift(S3)
    //
    //   Term -> S1
    //   Factor -> S18
    pub fn __state9<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state3(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state1(lookahead, tokens, sym2));
                }
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state18(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 10
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "/" -> Shift(S19)
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   ")" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S20)
    //
    pub fn __state10<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Div(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state19(lookahead, tokens, sym0, sym1));
            }
            Some(tok @ Tok::Times(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state20(lookahead, tokens, sym0, sym1));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 11
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
    //
    pub fn __state11<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 12
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S22)
    //   "-" -> Shift(S23)
    //   "+" -> Shift(S21)
    //
    pub fn __state12<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::RParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state22(lookahead, tokens, sym0, sym1, sym2));
            }
            Some(tok @ Tok::Minus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state23(lookahead, tokens, sym1, sym2));
            }
            Some(tok @ Tok::Plus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state21(lookahead, tokens, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 13
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S13)
    //   "Num" -> Shift(S14)
    //
    //   Term -> S11
    //   Factor -> S10
    //   Expr -> S24
    pub fn __state13<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym1));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state14(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state11(lookahead, tokens, sym1));
                }
                __Nonterminal::Factor(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state10(lookahead, tokens, sym1));
                }
                __Nonterminal::Expr(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state24(lookahead, tokens, sym0, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 14
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "-" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "+" -> Reduce(Term = "Num" => ActionFn(7);)
    //
    pub fn __state14<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 15
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state15<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 16
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   EOF -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state16<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 17
    //   Expr = Expr "+" Factor (*) [EOF]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S6)
    //   "*" -> Shift(S7)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   EOF -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //
    pub fn __state17<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state6(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state7(lookahead, tokens, sym2, sym3));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 18
    //   Expr = Expr "-" Factor (*) [EOF]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S6)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   EOF -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "*" -> Shift(S7)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //
    pub fn __state18<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state6(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state7(lookahead, tokens, sym2, sym3));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 19
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S14)
    //   "(" -> Shift(S13)
    //
    //   Term -> S25
    pub fn __state19<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state14(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state25(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 20
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S13)
    //   "Num" -> Shift(S14)
    //
    //   Term -> S26
    pub fn __state20<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state14(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state26(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 21
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S14)
    //   "(" -> Shift(S13)
    //
    //   Term -> S11
    //   Factor -> S27
    pub fn __state21<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state14(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state11(lookahead, tokens, sym2));
                }
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state27(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 22
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   EOF -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //
    pub fn __state22<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 23
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S13)
    //   "Num" -> Shift(S14)
    //
    //   Term -> S11
    //   Factor -> S28
    pub fn __state23<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state14(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state11(lookahead, tokens, sym2));
                }
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state28(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 24
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "-" -> Shift(S23)
    //   "+" -> Shift(S21)
    //   ")" -> Shift(S29)
    //
    pub fn __state24<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Minus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state23(lookahead, tokens, sym1, sym2));
            }
            Some(tok @ Tok::Plus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state21(lookahead, tokens, sym1, sym2));
            }
            Some(tok @ Tok::RParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state29(lookahead, tokens, sym0, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 25
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state25<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 26
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state26<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 27
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S19)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "*" -> Shift(S20)
    //
    pub fn __state27<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state19(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state20(lookahead, tokens, sym2, sym3));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 28
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S19)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "*" -> Shift(S20)
    //
    pub fn __state28<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state19(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state20(lookahead, tokens, sym2, sym3));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 29
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   ")" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //
    pub fn __state29<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }
}
fn __action0(
    __0: i32,
) -> i32 {
    (__0)
}
fn __action1(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l - r
}
fn __action2(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l + r
}
fn __action3(
    __0: i32,
) -> i32 {
    (__0)
}
fn __action4(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l * r
}
fn __action5(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l / r
}
fn __action6(
    __0: i32,
) -> i32 {
    (__0)
}
fn __action7(
    n: Tok,
) -> i32 {
    n.as_num()
}
fn __action8(
    _: Tok,
    __0: i32,
    _: Tok,
) -> i32 {
    (__0)
}