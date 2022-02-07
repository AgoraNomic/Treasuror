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

impl Parseable for Transaction {
    fn from_tokens(tokens: &mut Vec<Token>) -> SyntaxResult<Transaction> {
        Ok(Transaction {
            agent: take_identifier(tokens, "need identifier as first argument")?,
            amount: take(tokens)?,
            operator: take_operator(tokens, "need operator in transaction")?,
            comment: take_stringlike(tokens, "").unwrap_or_else(|_| String::new()),
        })
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

    pub fn patient(&self) -> &str {
        &self.patient
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }
}

impl Parseable for Trade {
    fn from_tokens(tokens: &mut Vec<Token>) -> SyntaxResult<Trade> {
        Ok(Trade {
            amount: take(tokens)?,
            patient: take_identifier(tokens, "need identifier as second argument to trade")?,
        })
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
