pub mod api;
mod driver;

use api::Introspectable;
use api::DriverType;

pub fn init(driver_type: DriverType) -> Box<Introspectable> {
    println!("vmi init");

    match driver_type {
        DriverType::Dummy => {
            Box::new(driver::dummy::Dummy) as Box<Introspectable>
        },
        DriverType::Xen => {
            Box::new(driver::xen::Xen) as Box<Introspectable>
        },
    }
}
