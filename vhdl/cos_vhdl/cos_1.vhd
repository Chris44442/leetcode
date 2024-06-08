library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity cos_1 is
port(
    clk : in std_logic;
    reset : in std_logic;
    x : in sfixed(1 downto -32);
    cos_x : out sfixed(1 downto -32)
);
end entity;

architecture rtl of cos_1 is

-- calculate taylor coefficients with the remez minimax algorithm
-- docker run --rm lolremez -d 3 -r "0:1/4" "cos(pi*x^(1/2))"
-- // Degree 3 approximation of f(x) = cos(pi*x^(1/2))
-- // on interval [ 0, 1/4 ]
-- // p(x)=((-1.2221270623190845*x+4.0412838261529709)*x-4.9339380151432999)*x+9.9999329528216742e-1
-- // Estimated max error: 6.7047178325783373e-6

  constant FRACS : integer := -32;
-- taylor coefficients
  constant c1 : sfixed(3 downto -32) := to_sfixed(-1.2221270623190845,3,-32);
  constant c2 : sfixed(3 downto -32) := to_sfixed(4.0412838261529709,3,-32);
  constant c3 : sfixed(3 downto -32) := to_sfixed(-4.9339380151433003,3,-32);
  constant c4 : sfixed(3 downto -32) := to_sfixed(0.9999932952821674,3,-32);
  constant A_HALF : sfixed(1 downto FRACS) := to_sfixed(0.5,1,FRACS);
-- input x to the power of 2 and 4
  signal pow_x_2 : sfixed(3 downto -64);
  signal pow_x_4, pow_x_4_reg : sfixed(7 downto -128);
-- intermediate results
  signal med1_tmp : sfixed(7 downto -96);
  signal med1 : sfixed(7 downto FRACS);
  signal med2 : sfixed(8 downto FRACS);
  signal med3_tmp : sfixed(16 downto FRACS-128);
  signal med3 : sfixed(16 downto FRACS);
  signal med4_tmp : sfixed(7 downto -96);
  signal med4 : sfixed(7 downto FRACS);
  signal med5, med5_reg : sfixed(8 downto FRACS);
  signal med6 : sfixed(17 downto FRACS);

-- range reduction and reconstruction
  signal x_reg1, x_reg2, x_reg3, x_reg4, x_reg5, x_reg6 : sfixed(1 downto -32);
  signal x_tmp1 : sfixed(2 downto -32);
  signal x_tmp2 : sfixed(1 downto -32);
  signal cos_1_tmp2 : sfixed(1 downto FRACS);
  signal cos_1_tmp1 : sfixed(2 downto FRACS);
begin

med1 <= med1_tmp(7 downto FRACS);
med3 <= med3_tmp(16 downto FRACS);
med4 <= med4_tmp(7 downto FRACS);
x_tmp1 <= 1 - x;
cos_1_tmp1 <= -med6(1 downto FRACS);

process(all) begin
  if rising_edge(clk) then

-- range reduction
    if abs(x) > A_HALF then
      x_tmp2 <= x_tmp1(1 downto -32);
    else
      x_tmp2 <= x;
    end if;

-- taylor series
    pow_x_2 <= x_tmp2 * x_tmp2;
    pow_x_4 <= pow_x_2 * pow_x_2;
    pow_x_4_reg <= pow_x_4;
    med1_tmp <= c1 * pow_x_2;
    med2 <= c2 + med1;
    med3_tmp <= med2 * pow_x_4_reg;
    med4_tmp <= c3 * pow_x_2;
    med5 <= med4 + c4;
    med5_reg <= med5;
    med6 <= med5_reg + med3;

-- pipeline registers
    x_reg1 <= x;
    x_reg2 <= x_reg1;
    x_reg3 <= x_reg2;
    x_reg4 <= x_reg3;
    x_reg5 <= x_reg4;
    x_reg6 <= x_reg5;
-- reconstruction
    if x_reg6 = to_sfixed(0,1,-32) then
      cos_1_tmp2 <= to_sfixed(1,1,-32);
    elsif x_reg6 = to_sfixed(1,1,-32) then
      cos_1_tmp2 <= to_sfixed(-1,1,-32);
    elsif x_reg6 = to_sfixed(-1,1,-32) then
      cos_1_tmp2 <= to_sfixed(-1,1,-32);
    elsif abs(x_reg6) > A_HALF then
      cos_1_tmp2 <= cos_1_tmp1(1 downto FRACS);
    else
      cos_1_tmp2 <= med6(1 downto FRACS);
    end if;
  end if;
end process;

cos_x <= cos_1_tmp2(1 downto -32);

end architecture;

