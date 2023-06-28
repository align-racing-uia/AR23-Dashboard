#ifndef CAN_H
#define CAN_H
#include <mcp_can.h>
#include <states.h>
#include <inverter.h>
#include <bms.h>

#define CAN_CS 5
#define CAN_INT 6
MCP_CAN CAN0(CAN_CS);

#define CAN_ID 0x0E0

long unsigned int rxId;
unsigned char rxLen = 0;
unsigned char rxBuf[8];


void canRx(){
  
  if(!digitalRead(CAN_INT)){
    CAN0.readMsgBuf(&rxId, &rxLen, rxBuf);
  }else{
    return;
  }
  int oldState = 0;
  switch(rxId){
    case 0x0AA:
      // Handle VSM State
      oldState = vsmState;
      vsmState = rxBuf[0];
      updateBottomStatus = updateBottomStatus || vsmState != oldState;

      oldState = inverterState;
      inverterState = rxBuf[2];

      updateTopStatus = updateTopStatus || oldState != inverterState;
      
      inverterTimestamp = millis();
      break;

    case 0x0A7:
      oldState = dcVoltage;
      dcVoltage = rxBuf[1] << 8;
      dcVoltage |= rxBuf[0];
      dcVoltage /= 10;
      updateMiddleStatus = updateMiddleStatus || dcVoltage != oldState;
      break;

    case 0x6B0:
      oldState = soc;
      soc = rxBuf[4]/2;
      updateMiddleStatus = updateMiddleStatus || soc != oldState;
      break;

    case 0x0AB:
      inverterPostFaultCode[0] = rxBuf[0];
      inverterPostFaultCode[1] = rxBuf[1];
      inverterPostFaultCode[2] = rxBuf[2];
      inverterPostFaultCode[3] = rxBuf[3];
      inverterRunFaultCode[0] = rxBuf[4];
      inverterRunFaultCode[1] = rxBuf[5];
      inverterRunFaultCode[2] = rxBuf[6];
      inverterRunFaultCode[3] = rxBuf[7];

      break;

    case 0x0E1:
      oldState = r2dState;
      r2dState = 0x2 & rxBuf[3];
      updateBottomStatus = updateBottomStatus || oldState != r2dState;

      oldState = sdcState;
      sdcState = 0x4 & rxBuf[3];
      updateBottomStatus = updateBottomStatus || oldState != sdcState;
      appsTimestamp = millis();
      break;
    default:
      Serial.println("Recieved unknown message..");
      return;
  }

  
}

void canTx(){
  byte canFrame[2] = {0x0, 0x0};
  canFrame[0] = btn1;
  canFrame[0] <<= 1;
  canFrame[0] |= btn2;
  canFrame[0] <<= 1;
  canFrame[0] |= btn3;
  canFrame[0] <<= 1;
  canFrame[0] |= btn4;
  canFrame[0] <<= 1;
  canFrame[0] |= r2d;

  // Byte 2 is logged error state.
  canFrame[1] = criticalError;

  CAN0.sendMsgBuf(CAN_ID, 0, 2, canFrame);
  // Serial.println("Frame sent.");
}




#endif