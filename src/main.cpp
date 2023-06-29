/*

Author: Håvard Nygård.
Written for Align Racing, for use in the dashboard of AR23.

*/

// Duh
#include <Arduino.h>
#include <FreeRTOS.h>
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


void setup(void) {
  Serial.begin(9600);

  initUI();
  screensaver();
  
  // Initializing CANBUS communication
  // Initialize MCP2515 running at 16MHz with a baudrate of 500kb/s and the masks and filters disabled.

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

  

}

// loop1() and loop() is alternating between who is in control with 1ms intervals, which means they can both access
// the same regions of memory

void setup1(){
  canInit();
}

void loop1(){
  canRx();
  if(millis() - broadcastTimestamp > 100){
    canTx();
    broadcastTimestamp = millis();
  }
}