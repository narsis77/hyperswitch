use std::str::FromStr;

pub use common_enums::*;
#[cfg(feature = "dummy_connector")]
use common_utils::errors;
use utoipa::ToSchema;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
)]

/// The routing algorithm to be used to process the incoming request from merchant to outgoing payment processor or payment method. The default is 'Custom'
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RoutingAlgorithm {
    RoundRobin,
    MaxConversion,
    MinCost,
    Custom,
}

/// A connector is an integration to fulfill payments
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    ToSchema,
    serde::Deserialize,
    serde::Serialize,
    strum::VariantNames,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
    Hash,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Connector {
    // Novalnet,
    // Nexixpay,
    Adyenplatform,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "phonypay")]
    #[strum(serialize = "phonypay")]
    DummyConnector1,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "fauxpay")]
    #[strum(serialize = "fauxpay")]
    DummyConnector2,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "pretendpay")]
    #[strum(serialize = "pretendpay")]
    DummyConnector3,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "stripe_test")]
    #[strum(serialize = "stripe_test")]
    DummyConnector4,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "adyen_test")]
    #[strum(serialize = "adyen_test")]
    DummyConnector5,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "checkout_test")]
    #[strum(serialize = "checkout_test")]
    DummyConnector6,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "paypal_test")]
    #[strum(serialize = "paypal_test")]
    DummyConnector7,
    Aci,
    Adyen,
    Airwallex,
    Authorizedotnet,
    Bambora,
    Bamboraapac,
    Bankofamerica,
    Billwerk,
    Bitpay,
    Bluesnap,
    Boku,
    Braintree,
    Cashtocode,
    Checkout,
    Coinbase,
    Cryptopay,
    Cybersource,
    Datatrans,
    // Deutschebank,
    Dlocal,
    Ebanx,
    Fiserv,
    Fiservemea,
    // Fiuu,
    Forte,
    Globalpay,
    Globepay,
    Gocardless,
    Gpayments,
    Helcim,
    Iatapay,
    Itaubank,
    Klarna,
    Mifinity,
    Mollie,
    Multisafepay,
    Netcetera,
    Nexinets,
    Nmi,
    Noon,
    Nuvei,
    // Opayo, added as template code for future usage
    Opennode,
    Paybox,
    // Payeezy, As psync and rsync are not supported by this connector, it is added as template code for future usage
    Payme,
    Payone,
    Paypal,
    Payu,
    Placetopay,
    Powertranz,
    Prophetpay,
    Rapyd,
    Razorpay,
    Shift4,
    Square,
    Stax,
    Stripe,
    // Taxjar,
    Threedsecureio,
    Trustpay,
    Tsys,
    Volt,
    Wellsfargo,
    // Wellsfargopayout,
    Wise,
    Worldline,
    Worldpay,
    Signifyd,
    Plaid,
    Riskified,
    Zen,
    Zsl,
}

