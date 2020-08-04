const { app } = require("electron");
const assert = require("assert");

// NOTE: Tests are run without a BrowserWindow, in headless mode;
// This is a minimum test harness for checking modules in an electron run-time;

app.whenReady().then(() => {
    // Load native modules;
    let addon = require('./dist');
    
    let obj = new addon.MyObject(10,2);
    
    assert.equal(obj.value,10,"verify value works");
    assert.equal(obj.plusOne(),11);
    assert.equal(obj.value2,2);

    obj.changeValue(100);
    assert.equal(obj.value,100);

    obj.updateValue(50);
    assert.equal(obj.value,50);

    obj.value3 = 10;
    assert.equal(obj.value,10,"test setter");

    obj.value4 = 60;
    assert.equal(obj.value,60,"test test setter with custom property");

    obj.value4 = -10;
    assert.equal(obj.isPositive,false);

    obj.value4 = 10;
    assert.equal(obj.isPositive,true);

    obj.clear = false;
    assert.equal(obj.value,10);

    obj.clear = true;
    assert.equal(obj.value,0);

    console.log("class simple test succeed");
})