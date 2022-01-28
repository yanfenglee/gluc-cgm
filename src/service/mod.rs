
pub use cgm_service::CgmService;

pub mod cgm_service;

lazy_static! {
   pub static ref CGM_SERVICE: CgmService = CgmService{};
}
