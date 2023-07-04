#ifndef CAN_H_
#define CAN_H_
#include <mcp_can.h>
#include <states.h>
#include <inverter.h>
#include <bms.h>

#define CAN_CS 5
#define CAN_MOSI 3
#define CAN_MISO 4
#define CAN_SCK 2
#define CAN_INT 6
extern MCP_CAN CAN0;

#define CAN_ID 0x0E0

extern long unsigned int rxId;
extern unsigned char rxLen;
extern unsigned char rxBuf[8];

void canUpdate();

void canRx();

void canTx();

void canInit();


#endif