

module top(sig_a,sig_b,sig_sum,clock);
    
    // Module arguments
    input wire  [7:0] sig_a;
    input wire  [7:0] sig_b;
    output reg  [7:0] sig_sum;
    input wire  clock;
    
    // Stub signals
    reg  [7:0] my_reg$d;
    wire  [7:0] my_reg$q;
    reg  my_reg$clock;
    
    // Sub module instances
    top$my_reg my_reg(
        .d(my_reg$d),
        .q(my_reg$q),
        .clock(my_reg$clock)
    );
    
    // Update code
    always @(*) begin
        my_reg$clock = clock;
        my_reg$d = my_reg$q;
        my_reg$d = sig_a + sig_b;
        sig_sum = my_reg$q;
    end
    
endmodule // top


module top$my_reg(d,q,clock);
    
    // Module arguments
    input wire  [7:0] d;
    output reg  [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$my_reg

