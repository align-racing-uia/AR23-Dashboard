#ifndef GUI_H
#define GUI_H


#include <TFT_eSPI.h> // This is configured for this exact build.
#include <inverter.h>
#include <states.h>

#include <logo.h>

TFT_eSPI tft = TFT_eSPI();
#define SCREEN_WIDTH 480
#define SCREEN_HEIGHT 320


// Error list
char *criticalErrors[] = {"N/A",
                      "AMS Fault", 
                      "Brake Implausibility",
                      "SDC Open",
                      "Inverter Silent",
                      "APPS Silent",
                      "Fault Codes Blinking"};

void screensaver(){
  // Boot up screen
  tft.fillScreen(TFT_BLACK);
  tft.fillRect(240-logo_width/2, 160-logo_height/2,logo_width, logo_height, TFT_ALIGN);
  tft.drawXBitmap(240-logo_width/2, 160-logo_height/2, logo, logo_width, logo_height, TFT_BLACK);
}

void clearScreen(){
  tft.fillScreen(TFT_BLACK);
}

void drawInverterStatus(){
  tft.setTextSize(3);
  // Displaying errors take priority!
  if(criticalError > 0){
    tft.setTextColor(TFT_RED, TFT_BLACK);
    tft.drawString(criticalErrors[criticalError], 18, SCREEN_HEIGHT/10+15);
    return;
  }

  if(inverterState <= 4){
    tft.setTextColor(TFT_YELLOW, TFT_BLACK);
    // Serial.println("Try change to yellow");
    tft.drawString(inverterStates[inverterState], 18, SCREEN_HEIGHT/10+15);
  }else{
    tft.setTextColor(TFT_GREEN, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString(inverterStates[inverterState], 18, SCREEN_HEIGHT/10+15);
  }
  
}

void drawR2DStatus(){
  tft.setTextSize(3);
  int color = TFT_RED;
  if(r2dState){
    color = TFT_GREEN;
  }
  tft.fillRect(((SCREEN_WIDTH / 6) * 5) + 1, SCREEN_HEIGHT / 5 * 4 + 1, (SCREEN_WIDTH / 6) - 2, (SCREEN_HEIGHT / 5)- 2, color);
  tft.setTextDatum(BC_DATUM);
  tft.setTextColor(TFT_BLACK, color);
  tft.drawString("R2D", ((SCREEN_WIDTH / 6) * 5) + (SCREEN_WIDTH / 12), SCREEN_HEIGHT/5*4 + SCREEN_HEIGHT/10+15);
  tft.setTextDatum(BL_DATUM);

}

void drawSDCStatus(){
  tft.setTextSize(3);
  int color = TFT_RED;
  if(sdcState){
    color = TFT_GREEN;
  }
  tft.fillRect(((SCREEN_WIDTH / 6) * 5) + 1,1+(SCREEN_HEIGHT/5*3), (SCREEN_WIDTH / 6) - 2, (SCREEN_HEIGHT / 5) - 2, color);
  tft.setTextDatum(BC_DATUM);
  tft.setTextColor(TFT_BLACK, color);
  tft.drawString("SDC", ((SCREEN_WIDTH / 6) * 5) + (SCREEN_WIDTH / 12), 1+(SCREEN_HEIGHT/5*3)+SCREEN_HEIGHT/10+15);
  tft.setTextDatum(BL_DATUM);

}

void drawVsmStatus(){
  tft.setTextSize(2);
  if(vsmState < 5){
    tft.setTextColor(TFT_YELLOW, TFT_BLACK);
    // Serial.println("Try change to yellow");
    tft.drawString(vsmStates[vsmState], 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }else if(vsmState == 5){
    tft.setTextColor(TFT_GREEN, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString(vsmStates[vsmState], 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }else{
    tft.setTextColor(TFT_RED, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString("Fault Codes Blinking", 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }
}

void drawMiddleStatus(){
  tft.setTextSize(3);
  tft.setTextDatum(BL_DATUM);
  tft.setTextColor(TFT_GREEN, TFT_BLACK);
  tft.fillRect(0, SCREEN_HEIGHT/5*1, SCREEN_WIDTH/2, SCREEN_HEIGHT/10*3, TFT_BLACK);
  tft.drawString("SOC: "+String(soc)+"%",18, SCREEN_HEIGHT/5*1+45);
  tft.drawString("TS DC: "+String(dcVoltage)+"V", 18, SCREEN_HEIGHT/5*2+20);
  tft.setTextColor(TFT_RED, TFT_BLACK);
  tft.setTextDatum(TL_DATUM);
  tft.setTextSize(2);
  tft.fillRect(0, SCREEN_HEIGHT/2, SCREEN_WIDTH/6*5, SCREEN_HEIGHT/10*3, TFT_BLACK);
  
  if(currentRunFault != String("")){
    tft.drawString("Run Faults:",18, SCREEN_HEIGHT/2+5);
    tft.drawString(currentRunFault,18, SCREEN_HEIGHT/2+25);
  }
  if(currentPostFault != String("")){
    tft.drawString("Post Faults:",18, SCREEN_HEIGHT/2+45);
    tft.drawString(currentPostFault,18, SCREEN_HEIGHT/2+65);
  }

  tft.setTextDatum(BL_DATUM);
}


void drawUI(){
  // No need to update the whole screen for everything, and risking blinky graphics.
  if(updateTopStatus){
    tft.fillRect(5,5, SCREEN_WIDTH-10, SCREEN_HEIGHT/5-10, TFT_BLACK);
    tft.drawRect(0,0, SCREEN_WIDTH, SCREEN_HEIGHT/5, TFT_ALIGN);
    drawInverterStatus();
    updateTopStatus = false;
  }

  if(updateMiddleStatus){
    drawMiddleStatus();
    updateMiddleStatus = false;
  }

  if(updateBottomStatus){
    tft.fillRect(0, SCREEN_HEIGHT/5 * 4, SCREEN_WIDTH, SCREEN_HEIGHT/5, TFT_BLACK);
    drawVsmStatus();
    updateBottomStatus = false;
    drawR2DStatus();
    drawSDCStatus();
  }
  
}

#endif