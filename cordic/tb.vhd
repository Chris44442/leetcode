library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use ieee.math_real.all;
library vunit_lib;
context vunit_lib.vunit_context;

entity cordic_tb is
  generic (runner_cfg : string);
end entity;

architecture sim of cordic_tb is
  constant PERIOD : time := 10 ns;
  -- constant NO_RUNS : natural := 20;
  -- constant MAX_ALLOWED_ERROR : real := 0.00000671;
  signal clk : std_logic := '0';
  -- signal cos_result : sfixed(1 downto -32);
  -- signal x : sfixed(1 downto -32);
  -- signal xreal : real;
  -- signal result_actual, result_expected : real;
  -- type real_array is array (natural range <>) of real;
  -- signal input_array : real_array(NO_RUNS-1 downto 0);
  -- signal ain : signed(15 downto 0) := (others => '0');
  signal ain : signed(15 downto 0) := 16x"1555";
  signal sinn : signed(15 downto 0);
  signal coss : signed(15 downto 0);


  constant lala : sfixed(0 downto -16) := to_sfixed(-0.335,0,-16);
  signal lala_real : real;
begin

clk <= not clk after PERIOD/2;
lala_real <= to_real(lala);
-- x <= to_sfixed(xreal,1,-32);
-- result_actual <= to_real(cos_result);

process
  -- variable seed1 : positive := 18;
  -- variable seed2 : positive := 77;
  -- variable rand_val : real := 0.0;
begin
  test_runner_setup(runner, runner_cfg);
  wait for 1*PERIOD;
  -- wait for 1*PERIOD;
  -- wait until falling_edge(clk);
  -- for i in 0 to NO_RUNS-1 loop
  --   UNIFORM(seed1,seed2,rand_val);
  --   input_array(i) <= rand_val*2.0-1.0;
  --   xreal <= rand_val*2.0-1.0;
    -- input_array(i) <= rand_val;
    -- xreal <= rand_val;
  --   wait for PERIOD;
  -- end loop;
  wait for 16*PERIOD;
  wait for 16*PERIOD;
  wait for 16*PERIOD;
  test_runner_cleanup(runner);
end process;

process begin
  -- wait for 9*PERIOD;
  -- for i in 0 to NO_RUNS-1 loop
  --   result_expected <= COS(MATH_PI*input_array(i));
  --   check_equal(result_actual, COS(MATH_PI*input_array(i)), max_diff => MAX_ALLOWED_ERROR);
    -- check_equal(result_actual, result_expected, max_diff => MAX_ALLOWED_ERROR);
  --   wait for PERIOD;
  -- end loop;
  wait;
end process;

cordic: entity work.sc_corproc port map (
  clk => clk,
  Ain => ain,
  sin => sinn,
  cos => coss
);

end architecture;