impl Connector {
    #[cfg(feature = "payouts")]
    pub fn supports_instant_payout(&self, payout_method: Option<PayoutType>) -> bool {
        matches!(
            (self, payout_method),
            (Self::Paypal, Some(PayoutType::Wallet))
                | (_, Some(PayoutType::Card))
                | (Self::Adyenplatform, _)
        )
    }
    #[cfg(feature = "payouts")]
    pub fn supports_create_recipient(&self, payout_method: Option<PayoutType>) -> bool {
        matches!((self, payout_method), (_, Some(PayoutType::Bank)))
    }
    #[cfg(feature = "payouts")]
    pub fn supports_payout_eligibility(&self, payout_method: Option<PayoutType>) -> bool {
        matches!((self, payout_method), (_, Some(PayoutType::Card)))
    }
    #[cfg(feature = "payouts")]
    pub fn is_payout_quote_call_required(&self) -> bool {
        matches!(self, Self::Wise)
    }
    #[cfg(feature = "payouts")]
    pub fn supports_access_token_for_payout(&self, payout_method: Option<PayoutType>) -> bool {
        matches!((self, payout_method), (Self::Paypal, _))
    }
    #[cfg(feature = "payouts")]
    pub fn supports_vendor_disburse_account_create_for_payout(&self) -> bool {
        matches!(self, Self::Stripe)
    }
    pub fn supports_access_token(&self, payment_method: PaymentMethod) -> bool {
        matches!(
            (self, payment_method),
            (Self::Airwallex, _)
                | (Self::Globalpay, _)
                | (Self::Paypal, _)
                | (Self::Payu, _)
                | (Self::Trustpay, PaymentMethod::BankRedirect)
                | (Self::Iatapay, _)
                | (Self::Volt, _)
                | (Self::Itaubank, _)
        )
    }
    pub fn supports_file_storage_module(&self) -> bool {
        matches!(self, Self::Stripe | Self::Checkout)
    }
    pub fn requires_defend_dispute(&self) -> bool {
        matches!(self, Self::Checkout)
    }
    pub fn is_separate_authentication_supported(&self) -> bool {
        match self {
            #[cfg(feature = "dummy_connector")]
            Self::DummyConnector1
            | Self::DummyConnector2
            | Self::DummyConnector3
            | Self::DummyConnector4
            | Self::DummyConnector5
            | Self::DummyConnector6
            | Self::DummyConnector7 => false,
            Self::Aci
            // Add Separate authentication support for connectors
			// | Self::Novalnet
			// | Self::Nexixpay
			// | Self::Fiuu
			// | Self::Taxjar
            | Self::Adyen
            | Self::Adyenplatform
            | Self::Airwallex
            | Self::Authorizedotnet
            | Self::Bambora
            | Self::Bamboraapac
            | Self::Bankofamerica
            | Self::Billwerk
            | Self::Bitpay
            | Self::Bluesnap
            | Self::Boku
            | Self::Braintree
            | Self::Cashtocode
            | Self::Coinbase
            | Self::Cryptopay
			// | Self::Deutschebank
            | Self::Dlocal
            | Self::Ebanx
            | Self::Fiserv
			| Self::Fiservemea
            | Self::Forte
            | Self::Globalpay
            | Self::Globepay
            | Self::Gocardless
            | Self::Gpayments
            | Self::Helcim
            | Self::Iatapay
            | Self::Itaubank
            | Self::Klarna
            | Self::Mifinity
            | Self::Mollie
            | Self::Multisafepay
            | Self::Nexinets
            | Self::Nuvei
            | Self::Opennode
			| Self::Paybox
			| Self::Payme
            | Self::Payone
            | Self::Paypal
            | Self::Payu
            | Self::Placetopay
            | Self::Powertranz
            | Self::Prophetpay
            | Self::Rapyd
            | Self::Shift4
            | Self::Square
            | Self::Stax
            | Self::Trustpay
            | Self::Tsys
            | Self::Volt
            | Self::Wellsfargo
			// | Self::Wellsfargopayout
            | Self::Wise
            | Self::Worldline
            | Self::Worldpay
            | Self::Zen
            | Self::Zsl
            | Self::Signifyd
            | Self::Plaid
            | Self::Razorpay
            | Self::Riskified
            | Self::Threedsecureio
            | Self::Datatrans
            | Self::Netcetera
            | Self::Noon
            | Self::Stripe => false,
            Self::Checkout | Self::Nmi | Self::Cybersource => true,
        }
    }
    pub fn is_pre_processing_required_before_authorize(&self) -> bool {
        matches!(self, Self::Airwallex)
    }
    #[cfg(feature = "dummy_connector")]
    pub fn validate_dummy_connector_enabled(
        &self,
        is_dummy_connector_enabled: bool,
    ) -> errors::CustomResult<(), errors::ValidationError> {
        if !is_dummy_connector_enabled
            && matches!(
                self,
                Self::DummyConnector1
                    | Self::DummyConnector2
                    | Self::DummyConnector3
                    | Self::DummyConnector4
                    | Self::DummyConnector5
                    | Self::DummyConnector6
                    | Self::DummyConnector7
            )
        {
            Err(errors::ValidationError::InvalidValue {
                message: "Invalid connector name".to_string(),
            }
            .into())
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "payouts")]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PayoutConnectors {
    Adyen,
    Adyenplatform,
    Cybersource,
    Ebanx,
    Payone,
    Paypal,
    Stripe,
    Wise,
}

#[cfg(feature = "payouts")]
impl From<PayoutConnectors> for RoutableConnectors {
    fn from(value: PayoutConnectors) -> Self {
        match value {
            PayoutConnectors::Adyen => Self::Adyen,
            PayoutConnectors::Adyenplatform => Self::Adyenplatform,
            PayoutConnectors::Cybersource => Self::Cybersource,
            PayoutConnectors::Ebanx => Self::Ebanx,
            PayoutConnectors::Payone => Self::Payone,
            PayoutConnectors::Paypal => Self::Paypal,
            PayoutConnectors::Stripe => Self::Stripe,
            PayoutConnectors::Wise => Self::Wise,
        }
    }
}

#[cfg(feature = "payouts")]
impl From<PayoutConnectors> for Connector {
    fn from(value: PayoutConnectors) -> Self {
        match value {
            PayoutConnectors::Adyen => Self::Adyen,
            PayoutConnectors::Adyenplatform => Self::Adyenplatform,
            PayoutConnectors::Cybersource => Self::Cybersource,
            PayoutConnectors::Ebanx => Self::Ebanx,
            PayoutConnectors::Payone => Self::Payone,
            PayoutConnectors::Paypal => Self::Paypal,
            PayoutConnectors::Stripe => Self::Stripe,
            PayoutConnectors::Wise => Self::Wise,
        }
    }
}

#[cfg(feature = "payouts")]
impl TryFrom<Connector> for PayoutConnectors {
    type Error = String;
    fn try_from(value: Connector) -> Result<Self, Self::Error> {
        match value {
            Connector::Adyen => Ok(Self::Adyen),
            Connector::Adyenplatform => Ok(Self::Adyenplatform),
            Connector::Cybersource => Ok(Self::Cybersource),
            Connector::Ebanx => Ok(Self::Ebanx),
            Connector::Payone => Ok(Self::Payone),
            Connector::Paypal => Ok(Self::Paypal),
            Connector::Stripe => Ok(Self::Stripe),
            Connector::Wise => Ok(Self::Wise),
            _ => Err(format!("Invalid payout connector {}", value)),
        }
    }
}

#[cfg(feature = "frm")]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FrmConnectors {
    /// Signifyd Risk Manager. Official docs: https://docs.signifyd.com/
    Signifyd,
    Riskified,
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, strum::Display, strum::EnumString, ToSchema,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FrmAction {
    CancelTxn,
    AutoRefund,
    ManualReview,
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, strum::Display, strum::EnumString, ToSchema,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FrmPreferredFlowTypes {
    Pre,
    Post,
}
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnresolvedResponseReason {
    pub code: String,
    /// A message to merchant to give hint on next action he/she should do to resolve
    pub message: String,
}

/// Possible field type of required fields in payment_method_data
#[derive(
    Clone,
    Debug,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FieldType {
    UserCardNumber,
    UserCardExpiryMonth,
    UserCardExpiryYear,
    UserCardCvc,
    UserFullName,
    UserEmailAddress,
    UserPhoneNumber,
    UserPhoneNumberCountryCode,           //phone number's country code
    UserCountry { options: Vec<String> }, //for country inside payment method data ex- bank redirect
    UserCurrency { options: Vec<String> },
    UserCryptoCurrencyNetwork, //for crypto network associated with the cryptopcurrency
    UserBillingName,
    UserAddressLine1,
    UserAddressLine2,
    UserAddressCity,
    UserAddressPincode,
    UserAddressState,
    UserAddressCountry { options: Vec<String> },
    UserShippingName,
    UserShippingAddressLine1,
    UserShippingAddressLine2,
    UserShippingAddressCity,
    UserShippingAddressPincode,
    UserShippingAddressState,
    UserShippingAddressCountry { options: Vec<String> },
    UserBlikCode,
    UserBank,
    Text,
    DropDown { options: Vec<String> },
    UserDateOfBirth,
    UserVpaId,
    LanguagePreference { options: Vec<String> },
    UserPixKey,
    UserCpf,
    UserCnpj,
}

impl FieldType {
    pub fn get_billing_variants() -> Vec<Self> {
        vec![
            Self::UserBillingName,
            Self::UserAddressLine1,
            Self::UserAddressLine2,
            Self::UserAddressCity,
            Self::UserAddressPincode,
            Self::UserAddressState,
            Self::UserAddressCountry { options: vec![] },
        ]
    }

    pub fn get_shipping_variants() -> Vec<Self> {
        vec![
            Self::UserShippingName,
            Self::UserShippingAddressLine1,
            Self::UserShippingAddressLine2,
            Self::UserShippingAddressCity,
            Self::UserShippingAddressPincode,
            Self::UserShippingAddressState,
            Self::UserShippingAddressCountry { options: vec![] },
        ]
    }
}

/// This implementatiobn is to ignore the inner value of UserAddressCountry enum while comparing
impl PartialEq for FieldType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UserCardNumber, Self::UserCardNumber) => true,
            (Self::UserCardExpiryMonth, Self::UserCardExpiryMonth) => true,
            (Self::UserCardExpiryYear, Self::UserCardExpiryYear) => true,
            (Self::UserCardCvc, Self::UserCardCvc) => true,
            (Self::UserFullName, Self::UserFullName) => true,
            (Self::UserEmailAddress, Self::UserEmailAddress) => true,
            (Self::UserPhoneNumber, Self::UserPhoneNumber) => true,
            (Self::UserPhoneNumberCountryCode, Self::UserPhoneNumberCountryCode) => true,
            (
                Self::UserCountry {
                    options: options_self,
                },
                Self::UserCountry {
                    options: options_other,
                },
            ) => options_self.eq(options_other),
            (
                Self::UserCurrency {
                    options: options_self,
                },
                Self::UserCurrency {
                    options: options_other,
                },
            ) => options_self.eq(options_other),
            (Self::UserCryptoCurrencyNetwork, Self::UserCryptoCurrencyNetwork) => true,
            (Self::UserBillingName, Self::UserBillingName) => true,
            (Self::UserAddressLine1, Self::UserAddressLine1) => true,
            (Self::UserAddressLine2, Self::UserAddressLine2) => true,
            (Self::UserAddressCity, Self::UserAddressCity) => true,
            (Self::UserAddressPincode, Self::UserAddressPincode) => true,
            (Self::UserAddressState, Self::UserAddressState) => true,
            (Self::UserAddressCountry { .. }, Self::UserAddressCountry { .. }) => true,
            (Self::UserShippingName, Self::UserShippingName) => true,
            (Self::UserShippingAddressLine1, Self::UserShippingAddressLine1) => true,
            (Self::UserShippingAddressLine2, Self::UserShippingAddressLine2) => true,
            (Self::UserShippingAddressCity, Self::UserShippingAddressCity) => true,
            (Self::UserShippingAddressPincode, Self::UserShippingAddressPincode) => true,
            (Self::UserShippingAddressState, Self::UserShippingAddressState) => true,
            (Self::UserShippingAddressCountry { .. }, Self::UserShippingAddressCountry { .. }) => {
                true
            }
            (Self::UserBlikCode, Self::UserBlikCode) => true,
            (Self::UserBank, Self::UserBank) => true,
            (Self::Text, Self::Text) => true,
            (
                Self::DropDown {
                    options: options_self,
                },
                Self::DropDown {
                    options: options_other,
                },
            ) => options_self.eq(options_other),
            (Self::UserDateOfBirth, Self::UserDateOfBirth) => true,
            (Self::UserVpaId, Self::UserVpaId) => true,
            (Self::UserPixKey, Self::UserPixKey) => true,
            (Self::UserCpf, Self::UserCpf) => true,
            (Self::UserCnpj, Self::UserCnpj) => true,
            (Self::LanguagePreference { .. }, Self::LanguagePreference { .. }) => true,
            _unused => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partialeq_for_field_type() {
        let user_address_country_is_us = FieldType::UserAddressCountry {
            options: vec!["US".to_string()],
        };

        let user_address_country_is_all = FieldType::UserAddressCountry {
            options: vec!["ALL".to_string()],
        };

        assert!(user_address_country_is_us.eq(&user_address_country_is_all))
    }
}

