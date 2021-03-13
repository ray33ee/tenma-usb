
const TENMA_PRODUCT_ID: u16 = 57352;
const TENMA_VENDOR_ID: u16 = 6790;

fn main() {

    match rusb::devices() {
        Ok(list) => {

            //Create and populate a list of tenma devices
            let mut tenma_devices = vec![];

            for device in list.iter() {

                match device.device_descriptor() {
                    Ok(descriptor) => {
                        if descriptor.product_id() == TENMA_PRODUCT_ID && descriptor.vendor_id() == TENMA_VENDOR_ID {
                            tenma_devices.push(device);
                        }
                    },
                    Err(e) => {
                        panic!("Error getting device descriptor: {}", e);
                    }
                }

            }

            let tenma_device = &tenma_devices[0];

            eprintln!("Devices: {:?}", tenma_device.device_descriptor());



        }
        Err(e) => {
            panic!("Error getting device list: {}", e);

        }
    }

}
