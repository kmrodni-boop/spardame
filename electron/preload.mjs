import { contextBridge } from "electron";

contextBridge.exposeInMainWorld("spardame", { desktop: true });
