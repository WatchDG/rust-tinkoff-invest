use tinkoff_invest_types;

#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    Unspecified,
    Input,
    BondTax,
    OutputSecurities,
    Overnight,
    Tax,
    BondRepaymentFull,
    SellCard,
    DividendTax,
    Output,
    BondRepayment,
    TaxCorrection,
    ServiceFee,
    BenefitTax,
    MarginFee,
    Buy,
    BuyCard,
    InputSecurities,
    SellMargin,
    BrokerFee,
    BuyMargin,
    Dividend,
    Sell,
    Coupon,
    SuccessFee,
    DividendTransfer,
    AccruingVarmargin,
    WritingOffVarmargin,
    DeliveryBuy,
    DeliverySell,
    TrackMfee,
    TrackPfee,
    TaxProgressive,
    BondTaxProgressive,
    DividendTaxProgressive,
    BenefitTaxProgressive,
    TaxCorrectionProgressive,
    TaxRepoProgressive,
    TaxRepo,
    TaxRepoHold,
    TaxRepoRefund,
    TaxRepoHoldProgressive,
    TaxRepoRefundProgressive,
    DivExt,
    TaxCorrectionCoupon,
}

impl Into<OperationType> for tinkoff_invest_types::OperationType {
    fn into(self) -> OperationType {
        match self {
            tinkoff_invest_types::OperationType::Unspecified => OperationType::Unspecified,
            tinkoff_invest_types::OperationType::Input => OperationType::Input,
            tinkoff_invest_types::OperationType::BondTax => OperationType::BondTax,
            tinkoff_invest_types::OperationType::OutputSecurities => {
                OperationType::OutputSecurities
            }
            tinkoff_invest_types::OperationType::Overnight => OperationType::Overnight,
            tinkoff_invest_types::OperationType::Tax => OperationType::Tax,
            tinkoff_invest_types::OperationType::BondRepaymentFull => {
                OperationType::BondRepaymentFull
            }
            tinkoff_invest_types::OperationType::SellCard => OperationType::SellCard,
            tinkoff_invest_types::OperationType::DividendTax => OperationType::DividendTax,
            tinkoff_invest_types::OperationType::Output => OperationType::Output,
            tinkoff_invest_types::OperationType::BondRepayment => OperationType::BondRepayment,
            tinkoff_invest_types::OperationType::TaxCorrection => OperationType::TaxCorrection,
            tinkoff_invest_types::OperationType::ServiceFee => OperationType::ServiceFee,
            tinkoff_invest_types::OperationType::BenefitTax => OperationType::BenefitTax,
            tinkoff_invest_types::OperationType::MarginFee => OperationType::MarginFee,
            tinkoff_invest_types::OperationType::Buy => OperationType::Buy,
            tinkoff_invest_types::OperationType::BuyCard => OperationType::BuyCard,
            tinkoff_invest_types::OperationType::InputSecurities => OperationType::InputSecurities,
            tinkoff_invest_types::OperationType::SellMargin => OperationType::SellMargin,
            tinkoff_invest_types::OperationType::BrokerFee => OperationType::BrokerFee,
            tinkoff_invest_types::OperationType::BuyMargin => OperationType::BuyMargin,
            tinkoff_invest_types::OperationType::Dividend => OperationType::Dividend,
            tinkoff_invest_types::OperationType::Sell => OperationType::Sell,
            tinkoff_invest_types::OperationType::Coupon => OperationType::Coupon,
            tinkoff_invest_types::OperationType::SuccessFee => OperationType::SuccessFee,
            tinkoff_invest_types::OperationType::DividendTransfer => {
                OperationType::DividendTransfer
            }
            tinkoff_invest_types::OperationType::AccruingVarmargin => {
                OperationType::AccruingVarmargin
            }
            tinkoff_invest_types::OperationType::WritingOffVarmargin => {
                OperationType::WritingOffVarmargin
            }
            tinkoff_invest_types::OperationType::DeliveryBuy => OperationType::DeliveryBuy,
            tinkoff_invest_types::OperationType::DeliverySell => OperationType::DeliverySell,
            tinkoff_invest_types::OperationType::TrackMfee => OperationType::TrackMfee,
            tinkoff_invest_types::OperationType::TrackPfee => OperationType::TrackPfee,
            tinkoff_invest_types::OperationType::TaxProgressive => OperationType::TaxProgressive,
            tinkoff_invest_types::OperationType::BondTaxProgressive => {
                OperationType::BondTaxProgressive
            }
            tinkoff_invest_types::OperationType::DividendTaxProgressive => {
                OperationType::DividendTaxProgressive
            }
            tinkoff_invest_types::OperationType::BenefitTaxProgressive => {
                OperationType::BenefitTaxProgressive
            }
            tinkoff_invest_types::OperationType::TaxCorrectionProgressive => {
                OperationType::TaxCorrectionProgressive
            }
            tinkoff_invest_types::OperationType::TaxRepoProgressive => {
                OperationType::TaxRepoProgressive
            }
            tinkoff_invest_types::OperationType::TaxRepo => OperationType::TaxRepo,
            tinkoff_invest_types::OperationType::TaxRepoHold => OperationType::TaxRepoHold,
            tinkoff_invest_types::OperationType::TaxRepoRefund => OperationType::TaxRepoRefund,
            tinkoff_invest_types::OperationType::TaxRepoHoldProgressive => {
                OperationType::TaxRepoHoldProgressive
            }
            tinkoff_invest_types::OperationType::TaxRepoRefundProgressive => {
                OperationType::TaxRepoRefundProgressive
            }
            tinkoff_invest_types::OperationType::DivExt => OperationType::DivExt,
            tinkoff_invest_types::OperationType::TaxCorrectionCoupon => {
                OperationType::TaxCorrectionCoupon
            }
        }
    }
}
