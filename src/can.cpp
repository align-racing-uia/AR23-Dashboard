
#include <can.h>

long unsigned int rxId = 0;
unsigned char rxLen = 0;
unsigned char rxBuf[] = {0, 0, 0, 0, 0, 0, 0, 0};
MCP_CAN CAN0(CAN_CS);

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

void canInit(){
    pinMode(CAN_INT, INPUT);
    pinMode(CAN_CS, OUTPUT);

    
    if(CAN0.begin(MCP_STDEXT, CAN_500KBPS, MCP_16MHZ) == CAN_OK){
        Serial.println("MCP2515 Initialized at 500kbps (16MHz)");
    }else{
        Serial.println("MCP2515 Error...");
    }
    // All filters and masks has to be set for it to work properly.
    CAN0.init_Mask(0, 0x03FF0000);
    CAN0.init_Mask(1, 0x06FF0000);
    CAN0.init_Filt(0, 0x00A70000); //+ Owned by mask 0
    CAN0.init_Filt(1, 0x00AA0000); //+
    CAN0.init_Filt(2, 0x06B00000); //- Owned by mask 1
    CAN0.init_Filt(3, 0x00E10000); //-
    CAN0.init_Filt(4, 0x00AB0000); //-
    CAN0.init_Filt(5, 0x00AA0000); //-


    CAN0.setMode(MCP_NORMAL);   // Change to normal mode to allow messages to be transmitted
}