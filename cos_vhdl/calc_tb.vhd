library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use ieee.math_real.all;
library vunit_lib;
context vunit_lib.vunit_context;

entity calc_tb is
  generic (runner_cfg : string);
end entity;

architecture sim of calc_tb is
  constant PERIOD : time := 10 ns;
  -- constant NO_RUNS : natural := 20;
  -- constant MAX_ALLOWED_ERROR : real := 0.00000671;
  signal clk : std_logic := '0';
  signal reset : std_logic := '1';
  -- signal cos_result : sfixed(1 downto -32);
  -- signal x : sfixed(1 downto -32);
  -- signal xreal : real;
  -- signal result_actual, result_expected : real;
  -- type real_array is array (natural range <>) of real;
  -- signal input_array : real_array(NO_RUNS-1 downto 0);
begin

clk <= not clk after PERIOD/2;
-- x <= to_sfixed(xreal,1,-32);
-- result_actual <= to_real(cos_result);

process
  -- variable seed1 : positive := 18;
  -- variable seed2 : positive := 77;
  -- variable rand_val : real := 0.0;
begin
  test_runner_setup(runner, runner_cfg);
  wait for 1*PERIOD;
  reset <= '0';
  -- wait for 1*PERIOD;
  -- wait until falling_edge(clk);
  -- for i in 0 to NO_RUNS-1 loop
  --   UNIFORM(seed1,seed2,rand_val);
  --   input_array(i) <= rand_val*2.0-1.0;
  --   xreal <= rand_val*2.0-1.0;
  --   -- input_array(i) <= rand_val;
  --   -- xreal <= rand_val;
  --   wait for PERIOD;
  -- end loop;
  wait for 16*PERIOD;
  test_runner_cleanup(runner);
end process;

-- process begin
  -- wait for 9*PERIOD;
  -- for i in 0 to NO_RUNS-1 loop
  --   result_expected <= COS(MATH_PI*input_array(i));
  --   check_equal(result_actual, COS(MATH_PI*input_array(i)), max_diff => MAX_ALLOWED_ERROR);
    -- check_equal(result_actual, result_expected, max_diff => MAX_ALLOWED_ERROR);
  --   wait for PERIOD;
  -- end loop;
--   wait;
-- end process;

cos_1: entity work.calc port map (
  clk => clk,
  reset => reset
);

end architecture;

