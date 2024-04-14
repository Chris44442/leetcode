library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use ieee.math_real.all;
library vunit_lib;
context vunit_lib.vunit_context;

entity cos_1_tb is
  generic (runner_cfg : string);
end entity;

architecture sim of cos_1_tb is
  constant PERIOD : time := 10 ns;
  constant NO_RUNS : natural := 10000;
  constant MAX_ALLOWED_ERROR : real := 0.00000671;
  signal clk : std_logic := '0';
  signal reset : std_logic := '1';
  signal cos_result : sfixed(1 downto -32);
  signal x : sfixed(1 downto -32);
  signal xreal : real := 0.0;
  signal cos_real : real := 0.0;
  type real_array is array (natural range <>) of real;
  signal input_array : real_array(NO_RUNS-1 downto 0);
begin

clk <= not clk after PERIOD/2;
x <= to_sfixed(xreal,1,-32);
cos_real <= to_real(cos_result);

process
  variable seed1 : positive := 18;
  variable seed2 : positive := 77;
  variable bla : real;
begin
  test_runner_setup(runner, runner_cfg);
  reset <= '0';
  wait for 1*PERIOD;
  wait until falling_edge(clk);

  for i in 0 to NO_RUNS-1 loop
    UNIFORM(seed1,seed2,bla);
    input_array(i) <= bla;
    xreal <= bla;
    wait for PERIOD;
  end loop;

  wait for 16*PERIOD;
  test_runner_cleanup(runner);
end process;

process begin
  wait for 9*PERIOD;
  for i in 0 to NO_RUNS-1 loop
    check_equal(cos_real, COS(MATH_PI*input_array(i)), max_diff => MAX_ALLOWED_ERROR);
    wait for PERIOD;
  end loop;
  wait;
end process;

cos_1: entity work.cos_1 port map (
  clk => clk,
  x => x,
  cos_pi_x => cos_result
);

end architecture;

