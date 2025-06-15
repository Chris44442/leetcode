use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use memmap2::MmapOptions;
use std::slice::{from_raw_parts_mut, from_raw_parts};
use std::ptr::{write_volatile,read_volatile};

const DEVMEM_PATH: &str = "/dev/mem";
const LEN: usize = 4;

fn read_reg(adr: usize, bitmask: Option<u32>) -> u32 {
    let f = OpenOptions::new().read(true).custom_flags(libc::O_SYNC).write(false).create(false).open(DEVMEM_PATH).expect("Error opening file path");
    let mmap = unsafe { MmapOptions::new().offset(adr as u64).len(LEN).map(&f).expect("Error creating mmap") };
    let slice: &[u32] = unsafe { from_raw_parts(mmap.as_ptr() as *mut u32, mmap.len() / 4) };
    let read_val = unsafe {read_volatile(&slice[0]) };

    match bitmask {
        Some(mask) => {
            let masked = read_val & mask;
            let shift = mask.trailing_zeros();
            masked >> shift
        }
        None => read_val,
    }
}

fn write_reg(adr: usize, val: u32, bitmask: Option<u32>) {
    let f = OpenOptions::new().read(true).write(true).create(true).open(DEVMEM_PATH).expect("Error opening file path");
    let mut mmap = unsafe { MmapOptions::new().offset(adr as u64).len(LEN).map_mut(&f).expect("Error creating mutable mmap") };
    let slice = unsafe { from_raw_parts_mut(mmap.as_mut_ptr() as *mut u32, mmap.len() / 4) };

    match bitmask {
        Some(mask) => {
            let shift = mask.trailing_zeros();
            let new_value_at_bit_position = (val << shift) & mask;
            let old_remaining_bit_values = unsafe {read_volatile(&slice[0])} & !mask;
            unsafe {write_volatile(&mut slice[0], new_value_at_bit_position | old_remaining_bit_values)}
        }
        None => unsafe {write_volatile(&mut slice[0], val)},
    }

}

pub struct RegMap {
    base_addr: usize,
}

impl RegMap {
    pub const fn new() -> Self {
        RegMap { base_addr : 0xFF200000 }
    }

    const STATUS_REG_ADR: usize = 0x1000;
    const STATUS_REG_AFFE_MASK: u32 = 0xFFFF << 16;
    // const STATUS_ERROR: u32 = 1 << 1;

    pub fn read_status_reg(&self) -> u32 {
        let val = read_reg(self.base_addr + Self::STATUS_REG_ADR, None);
        val
    }

    pub fn read_status_affe_regfield(&self) -> u32 {
        let val = read_reg(self.base_addr + Self::STATUS_REG_ADR, Some(Self::STATUS_REG_AFFE_MASK));
        val
    }

    pub fn write_status_reg(&self, val: u32) {
        write_reg(self.base_addr + Self::STATUS_REG_ADR, val, None);
    }

    pub fn write_status_affe_regfield(&self, val: u32) {
        write_reg(self.base_addr + Self::STATUS_REG_ADR, val, Some(Self::STATUS_REG_AFFE_MASK));
    }

}

// fn main() {
//
//     let regmap = RegMap::new();
//
//     let test_val: u32 = 0xaffe1133;
//     regmap.write_status_reg(test_val);
//
//     let read_back = regmap.read_status_reg();
//     println!("status reg is {:x}", read_back);
//
//
//     let lul: u32 = 0x4376;
//
//     regmap.write_status_affe_regfield(lul);
//     let read_back = regmap.read_status_reg();
//     println!("status reg is {:x}", read_back);
// }


use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// mod regmap;
// use regmap::RegMap;

#[derive(Deserialize)]
struct WriteValue {
    value: u32,
}

#[derive(Serialize)]
struct ReadResponse {
    value: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/status", get(read_status).post(write_status))
        .route("/status/affe", get(read_status_affe).post(write_status_affe));

    // let addr = SocketAddr::from(("192.168.1.2".parse().unwrap(), 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    let tcp_listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // let _ = axum::serve(tcp_listener, app.into_make_service());

axum::serve(tcp_listener, app.into_make_service()).await.unwrap();
}

// Handlers
async fn read_status() -> Json<ReadResponse> {
    let regmap = RegMap::new();
    let value = regmap.read_status_reg();
    Json(ReadResponse { value })
}

async fn write_status(Json(payload): Json<WriteValue>) {
    let regmap = RegMap::new();
    regmap.write_status_reg(payload.value);
}

async fn read_status_affe() -> Json<ReadResponse> {
    let regmap = RegMap::new();
    let value = regmap.read_status_affe_regfield();
    Json(ReadResponse { value })
}

async fn write_status_affe(Json(payload): Json<WriteValue>) {
    let regmap = RegMap::new();
    regmap.write_status_affe_regfield(payload.value);
}
