library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity p2r_cordic is generic(
  PIPELINE : integer := 15;
  WIDTH    : integer := 16);
port(
  clk  : in std_logic;
  Xi  : in signed(WIDTH -1 downto 0);
  Yi : in signed(WIDTH -1 downto 0) := (others => '0');
  Zi  : in signed(WIDTH -1 downto 0);
  Xo  : out signed(WIDTH -1 downto 0);
  Yo  : out signed(WIDTH -1 downto 0));
end entity p2r_Cordic;

architecture rtl of p2r_cordic is
  type XYVector is array(PIPELINE downto 0) of signed(WIDTH -1 downto 0);
  type ZVector is array(PIPELINE downto 0) of signed(19 downto 0);
  signal X,Y : XYVector;
  signal Z : ZVector;
begin
  X(0) <= Xi;
  Y(0) <= Yi;
  Z(0)(19 downto 4) <= Zi;
  Z(0)(3 downto 0) <= (others => '0');

  gen_pipe:
  for n in 1 to PIPELINE generate
    Pipe: entity work.p2r_cordic_pipe 
      generic map(WIDTH => WIDTH, PIPEID => n -1)
      port map ( clk, X(n-1), Y(n-1), Z(n-1), X(n), Y(n), Z(n) );
  end generate gen_pipe;

  Xo <= X(PIPELINE);
  Yo <= Y(PIPELINE);
end architecture;

