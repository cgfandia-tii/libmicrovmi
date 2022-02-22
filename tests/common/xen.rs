use std::process::{Command, Stdio};

use log::debug;
use microvmi::api::params::{CommonInitParams, DriverInitParams};
use microvmi::api::{DriverType, Introspectable};
use microvmi::init;

use super::config::{CommonConfig, XenConfig};
use crate::common::context::Context;

#[derive(Default, Clone)]
pub struct Xen {
    config: XenConfig,
}

impl Context for Xen {
    /// restore VM from the Xen checkpoint file
    fn setup(&self) {
        debug!("setup test");
        Command::new("xl")
            .arg("restore")
            .arg(&self.config.checkpoint)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to start xl")
            .success()
            .then(|| 0)
            .expect("Failed to restore Xen checkpoint");
    }

    fn init_driver(&self) -> Box<dyn Introspectable> {
        init(
            Some(DriverType::Xen),
            Some(DriverInitParams {
                common: Some(CommonInitParams {
                    vm_name: self.config.common.vm.clone(),
                }),
                ..Default::default()
            }),
        )
        .expect("Failed to init Xen libmicrovmi driver")
    }

    /// shutdown VM
    fn teardown(&self) {
        debug!("teardown test");
        Command::new("xl")
            .arg("destroy")
            .arg(&self.config.common.vm)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to start xl")
            .success()
            .then(|| 0)
            .expect("Failed to destroy Xen VM");
    }

    fn config(&self) -> &CommonConfig {
        &self.config.common
    }
}
