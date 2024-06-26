library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity cos_1 is port(
  clk : in std_logic;
  x : in signed(63 downto 0); -- interpreted as 1 signum, 31 integer and 32 fractional bits
  cos_x : out signed(26 downto 0)); -- interpreted as 1 signum, 1 integer and 25 fractional bits
end entity;

architecture rtl of cos_1 is

-- calculate taylor coefficients with the remez minimax algorithm
-- docker run --rm lolremez -d 3 -r "0:1/4" "cos(pi*x^(1/2))"
-- // Degree 3 approximation of f(x) = cos(pi*x^(1/2))
-- // on interval [ 0, 1/4 ]
-- // p(x)=((-1.2221270623190845*x+4.0412838261529709)*x-4.9339380151432999)*x+9.9999329528216742e-1
-- // Estimated max error: 6.7047178325783373e-6
  signal x_bs : signed(33 downto 0);

-- taylor coefficients
  signal c1 : signed(35 downto 0) := x"EC722AE48";
  signal c2 : signed(35 downto 0) := x"40A9193AB";
  signal c3 : signed(35 downto 0) := x"B10E97030";
  signal c4 : signed(35 downto 0) := 36x"FFFF8F83";
  signal A_HALF : signed(33 downto 0) := 34x"080000000";
  signal ONE : signed(33 downto 0) := 34x"100000000";
  signal MINUS_ONE : signed(33 downto 0) := 34x"300000000";

-- input x to the power of 2 and 4
  signal pow_x_2 : signed(67 downto 0);
  signal pow_x_4, pow_x_4_reg : signed(135 downto 0);
-- intermediate results
  signal med1_tmp : signed(103 downto 0);
  signal med1 : signed(39 downto 0);
  signal med2 : signed(40 downto 0);
  signal med3_tmp : signed(16+128+32 downto 0);
  signal med3 : signed(16+32 downto 0);
  signal med4_tmp : signed(7+96 downto 0);
  signal med4 : signed(7+32 downto 0);
  signal med5, med5_reg : signed(8+32 downto 0);
  signal med6 : signed(17+32 downto 0);

-- range reduction and reconstruction
  signal x_reg1, x_reg2, x_reg3, x_reg4, x_reg5, x_reg6 : signed(33 downto 0);
  signal x_tmp1 : signed(34 downto 0);
  signal x_tmp2 : signed(33 downto 0);
  signal cos_1_tmp2 : signed(33 downto 0);
  signal cos_1_tmp1 : signed(34 downto 0);
begin

med1 <= med1_tmp(7+32+64 downto 0+64);
med3 <= med3_tmp(16+128+32 downto 128);
med4 <= med4_tmp(7+96 downto 64);
x_tmp1 <= "0" & (ONE - x_bs);
cos_1_tmp1 <= "0" & (-med6(33 downto 0));

x_bs(32 downto 0) <= x(32 downto 0);
x_bs(33) <= x(63);

process(clk) begin
  if rising_edge(clk) then

-- range reduction
    if abs(x_bs) > A_HALF then
      x_tmp2 <= x_tmp1(33 downto 0);
    else
      x_tmp2 <= x_bs;
    end if;

-- taylor series
    pow_x_2 <= x_tmp2 * x_tmp2;
    pow_x_4 <= pow_x_2 * pow_x_2;
    pow_x_4_reg <= pow_x_4;
    med1_tmp <= c1 * pow_x_2;
    med2 <= "0" & (c2 + med1);
    med3_tmp <= med2 * pow_x_4_reg;
    med4_tmp <= c3 * pow_x_2;
    med5 <= "0" & (med4 + c4);
    med5_reg <= med5;
    med6 <= "0" & (med5_reg + med3);

-- pipeline registers
    x_reg1 <= x_bs;
    x_reg2 <= x_reg1;
    x_reg3 <= x_reg2;
    x_reg4 <= x_reg3;
    x_reg5 <= x_reg4;
    x_reg6 <= x_reg5;

-- reconstruction
    if x_reg6 = 0 then
      cos_1_tmp2 <= ONE;
    elsif abs(x_reg6) = ONE then
      cos_1_tmp2 <= MINUS_ONE;
    elsif abs(x_reg6) = A_HALF then
      cos_1_tmp2 <= (others => '0');
    elsif abs(x_reg6) > A_HALF then
      cos_1_tmp2 <= cos_1_tmp1(33 downto 0);
    else
      cos_1_tmp2 <= med6(33 downto 0);
    end if;
  end if;
end process;

cos_x <= cos_1_tmp2(33 downto 7);

end architecture;

