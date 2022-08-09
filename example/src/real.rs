#[serde_zod::codegen]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum BlockingState {
    Blocked,
    Allowed { reason: AllowReason },
}

#[serde_zod::codegen]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AllowReason {
    ProtectionDisabled,
    OwnedByFirstParty,
    RuleException,
    AdClickAttribution,
    OtherThirdPartyRequest,
}

#[serde_zod::codegen]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DetectedRequest {
    url: String,
    state: BlockingState,
    owner_name: Option<String>,
    entity_name: Option<String>,
    category: Option<String>,
    prevalence: Option<f32>,
    page_url: String,
}
