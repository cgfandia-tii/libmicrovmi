use std::process::{Command, Stdio};

use log::debug;
use microvmi::api::params::{CommonInitParams, DriverInitParams, KVMInitParams};
use microvmi::api::{DriverType, Introspectable};
use microvmi::init;

use super::config::{CommonConfig, KVMConfig};
use crate::common::context::Context;

#[derive(Default, Clone)]
pub struct KVM {
    config: KVMConfig,
}

impl Context for KVM {
    /// restore VM state from internal QEMU snapshot
    fn setup(&self) {
        debug!("setup test");
        Command::new("virsh")
            .arg(format!("--connect={}", self.config.virsh_uri))
            .arg("snapshot-revert")
            .arg(&self.config.common.vm)
            .arg("--current")
            .arg("--running")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to start virsh")
            .success()
            .then(|| 0)
            .expect("Failed to run virsh snapshot-revert");
    }

    fn init_driver(&self) -> Box<dyn Introspectable> {
        init(
            Some(DriverType::KVM),
            Some(DriverInitParams {
                common: Some(CommonInitParams {
                    vm_name: self.config.common.vm.clone(),
                }),
                kvm: Some(KVMInitParams::UnixSocket {
                    path: self.config.kvmi_socket.to_string(),
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
            .arg(format!("--connect={}", self.config.virsh_uri))
            .arg("destroy")
            .arg(&self.config.common.vm)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to start virsh")
            .success()
            .then(|| 0)
            .expect("Failed to run virsh destroy");
    }

    fn config(&self) -> &CommonConfig {
        &self.config.common
    }
}
