const ENDPOINT_UNUSED: u8 = 0x00;
const ENDPOINT_TRANSMIT_ONLY: u8 = 0x15;
const ENDPOINT_RECEIVE_ONLY: u8 = 0x19;
const ENDPOINT_TRANSMIT_AND_RECEIVE: u8 = 0x1D;
const ENDPOINT_RECEIVE_ISOCHRONOUS: u8 = 0x18;
const ENDPOINT_TRANSMIT_ISOCHRONOUS: u8 = 0x14;

const VENDOR_ID: u16 = 0x16C0;
const PRODUCT_ID: u16 = 0x0483;
const DEVICE_CLASS: u8 = 2; // 2 = Communication Class
const MANUFACTURER_NAME: [char; 11] = ['T', 'e', 'e', 'n', 's', 'e', 'r', 'i', 'a', 'l', '!'];
const MANUFACTURER_NAME_LEN: u8 = 11;
const PRODUCT_NAME: [char; 10] = ['U', 'S', 'B', ' ', 'S', 'e', 'r', 'i', 'a', 'l'];
const PRODUCT_NAME_LEN: u8 = 10;
const EP0_SIZE: u8 = 64;
const NUM_ENDPOINTS: u8 = 4;
const NUM_USB_BUFFERS: u8 = 12;
const NUM_INTERFACE: u8 = 2;
const CDC_STATUS_INTERFACE: u8 = 0;
const CDC_DATA_INTERFACE: u8 = 1;
const CDC_ACM_ENDPOINT: u8 = 2;
const CDC_RX_ENDPOINT: u8 = 3;
const CDC_TX_ENDPOINT: u8 = 4;
const CDC_ACM_SIZE: u8 = 16;
const CDC_RX_SIZE: u8 = 64;
const CDC_TX_SIZE: u8 = 64;
const ENDPOINT2_CONFIG: u8 = ENDPOINT_TRANSMIT_ONLY;
const ENDPOINT3_CONFIG: u8 = ENDPOINT_RECEIVE_ONLY;
const ENDPOINT4_CONFIG: u8 = ENDPOINT_TRANSMIT_ONLY;

pub fn MSB(n: u16) -> u8 {
    ((n >> 8) & 255) as u8
}
pub fn LSB(n: u16) -> u8 {
    (n & 255) as u8
}

#[repr(C, packed)]
struct DeviceDescriptor {
    bLength: u8,
    bDescriptorType: u8,
    bcdUSB: u16,
    bDeviceClass: u8,
    bDeviceSubClass: u8,
    bDeviceProtocol: u8,
    bMaxPacketSize0: u8,
    idVendor: u16,
    idProduct: u16,
    bcdDevice: u16,
    iManufacturer: u8,
    iProduct: u8,
    iSerialNumber: u8,
    bNumConfigurations: u8,
}

impl DeviceDescriptor {
    pub fn new_USB_serial_descriptor() -> Self {
        DeviceDescriptor {
            bLength: 18,
            bDescriptorType: 1,
            bcdUSB: 0x0110,
            bDeviceClass: DEVICE_CLASS,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize0: EP0_SIZE,
            idVendor: VENDOR_ID,
            idProduct: PRODUCT_ID,
            bcdDevice: 0x0275,
            iManufacturer: 1,
            iProduct: 2,
            iSerialNumber: 3,
            bNumConfigurations: 1,
        }
    }
}
