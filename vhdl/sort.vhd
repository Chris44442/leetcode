library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.type_pkg.all;

entity sort is port(
  clk : in std_logic;
  data_in : in u_1D_array(31 downto 0)(31 downto 0);
  result : out unsigned(31 downto 0)
);
end entity;

architecture rtl of sort is
  type int_1D_array is array (natural range <>) of integer;
  type int_2D_array is array (natural range <>, natural range <>) of integer;
  constant RANK : integer range 0 to 31 := 31;
  constant NUMBER_OF_SORTERS : int_1D_array(14 downto 0) := (14,13,11,7,15,13,11,7,15,11,7,15,7,15,15);
  constant REARRANGE_INDEX : int_2D_array(14 downto 0, 31 downto 0) := (
  (31,29,14,28,13,27,12,26,11,25,10,24,9,23,8,22,7,21,6,20,5,19,4,18,3,17,2,16,1,15,0,30),
  (31,28,30,26,12,24,10,22,8,20,6,18,4,16,2,14,0,27,13,25,11,23,9,21,7,19,5,17,3,15,1,29),
  (31,30,25,24,29,28,21,20,9,8,17,16,5,4,13,12,1,0,23,22,11,10,19,18,7,6,15,14,3,2,27,26),
  (31,30,29,28,19,18,17,16,27,26,25,24,11,10,9,8,3,2,1,0,15,14,13,12,7,6,5,4,23,22,21,20),
  (31,30,29,28,27,26,25,24,7,6,5,4,3,2,1,0,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8),
  (31,27,13,26,12,25,11,24,10,23,9,22,8,21,7,30,29,20,6,19,5,18,4,17,3,16,2,15,1,14,0,28),
  (31,28,27,24,30,22,10,20,8,18,6,26,16,4,14,2,12,0,23,11,21,9,19,7,29,17,5,15,3,13,1,25),
  (31,30,25,24,23,22,17,16,29,28,13,12,5,4,21,20,9,8,1,0,15,14,7,6,27,26,11,10,3,2,19,18),
  (31,30,29,28,11,10,9,8,23,22,21,20,3,2,1,0,27,26,25,24,19,18,17,16,15,14,13,12,7,6,5,4),
  (31,23,11,22,10,21,9,30,27,17,5,16,4,15,3,26,29,20,8,19,7,18,6,28,25,14,2,13,1,12,0,24),
  (31,28,27,24,23,20,19,16,30,14,6,26,12,4,22,10,2,18,8,0,15,7,29,13,5,25,11,3,21,9,1,17),
  (31,30,13,12,27,26,9,8,23,22,5,4,19,18,1,0,29,28,25,24,21,20,17,16,15,14,11,10,7,6,3,2),
  (31,15,7,30,27,13,5,26,23,11,3,22,19,9,1,18,29,14,6,28,25,12,4,24,21,10,2,20,17,8,0,16),
  (31,14,29,12,27,10,25,8,23,6,21,4,19,2,17,0,30,28,26,24,22,20,18,16,15,13,11,9,7,5,3,1),
  (31,15,29,13,27,11,25,9,23,7,21,5,19,3,17,1,30,14,28,12,26,10,24,8,22,6,20,4,18,2,16,0)
  );
  type u_2D_array is array (natural range <>, natural range <>) of unsigned;
  signal sort_data : u_2D_array(30 downto 0, 31 downto 0)(31 downto 0);
begin

process(all) begin
  for i in 0 to 31 loop
    sort_data(0,i) <= data_in(i);
    for j in 0 to 14 loop
      if j mod 3 = 2 then
        if rising_edge(clk) then
          sort_data(2*j+2,i) <= sort_data(2*j+1,REARRANGE_INDEX(j,i));
        end if;
      else
        sort_data(2*j+2,i) <= sort_data(2*j+1,REARRANGE_INDEX(j,i));
      end if;
    end loop;
  end loop;
  for j in 0 to 14 loop
    for m in 0 to NUMBER_OF_SORTERS(j) loop
      sort_data(2*j+1,m) <= maximum(sort_data(2*j,m), sort_data(2*j,m+NUMBER_OF_SORTERS(j)+1));
      sort_data(2*j+1,m+NUMBER_OF_SORTERS(j)+1) <= maximum(sort_data(2*j,m), sort_data(2*j,m+NUMBER_OF_SORTERS(j)+1));
    end loop;
    for o in 31 downto (NUMBER_OF_SORTERS(j)+1)*2 loop
      sort_data(2*j+1,o) <= sort_data(2*j,o);
    end loop;
  end loop;
end process;

result <= sort_data(30,RANK);

end architecture;

