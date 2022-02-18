use std::process::{Command, Stdio};

use log::debug;
use microvmi::api::params::{CommonInitParams, DriverInitParams};
use microvmi::api::{DriverType, Introspectable};
use microvmi::init;

use super::config::{VIRSH_URI_XEN, VM_NAME};
use crate::common::context::Context;

pub struct Xen;

impl Context for Xen {
    /// restore VM state from internal QEMU snapshot
    fn setup(&self) {
        debug!("setup test");
        Command::new("virsh")
            .arg(format!("--connect={}", VIRSH_URI_XEN))
            .arg("start")
            .arg(VM_NAME)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to start virsh")
            .success()
            .then(|| 0)
            .expect("Failed to run virsh start");
    }

    fn init_driver(&self) -> Box<dyn Introspectable> {
        init(
            Some(DriverType::Xen),
            Some(DriverInitParams {
                common: Some(CommonInitParams {
                    vm_name: String::from(VM_NAME),
                }),
                ..Default::default()
            }),
        )
        .expect("Failed to init libmicrovmi")
    }

    /// shutdown VM
    fn teardown(&self) {
        debug!("teardown test");
        Command::new("virsh")
            .arg(format!("--connect={}", VIRSH_URI_XEN))
            .arg("destroy")
            .arg(VM_NAME)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to start virsh")
            .success()
            .then(|| 0)
            .expect("Failed to run virsh destroy");
    }
}
