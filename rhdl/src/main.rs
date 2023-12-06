use rust_hdl::prelude::*;


#[derive(LogicBlock)]  // <- This turns the struct into something you can simulate/synthesize
struct Adder {
    pub clock: Signal<In, Clock>, // <- input signal, type is clock
    pub reset: Signal<In, Bit>,
    pub cnt: Signal<Out, Bits<32>>,    // <- output signal, type is single bit
    cnt_reg: DFF<Bits<8>>,
    ram: RAM<Bits<32>,8>,
}

impl Default for Adder {
   fn default() -> Self {
       Self {
         clock: Default::default(),
         cnt: Default::default(),
         reset: Default::default(),
         cnt_reg: Default::default(),
         ram: Default::default(),
       }
    }
}

impl Logic for Adder {
    #[hdl_gen] // <- this turns the update function into an HDL Kernel that can be turned into Verilog
    fn update(&mut self) {
       // v-- write to the .next member     v-- read from .val() method
        dff_setup!(self, clock, cnt_reg);
       if self.reset.val() {
           self.cnt_reg.d.next = 0.into();
        } else {
        self.cnt_reg.d.next = self.cnt_reg.q.val() + 1;
        }
        // self.cnt.next = self.cnt_reg.q.val();
// ram_setup!(self, clock);
        self.ram.write_clock.next = self.clock.val();
        self.ram.read_clock.next = self.clock.val();
        if self.cnt_reg.q.val() == 15_u8.to_bits() {
            self.ram.write_address.next = 1_u8.to_bits();
            self.ram.write_data.next = 0xaffe1234_u32.to_bits();
            self.ram.write_enable.next = true;
        } else {
            self.ram.write_address.next = 0_u8.to_bits();
            self.ram.write_data.next = 0_u32.to_bits();
            self.ram.write_enable.next = false;
        }

        if self.cnt_reg.q.val() == 19_u8.to_bits() {
            self.ram.read_address.next = 1_u8.to_bits();
        } else {
            self.ram.read_address.next = 0_u8.to_bits();
        }


        self.cnt.next = self.ram.read_data.val();

    }
}

fn main() {
    // v--- build a simple simulation (1 testbench, single clock)
    let mut sim = simple_sim!(Adder, clock, 100_000_000_u64, ep, {
        let mut x = ep.init()?;
        x.reset.next = true.into();
        wait_clock_cycles!(ep, clock, x, 5);
        x.reset.next = false.into();
        wait_clock_cycles!(ep, clock, x, 25);
        ep.done(x)
    });

    // v--- construct the circuit
    let mut uut = Adder::default();
    uut.connect_all();
    println!("{}", generate_verilog(&uut));
    // v--- run the simulation, with the output traced to a .vcd file
    sim.run_to_file(Box::new(uut), 5 * sim_time::ONE_MICROSECOND, "adder.vcd").unwrap();
}

#[test]
fn test_hello_world() {

    let mut sim = simple_sim!(Adder, clock, 100_000_000_u64, ep, {
        let mut x = ep.init()?;
        x.reset.next = true.into();
        wait_clock_cycles!(ep, clock, x, 5);
        x.reset.next = false.into();
        assert_eq!("4","4");
        wait_clock_cycles!(ep, clock, x, 25);
        ep.done(x)
    });
    let uut = Adder::default();
    sim.run_to_file(Box::new(uut), 5 * sim_time::ONE_MICROSECOND, "adder.vcd").unwrap();
}
