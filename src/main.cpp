#include "align.h"
#include <Arduino.h>
#include <mcp_can.h>

#include <SPI.h>

#include <TFT_eSPI.h> // Hardware-specific library

TFT_eSPI tft = TFT_eSPI();       // Invoke custom library
#define TFT_ALIGN 0xcb42 // The most important color
#define SCREEN_WIDTH 480
#define SCREEN_HEIGHT 320

// All other screen configuration can be found in "./pio/libdeps/pico/TFT_eSPI/User_Setup.h"

// CANBUS Configuration

#define CAN_SCK 2
#define CAN_MOSI 3
#define CAN_MISO 4
#define CAN_INT 6
#define CAN_CS 5
MCP_CAN CAN0(CAN_CS);

#define CAN_ID 0x0E0

long unsigned int rxId;
unsigned char rxLen = 0;
unsigned char rxBuf[8];

// Configuring buttons
#define PIN_BTN1 27
#define PIN_BTN2 26
#define PIN_BTN3 22
#define PIN_BTN4 28
#define PIN_R2D 14



// Variables
// Inverter based states
uint16_t vsmState = 0;
uint16_t inverterState = 0;
char *inverterStates[] = {"Powering on", 
                      "Inverter Stopped", 
                      "Inverter Open Loop", 
                      "Inverter Closed Loop",
                      "Inverter Waiting",
                      "N/A",
                      "N/A",
                      "N/A",
                      "Inverter Idle Run",
                      "Inverter Idle Stopped",
                      "N/A",
                      "N/A",
                      "N/A"};
          

// Button states
bool btn1 = false;
bool btn2 = false;
bool btn3 = false;
bool btn4 = false;
bool r2d = false;

// Apps based states
bool r2dState = false;

// Internal states
int errorStatus = 0;
bool updateStatus = true;

// Timestamps
long unsigned int broadcastTimestamp = 0;

void screensaver(){
  // Boot up screen
  tft.fillScreen(TFT_BLACK);
  tft.fillRect(240-logo_width/2, 160-logo_height/2,logo_width, logo_height, TFT_ALIGN);
  tft.drawXBitmap(240-logo_width/2, 160-logo_height/2, logo, logo_width, logo_height, TFT_BLACK);
}

void clearScreen(){
  tft.fillScreen(TFT_BLACK);
}

void drawHeader(){
  tft.fillRect(5,5, SCREEN_WIDTH-10, SCREEN_HEIGHT/5-10, TFT_BLACK);
  tft.drawRect(0,0, SCREEN_WIDTH, SCREEN_HEIGHT/5, TFT_ALIGN);
}


void drawStatus(){
  tft.setTextSize(3);

  if(!errorStatus){
    tft.setTextColor(TFT_GREEN, TFT_BLACK);
    tft.drawString("Status OK.", 18, SCREEN_HEIGHT/10+15);
  }else{
    tft.setTextColor(TFT_RED, TFT_BLACK);
    tft.drawString("Error: AMS Fault.", 18, SCREEN_HEIGHT/10+15); // Todo implement fault codes.
  }
  
}

void drawR2DStatus(){
  int color = TFT_RED;
  if(r2dState){
    color = TFT_GREEN;
  }
  tft.fillRect(((SCREEN_WIDTH / 6) * 5) + 1,1, (SCREEN_WIDTH / 6) - 2, (SCREEN_HEIGHT / 5) - 2, color);
  tft.setTextDatum(BC_DATUM);
  tft.setTextColor(TFT_BLACK, color);
  tft.drawString("R2D", ((SCREEN_WIDTH / 6) * 5) + (SCREEN_WIDTH / 12), SCREEN_HEIGHT/10+15);
  tft.setTextDatum(BL_DATUM);

}

void canRx(){
  if(!digitalRead(CAN_INT)){
    CAN0.readMsgBuf(&rxId, &rxLen, rxBuf);
  }else{
    return;
  }
  bool newState = 0;
  switch(rxId){
    case 0x0AA:
      vsmState = rxBuf[0];
      updateStatus = vsmState != errorStatus;
      errorStatus = vsmState;
      break;
    case 0x0E1:
      newState = 0x2 & rxBuf[3];
      updateStatus = newState != r2dState;
      r2dState = newState;
      break;
    default:
      return;
  }

  
}

void canTx(){
  byte canFrame[1] = {0x0};
  canFrame[0] = btn1;
  canFrame[0] <<= 1;
  canFrame[0] |= btn2;
  canFrame[0] <<= 1;
  canFrame[0] |= btn3;
  canFrame[0] <<= 1;
  canFrame[0] |= btn4;
  canFrame[0] <<= 1;
  canFrame[0] |= r2d;

  // Serial.println("Frame sent.");
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
  int can_state = CAN0.begin(MCP_ANY, CAN_500KBPS, MCP_16MHZ);
  if(can_state != CAN_OK){
    Serial.print("Can error... ");
    Serial.println(can_state);
  }else{
    Serial.println("CAN OK.");
  }
  CAN0.setMode(MCP_NORMAL);   // Change to normal mode to allow messages to be transmitted
  pinMode(PIN_BTN1, INPUT);
  pinMode(PIN_BTN2, INPUT);
  pinMode(PIN_BTN3, INPUT);
  pinMode(PIN_BTN4, INPUT);
  pinMode(PIN_R2D, INPUT);

  clearScreen();

}

void loop() {
  // Gathering information
  canRx();
  btn1 = digitalRead(PIN_BTN1);
  btn2 = digitalRead(PIN_BTN2);
  btn3 = digitalRead(PIN_BTN3);
  btn4 = digitalRead(PIN_BTN4);
  r2d = digitalRead(PIN_R2D);

  // Display status banner.
  if(updateStatus){
    drawHeader();
    drawStatus();
    drawR2DStatus();
    updateStatus = false;
  }
  if(millis() - broadcastTimestamp > 100){
    canTx();
    broadcastTimestamp = millis();
  }

  // TODO: Display state of charge
  


  delay(5);

}


