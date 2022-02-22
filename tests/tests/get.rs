use crate::common::config::CommonConfig;

use super::IntegrationTest;
use microvmi::api::Introspectable;

fn get_vcpu_count(drv: Box<dyn Introspectable>, cfg: CommonConfig) {
    assert_eq!(cfg.vcpu, drv.get_vcpu_count().unwrap());
}

inventory::submit!(IntegrationTest {
    name: "get_vcpu_count",
    test_fn: get_vcpu_count
});
