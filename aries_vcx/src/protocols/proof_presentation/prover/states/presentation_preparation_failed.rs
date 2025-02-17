use crate::protocols::proof_presentation::prover::states::finished::FinishedState;
use messages::concepts::problem_report::ProblemReport;
use messages::protocols::proof_presentation::presentation_request::PresentationRequest;
use messages::status::Status;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationPreparationFailedState {
    pub presentation_request: PresentationRequest,
    pub problem_report: ProblemReport,
}

impl From<PresentationPreparationFailedState> for FinishedState {
    fn from(state: PresentationPreparationFailedState) -> Self {
        trace!("transit state from PresentationPreparationFailedState to FinishedState");
        FinishedState {
            presentation_request: Some(state.presentation_request),
            presentation: None,
            status: Status::Failed(state.problem_report),
        }
    }
}
