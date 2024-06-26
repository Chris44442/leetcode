library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity cos_1 is port(
  clk : in std_logic;
  x : in sfixed(1 downto -32);
  cos_x : out sfixed(1 downto -32));
end entity;

architecture rtl of cos_1 is

-- calculate taylor coefficients with the remez minimax algorithm
-- docker run --rm lolremez -d 3 -r "0:1/4" "cos(pi*x^(1/2))"
-- // Degree 3 approximation of f(x) = cos(pi*x^(1/2))
-- // on interval [ 0, 1/4 ]
-- // p(x)=((-1.2221270623190845*x+4.0412838261529709)*x-4.9339380151432999)*x+9.9999329528216742e-1
-- // Estimated max error: 6.7047178325783373e-6
  signal x_fake : signed(33 downto 0) := (others => '0');
  signal cos_x_fake : signed(33 downto 0);

  constant FRACS : integer := -32;
-- taylor coefficients
  signal c1 : sfixed(3 downto -32) := to_sfixed(-1.2221270623190845,3,-32);
  signal c1_fake : signed(35 downto 0) := x"EC722AE48";
  signal c2 : sfixed(3 downto -32) := to_sfixed(4.0412838261529709,3,-32);
  signal c2_fake : signed(35 downto 0) := x"40A9193AB";
  signal c3 : sfixed(3 downto -32) := to_sfixed(-4.9339380151433003,3,-32);
  signal c3_fake : signed(35 downto 0) := x"B10E97030";
  signal c4 : sfixed(3 downto -32) := to_sfixed(0.9999932952821674,3,-32);
  signal c4_fake : signed(35 downto 0) := 36x"FFFF8F83";
  signal A_HALF : sfixed(1 downto FRACS) := to_sfixed(0.5,1,FRACS);
  signal A_HALF_fake : signed(33 downto 0) := 34x"080000000";
  signal ONE : signed(33 downto 0) := 34x"100000000";

  signal d1 : sfixed(1 downto -32) := to_sfixed(1,1,-32);
  signal d2 : sfixed(1 downto -32) := to_sfixed(-1,1,-32);

-- input x to the power of 2 and 4
  signal pow_x_2 : sfixed(3 downto -64);
  signal pow_x_2_fake : signed(67 downto 0);
  signal pow_x_4, pow_x_4_reg : sfixed(7 downto -128);
  signal pow_x_4_fake, pow_x_4_reg_fake : signed(135 downto 0);
-- intermediate results
  signal med1_tmp : sfixed(7 downto -96);
  signal med1_tmp_fake : signed(103 downto 0);
  signal med1 : sfixed(7 downto FRACS);
  signal med1_fake : signed(39 downto 0);
  signal med2 : sfixed(8 downto FRACS);
  signal med2_fake : signed(40 downto 0);
  signal med3_tmp : sfixed(16 downto FRACS-128);
  signal med3_tmp_fake : signed(16+128+32 downto 0);
  signal med3 : sfixed(16 downto FRACS);
  signal med3_fake : signed(16+32 downto 0);
  signal med4_tmp : sfixed(7 downto -96);
  signal med4_tmp_fake : signed(7+96 downto 0);
  signal med4 : sfixed(7 downto FRACS);
  signal med4_fake : signed(7+32 downto 0);
  signal med5, med5_reg : sfixed(8 downto FRACS);
  signal med5_fake, med5_reg_fake : signed(8+32 downto 0);
  signal med6 : sfixed(17 downto FRACS);
  signal med6_fake : signed(17+32 downto 0);

-- range reduction and reconstruction
  signal x_reg1, x_reg2, x_reg3, x_reg4, x_reg5, x_reg6 : sfixed(1 downto -32);
  signal x_reg1_fake, x_reg2_fake, x_reg3_fake, x_reg4_fake, x_reg5_fake, x_reg6_fake : signed(33 downto 0);
  signal x_tmp1 : sfixed(2 downto -32);
  signal x_tmp1_fake : signed(34 downto 0);
  signal x_tmp2 : sfixed(1 downto -32);
  signal x_tmp2_fake : signed(33 downto 0);
  signal cos_1_tmp2 : sfixed(1 downto FRACS);
  signal cos_1_tmp2_fake : signed(33 downto 0);
  signal cos_1_tmp1 : sfixed(2 downto FRACS);
  signal cos_1_tmp1_fake : signed(34 downto 0);
  
  signal bla : sfixed(33 downto 0);
begin

med1 <= med1_tmp(7 downto FRACS);
med1_fake <= med1_tmp_fake(7+32+64 downto 0+64);
med3 <= med3_tmp(16 downto FRACS);
med3_fake <= med3_tmp_fake(16+128+32 downto 128);
med4 <= med4_tmp(7 downto FRACS);
med4_fake <= med4_tmp_fake(7+96 downto 64);
x_tmp1 <= 1 - x;
x_tmp1_fake <= "0" & (ONE - x_fake);
cos_1_tmp1 <= -med6(1 downto FRACS);
cos_1_tmp1_fake <= "0" & (-med6_fake(33 downto 0));


bla <= (x(1 downto -32));
x_fake(33 downto 0) <= signed(bla);

process(clk) begin
  if rising_edge(clk) then

-- range reduction
    if abs(x) > A_HALF then
      x_tmp2 <= x_tmp1(1 downto -32);
    else
      x_tmp2 <= x;
    end if;
    if abs(x_fake) > A_HALF_fake then
      x_tmp2_fake <= x_tmp1_fake(33 downto 0);
    else
      x_tmp2_fake <= x_fake;
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

    pow_x_2_fake <= x_tmp2_fake * x_tmp2_fake;
    pow_x_4_fake <= pow_x_2_fake * pow_x_2_fake;
    pow_x_4_reg_fake <= pow_x_4_fake;
    med1_tmp_fake <= c1_fake * pow_x_2_fake;
    med2_fake <= "0" & (c2_fake + med1_fake);
    med3_tmp_fake <= med2_fake * pow_x_4_reg_fake;
    med4_tmp_fake <= c3_fake * pow_x_2_fake;
    med5_fake <= "0" & (med4_fake + c4_fake);
    med5_reg_fake <= med5_fake;
    med6_fake <= "0" & (med5_reg_fake + med3_fake);

-- pipeline registers
    x_reg1 <= x;
    x_reg2 <= x_reg1;
    x_reg3 <= x_reg2;
    x_reg4 <= x_reg3;
    x_reg5 <= x_reg4;
    x_reg6 <= x_reg5;

    x_reg1_fake <= x_fake;
    x_reg2_fake <= x_reg1_fake;
    x_reg3_fake <= x_reg2_fake;
    x_reg4_fake <= x_reg3_fake;
    x_reg5_fake <= x_reg4_fake;
    x_reg6_fake <= x_reg5_fake;

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

    if x_reg6_fake = 0 then
      cos_1_tmp2_fake <= 34x"10000000";
    elsif x_reg6_fake = 34x"10000000" then
      cos_1_tmp2_fake <= 34x"30000000";
    elsif x_reg6_fake = 34x"30000000" then
      cos_1_tmp2_fake <= 34x"30000000";
    elsif abs(x_reg6_fake) > A_HALF_fake then
      cos_1_tmp2_fake <= cos_1_tmp1_fake(33 downto 0);
    else
      cos_1_tmp2_fake <= med6_fake(33 downto 0);
    end if;
  end if;
end process;

cos_x <= cos_1_tmp2(1 downto -32);
cos_x_fake <= cos_1_tmp2_fake(33 downto 0);

end architecture;

