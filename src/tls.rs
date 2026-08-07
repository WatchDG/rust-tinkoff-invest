//! TLS-конфигурация на базе Russian Trusted CA.

use tonic::transport::{Certificate, ClientTlsConfig};

/// Russian Trusted Root CA (RSA).
const RUSSIAN_TRUSTED_ROOT_CA_PEM: &[u8] =
    include_bytes!("../ca/russian_trusted_root_ca_pem.crt");

/// Russian Trusted Root CA GOST 2025.
const RUSSIAN_TRUSTED_ROOT_CA_GOST_2025_PEM: &[u8] =
    include_bytes!("../ca/russian_trusted_root_ca_gost_2025_pem.crt");

/// Russian Trusted Sub CA (RSA).
const RUSSIAN_TRUSTED_SUB_CA_PEM: &[u8] = include_bytes!("../ca/russian_trusted_sub_ca_pem.crt");

/// Russian Trusted Sub CA 2024.
const RUSSIAN_TRUSTED_SUB_CA_2024_PEM: &[u8] =
    include_bytes!("../ca/russian_trusted_sub_ca_2024_pem.crt");

/// Russian Trusted Sub CA GOST 2025.
const RUSSIAN_TRUSTED_SUB_CA_GOST_2025_PEM: &[u8] =
    include_bytes!("../ca/russian_trusted_sub_ca_gost_2025_pem.crt");

/// Встроенные Russian Trusted CA (root + sub).
pub fn russian_trusted_ca_certificates() -> Vec<Certificate> {
    vec![
        Certificate::from_pem(RUSSIAN_TRUSTED_ROOT_CA_PEM),
        Certificate::from_pem(RUSSIAN_TRUSTED_ROOT_CA_GOST_2025_PEM),
        Certificate::from_pem(RUSSIAN_TRUSTED_SUB_CA_PEM),
        Certificate::from_pem(RUSSIAN_TRUSTED_SUB_CA_2024_PEM),
        Certificate::from_pem(RUSSIAN_TRUSTED_SUB_CA_GOST_2025_PEM),
    ]
}

/// TLS-конфиг **только** с Russian Trusted CA (без системных корней).
pub fn russian_trusted_tls_config() -> ClientTlsConfig {
    ClientTlsConfig::new().ca_certificates(russian_trusted_ca_certificates())
}
