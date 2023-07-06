/*

Author: Håvard Nygård.
Written for Align Racing, for use in the dashboard of AR23.

*/

// Duh
#include <Arduino.h>
#include <pins_arduino_override.h>
// Timestamps and states
#include <states.h>

// Important to get the SPI pins working correctly for canbus

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

// Include fault handeling
#include <faults.h>

// Configuring buttons
#define PIN_BTN1 27
#define PIN_BTN2 26
#define PIN_BTN3 22
#define PIN_BTN4 28
#define PIN_R2D 14

bool btn1_toggled = false;

void setup(void) {
  Serial.begin(9600);

  initUI();
  
  // Initializing CANBUS communication
  // Initialize MCP2515 running at 16MHz with a baudrate of 500kb/s and the masks and filters disabled.

  pinMode(PIN_BTN1, INPUT_PULLDOWN);
  pinMode(PIN_BTN2, INPUT_PULLDOWN);
  pinMode(PIN_BTN3, INPUT_PULLDOWN);
  pinMode(PIN_BTN4, INPUT_PULLDOWN);
  pinMode(PIN_R2D, INPUT_PULLDOWN);

  clearScreen(TFT_BLACK);
  inverterTimestamp = millis();
  appsTimestamp = millis();

}

void loop() {
  // Gathering information
  btn1 = digitalRead(PIN_BTN1);
  btn2 = digitalRead(PIN_BTN2);
  btn3 = digitalRead(PIN_BTN3);
  btn4 = digitalRead(PIN_BTN4);
  r2d = digitalRead(PIN_R2D);

  checkForFaults();
  drawUI();
  //Resetting fault codes can be practical during testing
  if(btn2){
    resetFaultCodes();
  }

  blinkingTimestamp = millis();

  if(blinkingTimestamp % 1000 > 500){
    blink = true;
  }else{
    blink = false;
  }

  if(btn1 && !btn1_toggled) {
    btn1_toggled = true;
    currentScreen += 1;
    if(currentScreen > 2) {
      currentScreen = 1;
    }
    clearScreen(TFT_BLACK);
    updateBrakeStatus = true;
    updateScreensaver = true;
    updateBottomStatus = true;
    updateMiddleStatus = true;
    updateTopStatus = true;
  }

  if(!btn1) {
    btn1_toggled = false;
  }



  

}

// loop1() and loop() is alternating between who is in control with 1ms intervals, which means they can both access
// the same regions of memory

void setup1(){
  canInit();
}

void loop1(){
  canRx();
  canUpdate(); // Update shared states
  if(millis() - broadcastTimestamp > 100){
    canTx();
    broadcastTimestamp = millis();
  }
}