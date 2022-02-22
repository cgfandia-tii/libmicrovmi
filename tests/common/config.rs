use std::convert::TryInto;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CommonConfig {
    pub vm: String,
    pub vcpu: u16,
    pub timeout: u64,
    pub event_timeout: u32,
}

impl Default for CommonConfig {
    fn default() -> Self {
        CommonConfig {
            vm: env::var("TEST_VM").unwrap_or("winxp".to_string()),
            vcpu: 1,
            timeout: env::var("TEST_TIMEOUT")
                .unwrap_or("20".to_string())
                .parse()
                .unwrap(),
            event_timeout: env::var("TEST_EVENT_TIMEOUT")
                .unwrap_or("5000".to_string())
                .parse()
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KVMConfig {
    pub common: CommonConfig,
    pub virsh_uri: String,
    pub kvmi_socket: String,
}

impl Default for KVMConfig {
    fn default() -> Self {
        KVMConfig {
            common: CommonConfig::default(),
            virsh_uri: env::var("TEST_KVM_VIRSH_URI").unwrap_or("qemu:///system".to_string()),
            kvmi_socket: env::var("TEST_KVMI_SOCKET").unwrap_or("/tmp/introspector".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XenConfig {
    pub common: CommonConfig,
    pub checkpoint: PathBuf,
}

impl Default for XenConfig {
    fn default() -> Self {
        XenConfig {
            common: CommonConfig::default(),
            checkpoint: env::var("TEST_XEN_CHECKPOINT")
                .unwrap_or("/tmp/xen-checkpoint".to_string())
                .try_into()
                .unwrap(),
        }
    }
}
