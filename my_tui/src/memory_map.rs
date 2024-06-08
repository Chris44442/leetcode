
extern crate memmap;

pub struct MemoryMap {
    mmap: memmap::MmapMut,
}

impl MemoryMap {
    pub fn new (offset:u64, len:usize) -> Result<MemoryMap, std::io::Error> {
        let devmem = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")?;
        let mmap = unsafe {
            memmap::MmapOptions::new()
                .offset(offset)
                .len(len)
                .map_mut(&devmem)?
        };
        Ok(MemoryMap { mmap })
    }
    pub fn read (&mut self, addr:usize) -> u32 {
        let slice: &mut [u32] = unsafe {
            std::slice::from_raw_parts_mut(self.mmap.as_mut_ptr() as *mut u32, self.mmap.len()/4)
        };
        unsafe {std::ptr::read_volatile(&slice[addr])}
    }
    pub fn write (&mut self, addr:usize, value:u32) {
        let slice: &mut [u32] = unsafe {
            std::slice::from_raw_parts_mut(self.mmap.as_mut_ptr() as *mut u32, self.mmap.len()/4)
        };
        unsafe {std::ptr::write_volatile(&mut slice[addr] as *mut u32, value)}
    }
}

pub fn read_isle1() -> Result<String, std::io::Error>{
    let offset: u64 = 0xc110_0000;
    let offset: u64 = 0xc100_02FC;
    let len: usize = 4096;
    let mut mmap = MemoryMap::new(offset,len)?;
    let n = 7;

    let description_text = vec![
        "test register",
        "9..8 Azimuth Emulation, 4 ISLE1 LED Blinking, 0 ISLE1 Reset",
        "Interrupt registers",
        "PED Status: 10 Timeout, 9 Communication Err, 8 Ready; Azimuth Encoder: 3 Timeout, 2 Packet Timing Err, 1 Communication Err, 0 Parity Err",
        "0 MK_PWR_ON",
        "0 PDU_CB_ERR",
        "4 PSU_OVERRIDE, 0 RAP_OFF",
        "4 PSU_TEMPWARNING, 0 PSU_GOOD",
        "0 RFU_PWR_ON",
        "0 RFU_PWR_GOOD",
        "0 ANU_PWR_RX_ON",
        "0 ANU_PWR_RX_GOOD",
        "",
        "",
        "",
        "",
        "",
    ];

    let read_write_permisson = vec![" RW ", " RW ", " RW ", "  R ", " RW ", "  R ", " RW ", "  R ", " RW ", "  R ", " RW ", "  R "];
    let addresses_to_read : Vec<usize>= (0..n).collect();
    let values: Vec<u32> = addresses_to_read
        .iter()
        .map(|&addr| mmap.read(addr))
        .collect();
    let vector_of_strings: Vec<String> = addresses_to_read
        .iter()
        .enumerate()
        .map(|(i, addr)| format!("{:08x?} {:08x?} {} {}",
            offset as usize +addr*4, values[i], read_write_permisson[i], description_text[i])).collect();

    Ok(vector_of_strings.join("\n"))
    // let vector_of_strings: String = addresses_to_read.iter().enumerate().map(|(i, addr)|
    //     format!("{:08x?} {:08x?} {} {}", offset as usize +addr*4, values[i], read_write_permisson[i], description_text[i])).collect();

    // println!("{}",single_string);
    // for i in 0..vector_of_strings.len() {
    //     println!("{}",vector_of_strings[i]);
    // }
    // #####################################################################3

    // let offset: u64 = 0xc120_0000;
    // let len: usize = 4096;
    // let mut mmap = MemoryMap::new(offset,len)?;

    // let addresses_to_write : Vec<usize> = vec![0];
    // let values_to_write : Vec<u32> = vec![0xaffe7755];
    // for (&addr, &value) in addresses_to_write.iter().zip(values_to_write.iter()) {
    //     mmap.write(addr, value);
    // }

    // let n = 1;
    // let addresses_to_read : Vec<usize>= (0..n).collect();
    // let values: Vec<u32> = addresses_to_read
    //     .iter()
    //     .map(|&addr| mmap.read(addr))
    //     .collect();
    // for (addr, value) in addresses_to_read.iter().zip(values.iter()) {
    //     println!("{:08x?} {:08x?}", offset as usize +addr*4, value);
    // }
    // Ok(())
}

