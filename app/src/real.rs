#[serde_zod::my_attribute]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "kind")]
pub enum BlockingState {
    Blocked,
    Allowed { reason: AllowReason },
}

#[serde_zod::my_attribute]
#[derive(Debug, Clone, serde::Serialize)]
pub enum AllowReason {
    ProtectionDisabled,
    OwnedByFirstParty,
    RuleException,
    AdClickAttribution,
    OtherThirdPartyRequest,
}

#[serde_zod::my_attribute]
#[derive(Debug, Clone, serde::Serialize)]
pub struct DetectedRequest {
    url: String,
    state: BlockingState,
    owner_name: Option<String>,
    entity_name: Option<String>,
    category: Option<String>,
    prevalence: Option<f32>,
    page_url: String,
}

// #[test]
// fn test_01() -> Result<(), serde_json::Error> {
//     #[derive(serde::Serialize)]
//     struct Request {
//         state: BlockingState,
//     };
//     let r = Request {
//         state: BlockingState::Allowed(AllowReason::OwnedByFirstParty),
//     };
//     let json = serde_json::to_string_pretty(&r)?;
//
//     println!("{}", json);
//     Ok(())
// }
