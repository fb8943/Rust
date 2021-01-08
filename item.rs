use serde::{Deserialize, Serialize};

//Item and Status are define in this file from
// https://plaid.com/docs/api/items/#itemget



//this is use in Error_Item
enum ErrorType {
    "INVALID_REQUEST",
    "INVALID_INPUT",
    "INSTITUTION_ERROR",
    "RATE_LIMIT_EXCEEDED",
    "API_ERROR, ITEM_ERROR",
    "ASSET_REPORT_ERROR",
    "RECAPTCHA_ERROR",
    "OAUTH_ERROR",
    "PAYMENT_ERROR",
    "BANK_TRANSFER_ERROR",
}

//this is use in Item
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error_Item{
    pub error_type:ErrorType,
    pub error_code:String,
    pub display_message:Option<String>,
    pub request_id:Options<String>,
    pub causes:Vector<String>,//not sure if this causes are String
    pub documentation_url:Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct item{
    pub item_id: String,
    pub institution_id: Option<String>,
    pub webhook: Option<String>,
    pub error:Option<Error_Item>
    pub available_products:Vector<String>,
    pub consent_expiration_time:Option<String>,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct investment{
    pub last_successful_update:Option<String>,
    pub last_failed_update:Option<String>,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct transaction{
    pub last_successful_update:Option<String>,
    pub last_failed_update:Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct lastwebhook{
    pub sent_at:Option<String>,
    pub code_sent:Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct status{
    pub investments:Option<investment>,
    pub transactions:Option<transaction>,
    pub last_webhook:Option<lastwebhook>,
    pub request_id:String,
    pub access_token:Option<String>
    pub otp_success:Option<bool>
}

