#include <3ds.h>
#include "plgldr.h"

static Handle   plgLdrHandle;
static int      plgLdrRefCount;

Result  plgLdrInit(void)
{
    Result res = 0;

    if (AtomicPostIncrement(&plgLdrRefCount) == 0)
        res = svcConnectToPort(&plgLdrHandle, "plg:ldr");
    return res;
}

void    plgLdrExit(void)
{
    if (AtomicDecrement(&plgLdrRefCount))
        return;
    svcCloseHandle(plgLdrHandle);
}

Result  PLGLDR__IsPluginLoaderEnabled(bool *isEnabled)
{
    Result res = 0;

    u32 *cmdbuf = getThreadCommandBuffer();

    cmdbuf[0] = IPC_MakeHeader(2, 0, 0);
    if (R_SUCCEEDED((res = svcSendSyncRequest(plgLdrHandle))))
    {
        res = cmdbuf[1];
        *isEnabled = cmdbuf[2];
    }
    return res;
}

Result  PLGLDR__SetPluginLoaderState(bool enabled)
{
    Result res = 0;

    u32 *cmdbuf = getThreadCommandBuffer();

    cmdbuf[0] = IPC_MakeHeader(3, 1, 0);
    cmdbuf[1] = (u32)enabled;

    if (R_SUCCEEDED((res = svcSendSyncRequest(plgLdrHandle))))
    {
        res = cmdbuf[1];
    }
    return res;
}

Result  PLGLDR__SetPluginLoadParameters(PluginLoadParameters *parameters)
{
    Result res = 0;

    u32 *cmdbuf = getThreadCommandBuffer();

    cmdbuf[0] = IPC_MakeHeader(4, 2, 4);
    cmdbuf[1] = (u32)parameters->noFlash;
    cmdbuf[2] = parameters->lowTitleId;
    cmdbuf[3] = IPC_Desc_Buffer(256, IPC_BUFFER_R);
    cmdbuf[4] = (u32)parameters->path;
    cmdbuf[5] = IPC_Desc_Buffer(32 * sizeof(u32), IPC_BUFFER_R);
    cmdbuf[6] = (u32)parameters->config;

    if (R_SUCCEEDED((res = svcSendSyncRequest(plgLdrHandle))))
    {
        res = cmdbuf[1];
    }
    return res;
}