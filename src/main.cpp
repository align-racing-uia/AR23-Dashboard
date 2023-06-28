/*

Author: Håvard Nygård.
Written for Align Racing, for use in the dashboard of AR23.

*/

// Duh
#include <Arduino.h>

// Important to get the SPI pins working correctly for canbus
#include <pins_arduino_override.h>
#include <SPI.h>

// The handeling of the CANBUS
#include <can.h>

// Include everything related to the inverter
#include <inverter.h>

// Include everything related to the BMS
#include <bms.h>

// Adding some helper functions for multicore support
#include <core1.h>

// Include everything related to the GUI
#include <gui.h>

// Timestamps and states
#include <states.h>


// Configuring buttons
#define PIN_BTN1 27
#define PIN_BTN2 26
#define PIN_BTN3 22
#define PIN_BTN4 28
#define PIN_R2D 14

// Should be simplified, but will probably not run much during a single session
void mapRunFault(){
  if(currentRunFault != ""){
    return;
  }
  for(int i=0; i<4; i++){
    for(int y=0; y<8; y++){
      if(inverterRunFaultCode[i] >> y == 1){
        currentRunFault = runFaults[i*8 + y];
        Serial.println(currentRunFault);
        updateMiddleStatus = true;
        return;
      }
    }
  }
}

void mapPostFault(){
  if(currentPostFault != ""){
    return;
  }
  for(int i=0; i<4; i++){
    for(int y=0; y<8; y++){
      if(inverterPostFaultCode[i] >> y == 1){
        currentPostFault = postFaults[i*8 + y];
        Serial.println(currentPostFault);
        updateMiddleStatus = true;
        return;
      }
    }
  }
}

void resetFaultCodes(){
  criticalError = 0;
  currentPostFault = "";
  currentRunFault = "";
  updateTopStatus = true;
  updateBottomStatus = true;
  updateMiddleStatus = true;

}

void checkForFaults(){

  // If any bit is set, there is a error.
  if(inverterRunFaultCode[0] | inverterRunFaultCode[1] | inverterRunFaultCode[2] | inverterRunFaultCode[3] > 0){
    mapRunFault();
    if(criticalError != 6){
      updateTopStatus = true;
    }
    criticalError = 6;
  }else if(currentRunFault != ""){
    resetFaultCodes();
  }

  if(inverterPostFaultCode[0] | inverterPostFaultCode[1] | inverterPostFaultCode[2] | inverterPostFaultCode[3] > 0){
    mapPostFault();
    if(criticalError != 6){
      updateTopStatus = true;
    }

    criticalError = 6;
  }else if(currentPostFault != ""){
    resetFaultCodes();
  }

  // Will only display the first proper error.
  // The last error checked will probably take priority

  if(criticalError < 1){
   // If inverter stops communicating on the CANBUS network, display this error:
    if(millis() - inverterTimestamp > 1000){
      updateTopStatus = criticalError != 4;
      criticalError = 4;
    }

    // If APPS stops communicating on the CANBUS network, display this error:
    if(millis() - appsTimestamp > 2000){
      updateTopStatus = criticalError != 5;
      criticalError = 5;
    }
  }
}





void setup(void) {
  Serial.begin(9600);
  SPI.begin();
  delay(1000);
  // Configuring screen
  tft.init();
  tft.setRotation(1);
  tft.setTextDatum(BL_DATUM);

  screensaver();
  
  // Initializing CANBUS communication
  // Initialize MCP2515 running at 16MHz with a baudrate of 500kb/s and the masks and filters disabled.
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


  pinMode(PIN_BTN1, INPUT);
  pinMode(PIN_BTN2, INPUT);
  pinMode(PIN_BTN3, INPUT);
  pinMode(PIN_BTN4, INPUT);
  pinMode(PIN_R2D, INPUT);

  clearScreen();
  inverterTimestamp = millis();
  appsTimestamp = millis();

}

void loop() {
  // Gathering information
  canRx();
  btn1 = digitalRead(PIN_BTN1);
  btn2 = digitalRead(PIN_BTN2);
  btn3 = digitalRead(PIN_BTN3);
  btn4 = digitalRead(PIN_BTN4);
  r2d = digitalRead(PIN_R2D);

  checkForFaults();
  drawUI();
  //Resetting fault codes can be practical during testing
  if(btn3){
    resetFaultCodes();
  }

  if(millis() - broadcastTimestamp > 100){
    canTx();
    broadcastTimestamp = millis();
  }

  // TODO: Display state of charge
  


  delay(1);

}
