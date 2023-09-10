use rust_hdl::prelude::*;   // <- shorthand to bring in all definitions

//        v--- Required by RustHDL
#[derive(LogicBlock, Default)]
struct MyAdder {
    pub sig_a: Signal<In, Bits<8>>,
    pub sig_b: Signal<In, Bits<8>>,
    pub sig_sum: Signal<Out, Bits<8>>,
    pub clock: Signal<In, Clock>,
    my_reg: DFF<Bits<8>>,
}

impl Logic for MyAdder {
  #[hdl_gen]  // <--- don't forget this
  fn update(&mut self) {
       // Setup the DFF.. this macro is handy to prevent latches
       dff_setup!(self, clock, my_reg);
       self.my_reg.d.next = self.sig_a.val() + self.sig_b.val();
       self.sig_sum.next = self.my_reg.q.val();
   }
}

fn main() {

  let mut uut = MyAdder::default();
  uut.connect_all();
  println!("{}", generate_verilog(&uut));
}