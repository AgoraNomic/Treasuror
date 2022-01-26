use crate::{
    model::{Amount, Currency},
    parser::{
        common::{token_com::*, Operator, Parseable, Token},
        error::syntax::SyntaxResult,
    },
};

#[derive(Clone)]
pub struct Transaction {
    agent: String,
    amount: Amount,
    operator: Operator,
    comment: String,
}

impl Transaction {
    pub fn new(agent: String, amount: Amount, operator: Operator, comment: String) -> Transaction {
        Transaction {
            agent,
            amount,
            operator,
            comment,
        }
    }

    pub fn from_vec(mut tokens: Vec<Token>) -> SyntaxResult<Transaction> {
        Ok(Transaction {
            agent: expect_identifier(&mut tokens, "need identifier as first argument")?,
            amount: expect(&mut tokens)?,
            operator: expect_operator(&mut tokens, "need operator in transaction")?,
            comment: expect_stringlike(&mut tokens, "").unwrap_or_else(|_| String::new()),
        })
    }

    pub fn agent(&self) -> &str {
        &self.agent
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Trade {
    patient: String,
    amount: Amount,
}

impl Trade {
    pub fn new<S: Into<String>>(amount: Amount, patient: S) -> Trade {
        Trade {
            patient: patient.into(),
            amount,
        }
    }

    pub fn from_vec(tokens: &mut Vec<Token>) -> SyntaxResult<Trade> {
        Ok(Trade {
            amount: expect(tokens)?,
            patient: expect_identifier(tokens, "need identifier as second argument to trade")?,
        })
    }

    pub fn patient(&self) -> &str {
        &self.patient
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }
}

#[derive(Clone)]
pub struct AtomicTransaction {
    agent: String,
    amount: i32,
    currency: Currency,
    comment: String,
}

impl AtomicTransaction {
    pub fn new<S: Into<String>>(
        agent: S,
        amount: i32,
        currency: Currency,
        comment: S,
    ) -> AtomicTransaction {
        AtomicTransaction {
            agent: agent.into(),
            amount,
            currency,
            comment: comment.into(),
        }
    }

    pub fn transfer_vec(
        agent: &str,
        patient: &str,
        amount: i32,
        currency: Currency,
        comment: &str,
    ) -> Vec<AtomicTransaction> {
        vec![
            AtomicTransaction::new(
                agent.to_string(),
                -amount,
                currency,
                format!(
                    "Transfer {}{}",
                    patient.replace("_", " "),
                    if comment.is_empty() {
                        String::from("")
                    } else {
                        String::from(": ") + comment
                    }
                ),
            ),
            AtomicTransaction::new(
                patient.to_string(),
                amount,
                currency,
                format!(
                    "Transfer {}{}",
                    agent.replace("_", " "),
                    if comment.is_empty() {
                        String::from("")
                    } else {
                        String::from(": ") + comment
                    }
                ),
            ),
        ]
    }

    pub fn trade_vec(
        agent: &str,
        patient: &str,
        amount1: i32,
        currency1: Currency,
        amount2: i32,
        currency2: Currency,
        comment: &str,
    ) -> Vec<AtomicTransaction> {
        let mut one = AtomicTransaction::transfer_vec(agent, patient, amount1, currency1, comment);
        let mut two = AtomicTransaction::transfer_vec(patient, agent, amount2, currency2, comment);
        one.append(&mut two);
        one
    }

    pub fn agent(&self) -> &str {
        &self.agent
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}
