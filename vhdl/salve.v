module axi4lite_slave (
  input wire clk,
  input wire reset,
  input wire [3:0] AWADDR,
  input wire [2:0] AWPROT,
  input wire AWVALID,
  output wire AWREADY,
  input wire [31:0] WDATA,
  input wire [3:0] WSTRB,
  input wire WVALID,
  output wire WREADY,
  input wire [3:0] ARADDR,
  input wire [2:0] ARPROT,
  input wire ARVALID,
  output wire ARREADY,
  output reg[31:0] RDATA,
  output reg [1:0] RRESP,
  output reg RVALID,
  input wire RREADY,
  output[1:0] BRESP,
  output reg BVALID,
  input wire BREADY
);

  reg [31:0] reg1;

  always @(posedge clk) begin
    BVALID <= 1'b0;
    if (reset)
      reg1 <= 32'h12345678;
    else if (AWVALID) begin
      if (AWADDR == 4'h00) begin
        if (WVALID) begin
          reg1 <= WDATA;
          BVALID <= 1'b1;
        end
      end
    end
    else if (ARVALID && ARREADY) begin
      if (ARADDR == 4'h00) begin
        RDATA <= reg1;
        RRESP <= 2'b00;
        RVALID <= 1'b1;
      end
    end
  end

  assign AWREADY = 1'b1;
  assign WREADY = 1'b1;
  assign BRESP = 2'b00;
  assign ARREADY = 1'b1;

endmodule