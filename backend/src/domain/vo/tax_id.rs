use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use regex::Regex;
use std::sync::Arc;

const TAX_ID_BRAZIL_CPF_LENGTH: usize = 11;

pub trait Document {
    fn validate(&self) -> ResultApp<()>;
    fn format(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum TaxId {
    BrazilCpf(String),
    BrazilCnpj(String),
}

impl TaxId {
    pub fn new(value: String) -> Self {
        if value.len() == TAX_ID_BRAZIL_CPF_LENGTH {
            TaxId::BrazilCpf(value)
        } else {
            TaxId::BrazilCnpj(value)
        }
    }

    pub fn format(&self) -> ResultApp<String> {
        match &self {
            TaxId::BrazilCpf(s) => {
                let re = Regex::new(r"^(\d{3})(\d{3})(\d{3})(\d{2})$").unwrap();
                let cpf = re.replace_all(s, "$1.$2.$3-$4");
                Ok(cpf.to_string())
            }
            TaxId::BrazilCnpj(s) => {
                let re = Regex::new(r"^(\d{2})(\d{3})(\d{3})(\d{4})(\d{2})$").unwrap();
                let cnpj = re.replace_all(s, "$1.$2.$3/$4-$5");
                Ok(cnpj.to_string())
            }
        }
    }
}

impl Document for TaxId {
    fn format(&self) -> String {
        match &self {
            TaxId::BrazilCpf(cpf) => cpf.to_string(),
            TaxId::BrazilCnpj(cnpj) => cnpj.to_string(),
        }
    }

    fn validate(&self) -> ResultApp<()> {
        match &self {
            TaxId::BrazilCpf(cpf) => {
                let re = Regex::new(r"^(\d{3})(\d{3})(\d{3})(\d{2})$").unwrap();
                if re.is_match(cpf) {
                    return Ok(());
                }
                Err(Arc::new(AppError::Validation(ErrorData::new(
                    "invalid-brazilian-cpf",
                    "Invalid Brazilian CPF",
                ))))
            }
            TaxId::BrazilCnpj(cnpj) => {
                let re = Regex::new(r"^(\d{2})(\d{3})(\d{3})(\d{4})(\d{2})$").unwrap();
                if re.is_match(cnpj) {
                    return Ok(());
                }
                Err(Arc::new(AppError::Validation(ErrorData::new(
                    "invalid-brazilian-cnpj",
                    "Invalid Brazilian CNPJ",
                ))))
            }
        }
    }
}
