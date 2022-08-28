#pragma once

#include <3ds/types.h>

typedef struct
{
    bool    noFlash;
    u32     lowTitleId;
    char    path[256];
    u32     config[32];
}   PluginLoadParameters;

Result  plgLdrInit(void);
void    plgLdrExit(void);
Result  PLGLDR__IsPluginLoaderEnabled(bool *isEnabled);
Result  PLGLDR__SetPluginLoaderState(bool enabled);
Result  PLGLDR__SetPluginLoadParameters(PluginLoadParameters *parameters);