use crate::{
    model::{Amount, Currency},
    parser::{
        common::{token_com::*, Operator, Token},
        error::syntax::SyntaxError,
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

    pub fn from_vec(mut tokens: Vec<Token>) -> Result<Transaction, SyntaxError> {
        Ok(Transaction {
            agent: expect_identifier(&mut tokens, "need identifier as first argument")?,
            amount: expect_amount(&mut tokens)?,
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

#[derive(Clone)]
pub struct AtomicTransaction {
    agent: String,
    amount: i32,
    currency: Currency,
    comment: String,
}

impl AtomicTransaction {
    pub fn new(
        agent: String,
        amount: i32,
        currency: Currency,
        comment: String,
    ) -> AtomicTransaction {
        AtomicTransaction {
            agent,
            amount,
            currency,
            comment,
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
