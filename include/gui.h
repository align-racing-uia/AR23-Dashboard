#ifndef GUI_H
#define GUI_H


#include <TFT_eSPI.h> // This is configured for this exact build.
#include <inverter.h>
#include <states.h>

#include <logo.h>

extern TFT_eSPI tft;
#define SCREEN_WIDTH 480
#define SCREEN_HEIGHT 320


// Error list
extern String criticalErrors[];

void screensaver();

void clearScreen();

void drawInverterStatus();

void drawR2DStatus();

void drawSDCStatus();

void drawVsmStatus();

void drawMiddleStatus();

void drawUI();

void initUI();


#endif