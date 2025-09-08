# Overview

In the world of computer engineering, we use Verilog to map out the layout of an ultra-small circuit such as a CPU or an Integrated Circuit. Verilog is formally categorized as a Hardware Description Language, which is like a programming language with the crucial difference that it defines how hardware functions, not software. 

Verilog is notorious for being long-winded to write and dangerous to verify. In order to reduce the mental strain on the developer, I created Verimacro, which is **a macro expander for Verilog**, allowing us to write a lot of code using much shorter and simpler syntax.

# Features

Instead of the clunky `begin` and `end` syntax, I opted to replace block definitions with the more common `{` and `}`. This makes much code much smaller to write. Many common keywords have been shortened, so repeated instances of `input` and `output` will be reduced to `i` and `o`. Though these changes may seem minimal, the effect on the syntax of the written program is immense. To illustrate this, I would like to demonstrate the following code:

Source code:
```verilog
mod t (i l clk, i l [15] w_data, i l [2] w_addr, i l w_en, i l [2] r_addr, o l [15] r_data) {
	l [15] m[0:7];
	-> r_data = m[r_addr];
	ff (posedge clk) {
		if (w_en) { 
			m[w_addr] <= w_data;
		}
	}

	comb {case (a) {2'b11: a=0; 2'b10: a=2; 2'b00: a=1; 2'b01: a=3; def>> a=2;
	} }
}
```

Which generates:
```verilog
module t (input logic clk, input logic [15:0] w_data, input logic [2:0] w_addr, input logic w_en, input logic [2:0] r_addr, output logic [15:0] r_data) ;
	logic [15:0] m[0:7];
	assign r_data = m[r_addr];
	always_ff(posedge clk) begin
		if (w_en) begin 
			m[w_addr] <= w_data;
		end
	end

	

	always_comb begin case (a) 2'b11: a=0; 2'b10: a=2; 2'b00: a=1; 2'b01: a=3; default: a=2;
	endcase end
endmodule
```

With complete control over the way the code works, Verimacro allows the programmer to style any way they want. With this tool, the user will find their productivity would suddenly increase greatly!