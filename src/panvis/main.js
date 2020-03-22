const electron = require('electron');
const url = require('url');
const path = require('path');





const {app, BrowserWindow, Menu, ipcMain} = electron;

let mainWindow;

//Listen for app to be ready
app.on('ready', function(){
    //Create Window
    mainWindow = new BrowserWindow({webPreferences: {
        nodeIntegration: true
    }});
   
    //Load HTML Window
    mainWindow.loadURL(url.format({
        pathname: path.join(__dirname, 'mainWindow.html'),
        protocol:'file',
        slashes: true
    }));
    mainWindow.on('closed', function(){
        app.quit();
      });

      // Build menu from template
    const mainMenu = Menu.buildFromTemplate(mainMenuTemplate);
    // Insert menu
    Menu.setApplicationMenu(mainMenu)

  
   });

//Catch item:start
ipcMain.on('submit:start', function(e, population){
    console.log(population);
    mainWindow.webContents.send('submit:start', population);
  });



   // Create menu template
const mainMenuTemplate =  [
    // Each object is a dropdown
    {
      label: 'File',
      submenu:[
        {
          label:'Reset',
          click(){
        
          }
        },
        {
          label: 'Quit',
          accelerator:process.platform == 'darwin' ? 'Command+Q' : 'Ctrl+Q',
          click(){
            app.quit();
          }
        }
      ]
    }
  ];

// Add developer tools option if in dev
if(process.env.NODE_ENV !== 'production'){
    mainMenuTemplate.push({
      label: 'Developer Tools',
      submenu:[
        {
          role: 'reload'
        },
        {
          label: 'Toggle DevTools',
          accelerator:process.platform == 'darwin' ? 'Command+I' : 'Ctrl+I',
          click(item, focusedWindow){
            focusedWindow.toggleDevTools();
          }
        }
      ]
    });
  }

