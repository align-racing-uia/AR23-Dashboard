
#include <can.h>
#include <core1.h>

long unsigned int rxId = 0;
unsigned char rxLen = 0;
unsigned char rxBuf[] = {0, 0, 0, 0, 0, 0, 0, 0};
MCP_CAN CAN0(CAN_CS);

void canUpdate(){
  if(!readyForData) return;
  sharedDcVoltage = dcVoltage;
  sharedInverterState = inverterState;
  sharedCommandedTorque = commandedTorque;
  sharedSoc = soc;
  sharedR2DState = r2dState;
  sharedSdcState = sdcState;
  sharedVsmState = vsmState;
  sharedDeviationState = deviationState;
  sharedPackTemp = packTemp;
  sharedPackPower = packPower;
  readyForData = false;
  sharedLowVoltageState = lowVoltageState;
  sharedUpdateBottomStatus = updateBottomStatus;
  sharedUpdateMiddleStatus = updateMiddleStatus;
  sharedUpdateTopStatus = updateTopStatus;
  sharedUpdateDriverStatus = updateDriverStatus;
  for(int i=0; i<4; i++){
    sharedInverterPostFaultCode[i] = inverterPostFaultCode[i];
    sharedInverterRunFaultCode[i] = inverterRunFaultCode[i];
  }
  updateBottomStatus = false;
  updateMiddleStatus = false;
  updateTopStatus = false;
}

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

    case 0x0A9:
      oldState = lowVoltageState;
      lowVoltageState = rxBuf[7] << 8;
      lowVoltageState |= rxBuf[6];
      updateMiddleStatus = updateMiddleStatus || lowVoltageState != oldState;

      break;

    case 0x0AC:
      
      oldState = packPower;
      packPower = rxBuf[3] << 8;
      packPower |= rxBuf[2];
      packPower /= 10;
      // updateBottomStatus = updateBottomStatus || packPower != oldState;

      break;

    case 0x6B0:
      oldState = soc;
      soc = rxBuf[4]/2;
      updateMiddleStatus = updateMiddleStatus || soc != oldState;

      oldState = dcVoltage;
      dcVoltage = rxBuf[2] << 8;
      dcVoltage |= rxBuf[3];
      dcVoltage /= 10;
      updateMiddleStatus = updateMiddleStatus || dcVoltage != oldState;

      bmsTimestamp = millis();
      
      break;
    
    case 0x6B1:
      oldState = packTemp;
      packTemp = rxBuf[4];
      updateMiddleStatus = updateMiddleStatus || packTemp != oldState;

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
      oldState = deviationState;
      deviationState = 0x1 & rxBuf[3];
      // Handled in faults.cpp
      //updateBottomStatus = updateBottomStatus || oldState != deviationState;

      oldState = r2dState;
      r2dState = 0x2 & rxBuf[3];
      updateBottomStatus = updateBottomStatus || oldState != r2dState;

      oldState = sdcState;
      sdcState = 0x8 & rxBuf[3];
      updateBottomStatus = updateBottomStatus || oldState != sdcState;
      appsTimestamp = millis();
      break;

    case 0x0C0:
      oldState = commandedTorque;
      commandedTorque = rxBuf[1] << 8;
      commandedTorque |= rxBuf[0];
      updateDriverStatus = updateDriverStatus || oldState != commandedTorque;
      updateBottomStatus = updateDriverStatus || updateBottomStatus;
      break;

    default:
      return;
  }

  
}

void canTx(){
  byte canFrame[2] = {0x0, 0x0};
  canFrame[0] = btn2;
  canFrame[0] <<= 1;
  canFrame[0] |= btn1;
  canFrame[0] <<= 1;
  canFrame[0] |= btn4;
  canFrame[0] <<= 1;
  canFrame[0] |= btn3;
  canFrame[0] <<= 1;
  canFrame[0] |= r2d;

  // Byte 2 is logged error state.
  canFrame[1] = criticalError;

  CAN0.sendMsgBuf(CAN_ID, 0, 2, canFrame);
  // Serial.println("Frame sent.");
}

void canInit(){

    // Configuring our canbus spi connection correctly
    SPI.setRX(CAN_MISO);
    SPI.setSCK(CAN_SCK);
    SPI.setTX(CAN_MOSI);
  

    pinMode(CAN_INT, INPUT);
    pinMode(CAN_CS, OUTPUT);

    
    if(CAN0.begin(MCP_STDEXT, CAN_500KBPS, MCP_16MHZ) == CAN_OK){
        Serial.println("MCP2515 Initialized at 500kbps (16MHz)");
    }else{
        Serial.println("MCP2515 Error...");
    }
    // All filters and masks has to be set for it to work properly.
    CAN0.init_Mask(0, 0x00F00000);
    CAN0.init_Filt(0, 0x00A00000); //+ Owned by mask 0
    CAN0.init_Filt(1, 0x00E00000); //+

    CAN0.init_Mask(1, 0x06FF0000);
    CAN0.init_Filt(2, 0x06B00000); //- Owned by mask 1
    CAN0.init_Filt(3, 0x06B10000); //-
    CAN0.init_Filt(4, 0x00C00000); //-
    CAN0.init_Filt(5, 0x06B00000); //-


    CAN0.setMode(MCP_NORMAL);   // Change to normal mode to allow messages to be transmitted
}