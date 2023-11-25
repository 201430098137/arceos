#![no_std]


#[macro_use]
extern crate log;

extern crate fdt;

use fdt::{node::FdtNode, standard_nodes::Compatible, Fdt};
pub use fdt::{FdtError};

extern crate alloc;
use alloc::vec::Vec;

//use core::ptr::NonNull;

// mod virtio_drivers;
// use virtio_drivers::transport::{
//         mmio::{MmioTransport, VirtIOHeader},
//         DeviceType, Transport,
// };



pub fn parse_dt(dtb: usize) -> Result<DtbInfo, FdtError>  {
    info!("device tree @ {:#x}", dtb);
    // Safe because the pointer is a valid pointer to unaliased memory.
    let fdt_result = unsafe { Fdt::from_ptr(dtb as *const u8) };
    match fdt_result {
        Ok(fdt) => Ok(walk_dt(fdt)),
        Err(e) => Err(e),
    }
}

pub struct DtbInfo {
   pub memory_addr: usize,
   pub memory_size: usize,
   pub mmio_regions: Vec<(usize, usize)>,
}

impl DtbInfo {
    pub const fn new() -> Self {
        DtbInfo {
            memory_addr:0,
            memory_size:0,
            mmio_regions:Vec::new(),
        }
    }
}

fn walk_dt(fdt: Fdt) -> DtbInfo {
    let mut info = DtbInfo::new();
    for node in fdt.all_nodes() {
        if let Some(compatible) = node.compatible() {
            info!("compatible {}", compatible.first());
            if compatible.first() == "virtio,mmio" {
                match virtio_probe(node) {
                    Some(ioDevice) => info.mmio_regions.push((ioDevice.addr, ioDevice.size)),
                    None => (),
                };
            }
        }else {
            info!("not compatible");
            match memory_probe(node) {
                Some(ioDevice) => {
                    info.memory_addr = ioDevice.addr;
                    info.memory_size = ioDevice.size
                },
                None => (),
            };
        }
    }
    info
}

pub struct ioDevice {
    addr: usize,
    size: usize,
}

fn memory_probe(node: FdtNode) -> Option<ioDevice> {
    let p = node.property("device_type");
    match p {
        Some(np) => {
            if np.as_str().unwrap() == "memory" {
                if let Some(reg) = node.reg().and_then(|mut reg| reg.next()) {
                    let paddr = reg.starting_address as usize;
                    let size = reg.size.unwrap();
                    info!("memory addr={:#x}, size={:#x}", paddr, size);
                    return Some(ioDevice{ addr: paddr, size});
                }
            }
            info!("device_type {}", np.as_str().unwrap())
        } ,
        None => (),
    }
    return None;
}

fn riscv_virtio_probe(node: FdtNode) {
    let propertys = node.properties();
    for property in propertys.into_iter() {
        info!("property {:?}", property);
    }
}

fn virtio_probe(node: FdtNode) -> Option<ioDevice> {
    // let propertys = node.properties();
    // for property in propertys.into_iter() {
    //     info!("property {:?}", property);
    // }


    if let Some(reg) = node.reg().and_then(|mut reg| reg.next()) {
        let paddr = reg.starting_address as usize;
        let size = reg.size.unwrap();
        info!("walk dt addr={:#x}, size={:#x}", paddr, size);
        info!("Device tree node {}: {:?}",
            node.name,
            node.compatible().map(Compatible::first),);
        return Some(ioDevice{ addr: paddr, size});
        // let header = NonNull::new(vaddr as *mut VirtIOHeader).unwrap();
        // match unsafe { MmioTransport::new(header) } {
        //     Err(e) => warn!("Error creating VirtIO MMIO transport: {}", e),
        //     Ok(transport) => {
        //         info!(
        //             "Detected virtio MMIO device with vendor id {:#X}, device type {:?}, version {:?}",
        //             transport.vendor_id(),
        //             transport.device_type(),
        //             transport.version(),
        //         );
        //         //virtio_device(transport);
        //     }
        // }
    }
    return None;
}