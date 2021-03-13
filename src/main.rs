
use std::{str::FromStr, time::Duration};

//Attributes to connect to tenma device and endpoint
const TENMA_PRODUCT_ID: u16 = 57352;
const TENMA_VENDOR_ID: u16 = 6790;
const TENMA_READ_ENDPOINT: u8 = 0x82;
const TENMA_ENDPOINT_CONFIG: u8 = 1;
const TENMA_ENDPOINT_INTERFACE: u8 = 0;
const TENMA_ENDPOINT_SETTING: u8 = 0;

const NO_DATA_IDENTIFIER: u8 = 0xF0;
const DATA_TERMINATOR: u8 = 0x8A;

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

            eprintln!("Tenma devices: {:?}", tenma_devices);

            assert_ne!(tenma_devices.len(), 0);

            let tenma_device = & mut tenma_devices[0];

            eprintln!("Chosen Tenma: {:?}", tenma_device.device_descriptor());

            eprintln!("Opening device...");

            match tenma_device.open() {
                Ok(mut handle) => {
                    eprintln!("Device open.");


                    eprintln!("Get device name: {}", handle.read_product_string_ascii(&handle.device().device_descriptor().unwrap()).unwrap());

                    /*handle.set_active_configuration(TENMA_ENDPOINT_CONFIG).unwrap();
                    handle.claim_interface(TENMA_ENDPOINT_INTERFACE).unwrap();
                    handle.set_alternate_setting(TENMA_ENDPOINT_INTERFACE, TENMA_ENDPOINT_SETTING).unwrap();*/

                    let mut usb_reader_buffer = [0; 8];
                    let timeout = Duration::from_secs(1);

                    let mut tenma_data_buffer = [0; 12];
                    let mut index = 0;

                    loop {
                        match handle.read_interrupt(TENMA_READ_ENDPOINT, &mut usb_reader_buffer, timeout) {
                            Ok(len) => {
                                assert_eq!(len, 8);

                                if usb_reader_buffer[0] == NO_DATA_IDENTIFIER {
                                    index = 0;
                                } else {
                                    tenma_data_buffer[index] = usb_reader_buffer[1];
                                    index += 1;
                                }

                                if usb_reader_buffer[1] == DATA_TERMINATOR {
                                    println!("Data: {:?}", tenma_data_buffer);
                                }




                            }
                            Err(err) => panic!("could not read from endpoint: {}", err),
                        }
                    }



                },
                Err(e) => {
                    panic!("Could not open device - {}", e);
                }
            }


        }
        Err(e) => {
            panic!("Error getting device list: {}", e);

        }
    }

}
