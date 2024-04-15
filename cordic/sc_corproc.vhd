library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity sc_corproc is port(
  clk  : in std_logic;
  Ain  : in signed(15 downto 0);
  sin  : out signed(15 downto 0);
  cos  : out signed(15 downto 0));
end entity sc_corproc;

architecture rtl of sc_corproc is
  constant PipeLength : natural := 15;
  constant P : signed(15 downto 0) := x"4dba";  -- define aggregate constant

begin
cordic: entity work.p2r_cordic generic map(
  PIPELINE => PipeLength,
  WIDTH => 16)
port map(
  clk => clk,
  Xi => P,
  Zi => Ain,
  Xo => cos,
  Yo => sin);
end architecture;

