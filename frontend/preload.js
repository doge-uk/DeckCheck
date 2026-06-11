const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('boothunready', {
  selectFolder: () => ipcRenderer.invoke('select-folder'),
  getDevices: () => ipcRenderer.invoke('get-devices'),
  runScan: (folderPath, deviceName) => ipcRenderer.invoke('run-scan', folderPath, deviceName),
  // Add window control methods
  minimizeWindow: () => ipcRenderer.send('minimize-window'),
  closeWindow: () => ipcRenderer.send('close-window'),
  maximizeWindow: () => ipcRenderer.send('maximize-window')
});