/// Denotes the retry action
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
    Clone,
    PartialEq,
    Eq,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RetryAction {
    /// Payment can be retried from the client side until the payment is successful or payment expires or the attempts(configured by the merchant) for payment are exhausted
    ManualRetry,
    /// Denotes that the payment is requeued
    Requeue,
}

#[derive(Clone, Copy)]
pub enum LockerChoice {
    HyperswitchCardVault,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PmAuthConnectors {
    Plaid,
}

pub fn convert_pm_auth_connector(connector_name: &str) -> Option<PmAuthConnectors> {
    PmAuthConnectors::from_str(connector_name).ok()
}

pub fn convert_authentication_connector(connector_name: &str) -> Option<AuthenticationConnectors> {
    AuthenticationConnectors::from_str(connector_name).ok()
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
    Hash,
)]
pub enum PaymentChargeType {
    #[serde(untagged)]
    Stripe(StripeChargeType),
}

impl Default for PaymentChargeType {
    fn default() -> Self {
        Self::Stripe(StripeChargeType::default())
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    Hash,
    Eq,
    PartialEq,
    ToSchema,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum StripeChargeType {
    #[default]
    Direct,
    Destination,
}

#[cfg(feature = "frm")]
pub fn convert_frm_connector(connector_name: &str) -> Option<FrmConnectors> {
    FrmConnectors::from_str(connector_name).ok()
}
