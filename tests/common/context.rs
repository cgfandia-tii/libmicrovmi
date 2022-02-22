use super::config::CommonConfig;
use super::kvm::KVM;
use super::xen::Xen;
use microvmi::api::Introspectable;

pub trait Context: Send {
    fn setup(&self) -> ();
    fn init_driver(&self) -> Box<dyn Introspectable>;
    fn teardown(&self) -> ();
    fn config(&self) -> &CommonConfig;
}

pub fn init_context() -> Box<dyn Context> {
    if cfg!(feature = "kvm") {
        Box::new(KVM::default())
    } else if cfg!(feature = "xen") {
        Box::new(Xen::default())
    } else {
        panic!("Integration tests need to be run with a specific driver enabled")
    }
}
