#include <mcp_can.h>

#define CAN_CS 5
#define CAN_INT 6
MCP_CAN CAN0(CAN_CS);

#define CAN_ID 0x0E0

long unsigned int rxId;
unsigned char rxLen = 0;
unsigned char rxBuf[8];