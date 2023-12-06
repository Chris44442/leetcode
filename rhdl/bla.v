

module top(clock,reset,cnt);
    
    // Module arguments
    input wire  clock;
    input wire  reset;
    output reg  [31:0] cnt;
    
    // Stub signals
    reg  [7:0] cnt_reg$d;
    wire  [7:0] cnt_reg$q;
    reg  cnt_reg$clock;
    reg  [7:0] ram$read_address;
    reg  ram$read_clock;
    wire  [31:0] ram$read_data;
    reg  [7:0] ram$write_address;
    reg  ram$write_clock;
    reg  [31:0] ram$write_data;
    reg  ram$write_enable;
    
    // Sub module instances
    top$cnt_reg cnt_reg(
        .d(cnt_reg$d),
        .q(cnt_reg$q),
        .clock(cnt_reg$clock)
    );
    top$ram ram(
        .read_address(ram$read_address),
        .read_clock(ram$read_clock),
        .read_data(ram$read_data),
        .write_address(ram$write_address),
        .write_clock(ram$write_clock),
        .write_data(ram$write_data),
        .write_enable(ram$write_enable)
    );
    
    // Update code
    always @(*) begin
        cnt_reg$clock = clock;
        cnt_reg$d = cnt_reg$q;
        if (reset) begin
            cnt_reg$d = 32'h0;
        end
        else begin
            cnt_reg$d = cnt_reg$q + 32'h1;
        end
        ram$write_clock = clock;
        ram$read_clock = clock;
        if (cnt_reg$q == 8'hf) begin
            ram$write_address = 8'h1;
            ram$write_data = 32'haffe1234;
            ram$write_enable = 1'b1;
        end
        else begin
            ram$write_address = 8'h0;
            ram$write_data = 32'h0;
            ram$write_enable = 1'b0;
        end
        if (cnt_reg$q == 8'h13) begin
            ram$read_address = 8'h1;
        end
        else begin
            ram$read_address = 8'h0;
        end
        cnt = ram$read_data;
    end
    
endmodule // top


module top$cnt_reg(d,q,clock);
    
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
          
endmodule // top$cnt_reg


module top$ram(read_address,read_clock,read_data,write_address,write_clock,write_data,write_enable);
    
    // Module arguments
    input wire  [7:0] read_address;
    input wire  read_clock;
    output reg  [31:0] read_data;
    input wire  [7:0] write_address;
    input wire  write_clock;
    input wire  [31:0] write_data;
    input wire  write_enable;
    
    // Update code (custom)
    reg[31:0] mem[255:0];
    
    
    
    always @(posedge read_clock) begin
       read_data <= mem[read_address];
    end
    
    always @(posedge write_clock) begin
       if (write_enable) begin
          mem[write_address] <= write_data;
       end
    end
                
endmodule // top$ram

