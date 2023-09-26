use std::fs::File;
use std::io::Read;

use std::collections::BTreeMap;

#[derive(Debug, serde_derive::Deserialize)]
struct Register {
    width: u32,
    offset: u32,
}

#[derive(Debug, serde_derive::Deserialize)]
struct Config {
    register: BTreeMap<String, Register>,
}

fn main() {
    let mut toml_file = File::open("regmap.toml").expect(" File::open regmap.toml failed");
    let mut toml_str = String::new();
    toml_file.read_to_string(&mut toml_str).expect("toml_file.read_to_string failed");

    let config: Config = toml::from_str(&toml_str).expect("Failed to deserialize TOML");

    for (key, reg) in &config.register {
        // println!("Key: {}", key);
        // println!("width: {}", reg.width);
        // println!("offset: 0x{:08X}", reg.offset);

        println!("  {}_strobe : out std_logic;", key);
        println!("  {}_value  : in std_logic_vector({} downto 0);", key, reg.width-1);
    }

let axi_addr_width:u32 = 5;


println!("library ieee;");
println!("use ieee.std_logic_1164.all;");
println!("use ieee.numeric_std.all;");
println!("");
println!("entity regmap_regs is generic (");
println!("  AXI_ADDR_WIDTH : integer := {};", axi_addr_width); // TODO
println!("  BASEADDR : std_logic_vector(31 downto 0) := x\"00000000\"");
println!("  );");
println!("  port (");
println!("  clk           : in  std_logic;");
println!("  reset         : in  std_logic;");
println!("  s_axi_awaddr  : in  std_logic_vector(AXI_ADDR_WIDTH - 1 downto 0);");
println!("  s_axi_awprot  : in  std_logic_vector(2 downto 0);");
println!("  s_axi_awvalid : in  std_logic;");
println!("  s_axi_awready : out std_logic;");
println!("  s_axi_wdata   : in  std_logic_vector(31 downto 0);");
println!("  s_axi_wstrb   : in  std_logic_vector(3 downto 0);");
println!("  s_axi_wvalid  : in  std_logic;");
println!("  s_axi_wready  : out std_logic;");
println!("  s_axi_araddr  : in  std_logic_vector(AXI_ADDR_WIDTH - 1 downto 0);");
println!("  s_axi_arprot  : in  std_logic_vector(2 downto 0);");
println!("  s_axi_arvalid : in  std_logic;");
println!("  s_axi_arready : out std_logic;");
println!("  s_axi_rdata   : out std_logic_vector(31 downto 0);");
println!("  s_axi_rresp   : out std_logic_vector(1 downto 0);");
println!("  s_axi_rvalid  : out std_logic;");
println!("  s_axi_rready  : in  std_logic;");
println!("  s_axi_bresp   : out std_logic_vector(1 downto 0);");
println!("  s_axi_bvalid  : out std_logic;");
println!("  s_axi_bready  : in  std_logic;");

for (key, reg) in &config.register {
    println!("  {}_strobe : out std_logic;", key);
    println!("  {}_value  : in std_logic_vector({} downto 0);", key, reg.width-1);
}

println!("end entity regmap_regs;");
println!("");
println!("architecture RTL of regmap_regs is");

for (key, reg) in &config.register {
    println!("  constant {}_OFFSET      : unsigned(31 downto 0) := unsigned'(x\"{:08X}\");", key, reg.offset);
    println!("  constant {}_VALUE_RESET : std_logic_vector(31 downto 0) := std_logic_vector'(\"00000000000000000000000000000000\");", key);
}

}