library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use ieee.math_real.all;

entity calc is
port(
    clk : in std_logic;
    reset : in std_logic
);
end entity;

architecture rtl of calc is
  constant pi : sfixed(3 downto -32) := to_sfixed(MATH_PI,3,-32);
  constant one_over_pi : sfixed(1 downto -32) := to_sfixed(MATH_1_OVER_PI,1,-32);
  constant sqrt_of_2 : sfixed(2 downto -32) := to_sfixed(MATH_SQRT_2,2,-32);

  signal t_val1 : sfixed (31 downto 0) := to_sfixed(9_487_500_000.0,31,0);
  -- signal t_val1 : sfixed (31 downto 0) := to_sfixed(9_487_500_000.0,31,0);

  -- constant inreal : real := 5.719;
  constant inreal : real := 11116.719;
  constant input : sfixed(31 downto -32) := to_sfixed(inreal,31,-32);
  signal start : real := COS(inreal);

  signal bla : sfixed(1 downto -32);
  signal bla2 : sfixed(33 downto -64);

  signal coss : sfixed(1 downto -32);
  -- signal super1 : sfixed (3 downto -32) := to_sfixed(1.8204142391,3,-32);
  -- signal super2 : sfixed (3 downto -32) := to_sfixed(-1.0+0.8204142391,3,-32);
  -- signal super3 : sfixed (3 downto -32) := to_sfixed(-3.8204142391,3,-32);
  -- signal super4 : sfixed (3 downto -32) := to_sfixed(4.0-3.8204142391,3,-32);
  -- signal super5 : sfixed (3 downto -32) := to_sfixed(-2.111111,3,-32);
  -- signal super6 : sfixed (3 downto -32) := to_sfixed(2.0-2.111111,3,-32);
begin

process(all) begin
  if rising_edge(clk) then

    bla2 <= input * one_over_pi;

    bla(0 downto -32) <= bla2(0 downto -32);
    bla(1) <= bla2(0);

  end if;
end process;

cos_i : entity work.cos_1 port map(
  clk => clk,
  x => bla,
  cos_pi_x => coss
);


end architecture;

