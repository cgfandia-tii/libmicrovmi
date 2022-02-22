# Integration tests

Instructions for all tests:
- Tests are configured dynamically by the environment variables:
    - `TEST_VM=winxp` - target VM name
    - `TEST_VCPU=1` - VM vCPU count
    - `TEST_TIMEOUT=20` - test timeout in seconds
    - `TEST_EVENT_TIMEOUT=5000` - timeout of events listener in milliseconds
- `cargo test -- --nocapture`: displays the `log` output, useful for debugging

## KVM

**Requirements**
- virtual machine already configured to be introspected by KVM-VMI
- VM snapshot with live state
- [libkvmi](https://github.com/bitdefender/libkvmi)
- [`virsh`](https://libvirt.org/manpages/virsh.html) tool: (`libvirt-clients` package)

**Tests configuration**
- `TEST_KVM_VIRSH_URI=qemu:///` - virsh backend
- `TEST_KVMI_SOCKET=/tmp/introspector` - KVMI socket path

The VM state between each test is handled by the following commands:
- setup: `virsh snapshot-revert <vm_name> --current --running`
- teardown: `virsh destroy <vm_name>`

**Execution**

~~~
cargo test --features=kvm
~~~

## Xen

**Requirements**
- root privileges
- VM checkpoint with live state
- [`xl` toolstack](https://wiki.xenproject.org/wiki/XL) - probably already installed with Xen hypervisor

Create Xen checkpoint: `xl save -c <vm_name> <checkpoint>`

**Tests configuration**
- `TEST_XEN_CHECKPOINT=/tmp/xen-checkpoint` - path to VM checkpoint

The VM state between each test is handled by the following commands:
- setup: `xl restore <checkpoint>`
- teardown: `xl destroy <vm_name>`

**Execution**

~~~
cargo test --features=xen
~~~
