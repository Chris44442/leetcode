library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity p2r_CordicPipe is generic(
  WIDTH : natural := 16;
  PIPEID : natural := 1);
port(
  clk : in std_logic;
  Xi : in signed(WIDTH -1 downto 0); 
  Yi : in signed(WIDTH -1 downto 0);
  Zi : in signed(19 downto 0);
  Xo : out signed(WIDTH -1 downto 0);
  Yo : out signed(WIDTH -1 downto 0);
  Zo : out signed(19 downto 0));
end entity p2r_CordicPipe;

architecture dataflow of p2r_CordicPipe is

function CATAN(n :natural) return integer is
variable result  :integer;
begin
  case n is
    when 0 => result := 16#020000#;
    when 1 => result := 16#012E40#;
    when 2 => result := 16#09FB4#;
    when 3 => result := 16#05111#;
    when 4 => result := 16#028B1#;
    when 5 => result := 16#0145D#;
    when 6 => result := 16#0A2F#;
    when 7 => result := 16#0518#;
    when 8 => result := 16#028C#;
    when 9 => result := 16#0146#;
    when 10 => result := 16#0A3#;
    when 11 => result := 16#051#;
    when 12 => result := 16#029#;
    when 13 => result := 16#014#;
    when 14 => result := 16#0A#;
    when 15 => result := 16#05#;
    when 16 => result := 16#03#;
    when 17 => result := 16#01#;
    when others => result := 16#0#;
  end case;
  return result;
end;

  function AddSub(dataa, datab : in signed; add_sub : in std_logic) return signed is
  begin
    if (add_sub = '1') then
      return dataa + datab;
    else
      return dataa - datab;
    end if;
  end;

  signal dX, Xresult  : signed(WIDTH -1 downto 0);
  signal dY, Yresult  : signed(WIDTH -1 downto 0);
  signal atan, Zresult  : signed(19 downto 0);
  signal Zneg, Zpos  : std_logic;
  
begin

dX <= shift_right(Xi, PIPEID);
dY <= shift_right(Yi, PIPEID);
atan <= to_signed( catan(PIPEID), 20);
Zneg <= Zi(19);
Zpos <= not Zi(19);
Xresult <= AddSub(Xi, dY, Zneg);
Yresult <= AddSub(Yi, dX, Zpos);
Zresult <= AddSub(Zi, atan, Zneg);

gen_regs: process(all) begin
  if rising_edge(clk) then
    Xo <= Xresult;
    Yo <= Yresult;
    Zo <= Zresult;
  end if;
end process;

end architecture;

-- Function CATAN (constante arc-tangent).
-- This is a lookup table containing pre-calculated arc-tangents.
-- 'n' is the number of the pipe, returned is a 20bit arc-tangent value.
-- The numbers are calculated as follows: Z(n) = atan(1/2^n)
-- examples:
-- 20bit values => 2^20 = 2pi(rad)
--                 1(rad) = 2^20/2pi = 166886.053....
-- n:0, atan(1/1) = 0.7853...(rad)
--      0.7853... * 166886.053... = 131072(dec) = 20000(hex)
-- n:1, atan(1/2) = 0.4636...(rad)
--      0.4636... * 166886.053... = 77376.32(dec) = 12E40(hex)
-- n:2, atan(1/4) = 0.2449...(rad)
--      0.2449... * 166886.053... = 40883.52(dec) = 9FB3(hex)
-- n:3, atan(1/8) = 0.1243...(rad)
--      0.1243... * 166886.053... = 20753.11(dec) = 5111(hex)
